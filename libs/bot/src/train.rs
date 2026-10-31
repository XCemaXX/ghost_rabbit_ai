// Training data for `vision` and game scores for `policy`.
// Labels come from rollouts on a copy of the game; the copy is never used at play time.
use crate::rng::{MidRng, Rng};
use crate::{MOVES, climb_move, features};
use game_logic::{BOT_Y, Difficulty, GameState, MoveDirection};
use neural_network::{Network, train};
use rand_trait::GenRandFloat;

pub const DT: f32 = 1.0 / 60.0;
const GAME_FRAMES: usize = 3600;
// Gain and fall risk: hold the move, then climb. Monster risk: hold the move all the time.
const HOLD_FRAMES: usize = 10;
const FALL_FRAMES: usize = 90;
const MONSTER_FRAMES: usize = 40;
// Risk label is RISK_DECAY^(frames to death): 0.5 after 20 frames.
const RISK_DECAY: f32 = 0.965_93;
// Typical gain of one rollout is near 0.75 screen units; scale it to about [0, 1].
const GAIN_SCALE: f32 = 0.75;
// Data collection: share of random move series, so states near mistakes are in the data.
const RANDOM_MOVES: f32 = 0.1;
const RISK_WEIGHT: f32 = 4.0;
// GA fitness: points lost per unit of energy.
const ENERGY_COST: f32 = 0.2;
// Neighbour frames are almost the same: every second frame gives about the same data at half the cost.
const SAMPLE_EVERY: usize = 2;

pub struct Sample {
    pub inputs: Vec<f32>,
    pub labels: [f32; 9],
}

// Gain, monster risk, fall risk for Left / None / Right.
pub fn labels<R: GenRandFloat>(g: &GameState<R>) -> [f32; 9] {
    let long = MOVES.map(|m| rollout(g, m, HOLD_FRAMES, FALL_FRAMES));
    let short = MOVES.map(|m| rollout(g, m, MONSTER_FRAMES, MONSTER_FRAMES));
    let risk = |death: Option<(usize, bool)>, fall: bool| match death {
        Some((frame, by_fall)) if by_fall == fall => RISK_DECAY.powi(frame as i32),
        _ => 0.0,
    };
    let mut l = [0.0; 9];
    for i in 0..3 {
        l[i] = long[i].0;
        l[3 + i] = risk(short[i].1, false);
        l[6 + i] = risk(long[i].1, true);
    }
    l
}

// Height gain, and the frame of death with "was it a fall".
fn rollout<R: GenRandFloat>(
    g: &GameState<R>,
    m: MoveDirection,
    hold: usize,
    frames: usize,
) -> (f32, Option<(usize, bool)>) {
    let mut s = g.fork_plan(MidRng);
    let start = s.score_f();
    for i in 0..frames {
        let mv = if i < hold { m } else { climb_move(&s) };
        s.move_player_by_x(DT, mv);
        if s.next_step(DT).game_over {
            let fall = s.player.position.y < BOT_Y;
            return ((s.score_f() - start) / GAIN_SCALE, Some((i, fall)));
        }
    }
    ((s.score_f() - start) / GAIN_SCALE, None)
}

// One game for `vision` training data (DAgger): without `vision` the labels choose the moves,
// with it the net chooses. Every SAMPLE_EVERY-th visited frame is labelled.
pub fn play_and_label(vision: Option<&Network>, seed: u64) -> Vec<Sample> {
    let mut g = GameState::new(Rng(seed), Difficulty::Unreal);
    let mut rng = Rng(seed ^ 0x5151);
    let mut hold = (0, MoveDirection::None);
    let mut samples = Vec::new();
    for frame in 0..GAME_FRAMES {
        let sample = frame.is_multiple_of(SAMPLE_EVERY);
        let labels = if sample || vision.is_none() {
            labels(&g)
        } else {
            [0.0; 9]
        };
        let mut inputs = Vec::new();
        features(&g, &mut inputs);
        let m = if hold.0 > 0 {
            hold.0 -= 1;
            hold.1
        } else if rng.unit() < RANDOM_MOVES {
            hold = (rng.below(15), MOVES[rng.below(3)]);
            hold.1
        } else {
            let out = vision.map_or(labels, |v| {
                let o = v.propagate(inputs.clone());
                let mut p = [0.0; 9];
                for i in 0..3 {
                    p[i] = o[i];
                    p[3 + i] = train::sigmoid(o[3 + i]);
                    p[6 + i] = train::sigmoid(o[6 + i]);
                }
                p
            });
            best_move(&out)
        };
        if sample {
            samples.push(Sample { inputs, labels });
        }
        g.move_player_by_x(DT, m);
        if g.next_step(DT).game_over {
            break;
        }
    }
    samples
}

fn best_move(p: &[f32; 9]) -> MoveDirection {
    let value = |i: usize| p[i] - RISK_WEIGHT * p[3 + i] - RISK_WEIGHT * p[6 + i];
    MOVES[(0..3)
        .max_by(|&a, &b| value(a).partial_cmp(&value(b)).unwrap())
        .unwrap()]
}

// dLoss/dOutput of `vision`: squared error for gains, cross-entropy for risks.
pub fn loss_grad(output: &[f32], labels: &[f32; 9]) -> Vec<f32> {
    let mut grad = train::mse(&output[..3], &labels[..3]).1;
    grad.extend(train::bce_with_logits(&output[3..], &labels[3..]).1);
    grad
}

// GA fitness of one game: the score minus the energy spent.
// Energy per frame: a move costs 2, standing still 1, a reversal 0.5 more.
// For the score alone, shaking left and right is as good as standing still.
pub fn fitness(mut act: impl FnMut(&GameState<Rng>) -> MoveDirection, seed: u64) -> f32 {
    let mut last = MoveDirection::None;
    let mut energy = 0.0;
    let score = score(
        |g| {
            let m = act(g);
            energy += if m == MoveDirection::None { 1.0 } else { 2.0 };
            if matches!(
                (last, m),
                (MoveDirection::Left, MoveDirection::Right)
                    | (MoveDirection::Right, MoveDirection::Left)
            ) {
                energy += 0.5;
            }
            last = m;
            m
        },
        seed,
    );
    // Roulette selection needs fitness >= 0.
    (score - ENERGY_COST * energy).max(0.0)
}

// Score of one 60 s game on Unreal.
pub fn score(mut act: impl FnMut(&GameState<Rng>) -> MoveDirection, seed: u64) -> f32 {
    let mut g = GameState::new(Rng(seed), Difficulty::Unreal);
    for _ in 0..(60.0 / DT) as usize {
        let m = act(&g);
        g.move_player_by_x(DT, m);
        if g.next_step(DT).game_over {
            break;
        }
    }
    g.score_f() * 50.0
}
