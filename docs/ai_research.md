# AI bot: research summary

This file explains why the bot in `libs/bot` is built the way it is.
The full research log (in Russian) and the experiment harness `ai_lab` are in the local branch `ai-lab` (`ai_lab/REPORT.md`).

## Goal

- The bot plays the Unreal difficulty.
- In the game, the bot sees only the screen. It gets no hints, no copy of the game state, and no hand-written rules.
- Training can use anything, including a copy of the game.
- `tools/bot_train` makes the pretrained nets natively. A player can also train own nets in the browser, on the AI screen.

## Metric

- Mean score on 200–500 fixed held-out seeds, Unreal, 60 s limit.
- The climb speed is about 37 points per second, so the ceiling for 60 s is about 2300.
- The AI screen shows the median life and the median score of 200 games, each limited to 3 minutes. Very long games do not distort them. This score is not comparable with the 60 s mean above.

## The bot

| Part | Input → output | How it learns |
|---|---|---|
| `vision` | 17 screen numbers → for left / stay / right: height gain, risk to die by the monster, risk to die by a fall | Supervised. Labels come from rollouts on a copy of the game. |
| `policy` | for each move: its 3 `vision` numbers, "it is the previous move", "it is None" → a score; the best score wins | `genetic_algorithm`, fitness = game score − energy |

In the game: screen → `vision` → `policy` → move, every 2 frames.

| Setting | Value |
|---|---|
| Screen input | player vertical speed and x; 2 floors below and 3 above (dx with screen wrap, dy); monster when on the screen (flag, dx, dy, vx, vy) |
| `vision` | `[17, 64, 64, 64, 9]`, tanh hidden layers, linear output |
| `vision` labels | gain and fall risk: hold the move for 10 frames, then a simple climb rule, 90 frames in all; monster risk: hold the move for 40 frames. Risk label = γ^(frames to death), γ = 0.5^(1/20) |
| `vision` training | DAgger: the net plays, the game copy labels every second frame. 8 rounds × 1500 games. Each round trains on the frames of this and the previous round, 3 epochs (8 on round 0), Adam, batch 512. The loss of the monster risk has weight 3 |
| `policy` | `[5, 1]`, linear; one net with the same weights for all 3 moves |
| `policy` training | roulette, BLX-α, uniform mutation 0.1 / 0.3, elite 0.3; population 50, 4 games per individual, 50 generations, games up to 180 s |
| `policy` fitness | score − 0.2 × energy; energy per frame: move 2, stand still 1, reversal +0.5 |

## Results

60 s, Unreal, mean score.

| Bot | Sees | Score |
|---|---|---|
| Stands still | — | 18 |
| Original GA from `learn_ai.rs` | screen, poor inputs | 13–37 |
| Climb rule only (no dodging) | screen | 309 |
| Any net trained by GA on score only | screen | 175–325 |
| `vision` + old `policy` `[12, 8, 3]` | screen | ~1530 (best checkpoint ~1730) |
| `vision` + fixed formula `gain − K·risk` | screen | 1867 |
| Rule + rollout on a game copy (upper bound) | full game copy | 2218 |

The final bot is measured by median life in a 180 s game: see "The jump in quality".

## Why the original attempt failed

One factor at a time, on the original libraries, 200 generations:

| Change | Score |
|---|---|
| original setup | 37 |
| + inputs relative to the player | **231** |
| + each individual plays 8 shared games, not 1 random game | 101 |
| + argmax output, other mutation, larger population | no effect |

- The main cause was the inputs. The original code took the two **lowest** floors. They are below the player and useless.
- The second cause was noise: one random game per individual.
- The libraries were not the cause. With good inputs, the original GA settings were the best of all tested.

## What works and what does not

Works:

1. Predict facts from the game copy (will I die, how high will I climb). Do not copy the teacher's moves.
2. One output group per cause of death, each with its own horizon: monster — short, fall — long.
3. DAgger: every round adds frames from the states that the net itself reaches.
4. Soft risk labels γ^(frames to death): +78 at 4 rounds, +178 after 2 rounds.
5. A deeper `vision` net: `[64, 64, 64]` gives 1699, `[64, 64]` 1531, `[32, 32, 32]` 1331 (4 rounds).
6. A small GA policy on top of `vision`. Supervised `vision` + GA `policy` is the only way we found for "the net decides".
7. GA speed: 4 games per individual and a decision every 2 frames — 7× faster than 16 games, same quality.

Smooth moves (the bot shook left and right where it had to stay in place):

- In the physics, standing still (`None`) and shaking left-right give the same result, so a score-only GA never learned `None`: 0% of frames, 6.7 reversals per second.
- Fitness = score − energy fixes it: about 60% `None`, usually under 0.5 reversals per second.
- The policy needs "it is the previous move" as an input: without it the policy cannot keep a move.
- The change is sharp: at energy cost 0.1 and below, the bot shakes again (8–12 reversals per second), though it lives longer.

Does not work:

1. GA on score alone for a net that must learn the danger: ceiling ~300 (climbs, does not dodge).
2. GA instead of backprop on the supervised labels: the bot plays at the level of the climb rule.
3. Copying the teacher's discrete move (behaviour cloning of a planner, an action head): ~305–312.
4. One shared risk label for both causes of death.
5. Narrow nets: width 4 does not learn; width 8 learns only with ReLU and weakly.
6. Removing "safe" frames from the training data: −200. The gain output learns on them.
7. For `policy`: tournament selection, no crossover, normal mutation, elite 0.1 or 0.5 — all worse than the defaults.

## Cost

`bot_train` (8 rounds of `vision`, then 8 GAs of 50 generations) takes about 66 s on 12 threads.

Per unit of work, one thread:

| Work | Native | WASM (`node`) |
|---|---|---|
| GA, one game frame (mostly the `vision` forward pass) | 0.57 µs | 0.73 µs |
| `vision` label, one frame (rollouts on the game copy) | 16 µs | 25 µs |
| `vision` fit, one frame and epoch | 2.8 µs | 4.6 µs |

Training in the browser (one WASM thread, 12 ms of each screen frame), Intel Core i5-13400F. Measured by hand in Chrome:

| Stage | Time |
|---|---|
| `policy`, generation 0 → 5 | a few seconds |
| `policy`, generation 0 → 50 | ~45 s |
| `vision`, one round | 10–13 s |
| `vision`, all 8 rounds | ~100 s |

Memory of the frames at training time, WASM: 58 MB, measured. It does not grow after round 0.

## Training in the browser

- The same code as `tools/bot_train`, in small steps: one game of the GA; one labelled frame or one batch for `vision`.
- The game stands while a job runs. A hidden tab gets no frames, so the job waits.
- `policy` stops at generations 0, 1, 2, 5, 10, 20, 30, 50. Life grows fast at the start; later neighbours differ mostly by noise.
- A `policy` belongs to the `vision` it trained on. A pretrained `policy` never runs on the player's `vision`.
- Browser storage keeps own nets: all `vision` rounds, the best `policy` of each point, and the population after the last point. At most ~0.46 MB in base64.
- The frames of `vision` are not stored. After a page reload, the next round first plays the previous round again: the same games and net give the same frames.

## The jump in quality

The bot lived about 40 s of a 180 s game. Replacing parts of the bot with true data from a game copy showed where the quality was lost (round 7, median life):

| What the choice of the move sees | Median, s | Reached 180 s |
|---|---|---|
| All true labels, fixed formula | ≥ 180 | 72% |
| True monster risk only, the rest from `vision`, formula | ≥ 180 | 58% |
| `vision` and formula | 73 | 16% |
| `vision` and the old `policy` `[12, 8, 3]` | ~40 | 4% |

- The labels are good. Most of the loss was in `policy`, then in `vision`.
- The formula is only a reference. A hand-written rule does not go into the game.
- The old `policy` had to find the same rule for each move, in about 130 weights. GA does not find it from a random start, but it improves on the formula when started from it: the fitness was right, the search failed.
- One small net with shared weights leaves about 5 numbers to find. GA finds them in a few generations.
- Monster loss weight 3 lifts `vision` almost for free: r7 74 → 101 s with the formula. Weight 10 is worse: falls grow.
- A wider net costs much more than it gives: `[128, 128, 128]` r7 ~100 s against ~92 s, at 2.4× the training time.
- Games of 180 s in GA instead of 60 s: about +20%. At 60 s, life after one minute gives nothing.
- Population 50 instead of 100 halves the GA time for a small loss: r7 −3.4 ± 6.7 s, with a wider spread. Shorter games lose more: 120 s r7 80 s, 60 s r7 73 s.
- No overfitting: the loss on new frames is about the same as on trained frames, and test results on unused seeds are the same.

Final bot, 5 training runs, mean of the medians at generations 20–50:

| Round | Median life, s | Score |
|---|---|---|
| r1 | 61 | 2102 |
| r3 | 76 | 2646 |
| r7 | 85 (68–105) | 2922 |

`None` about 60% of frames, usually under 0.5 reversals per second.

## Fast `vision` training

Training on all frames of all rounds, 8 epochs each, grows with the square of the rounds: most of the time goes to the last rounds. These settings make it about 3.4× faster, with 3.5× less memory.

Median life, s, mean of g50–g100, 15 training seeds, with the old `policy` `[12, 8, 3]`. The current `policy` is in the table above:

| Settings | Time | r1 | r3 | r7 |
|---|---|---|---|---|
| All frames, 8 epochs, batch 2048 | 100% | 24.4 | 33.5 | 42.2 |
| Last 2 rounds of frames, 3 epochs (8 on round 0), batch 512 | 30% | 21.6 | 28.6 | 39.3 |

- The batch of 2048 was for the threads of `bot_train`. One browser thread does not need it, and more Adam steps per pass allow fewer epochs.
- The window of 2 rounds needs the small batch: with batch 2048 r7 falls to 26 s.
- A smaller net with the window loses much: `[32,32,32]` 23 s, `[48,48,48]` 30 s.
- Fewer games lose on the early rounds: 750 games r1 15 s, r7 32 s.
- Keeping Adam between rounds with the window is worse: r7 31 s.
- One training run is noisy: r7 of a run is 23–51 s.

## Library changes

- `neural_network`: activation per layer (ReLU / tanh / linear); losses and Adam in `train`; `new` scales weights by fan-in; the weighted sum uses 8 partial sums, so the compiler can use SIMD (native SSE, WASM `simd128`): about 3× faster; `tanh` is a rational approximation, applied over the layer outputs as a slice (SIMD): the forward pass is about 4× faster than with `f32::tanh`; `backward_batch` takes many inputs layer by layer.
- `genetic_algorithm`: tournament selection; no crossover; `UniformMutation` (the old behaviour) and a real `GaussianMutation`; roulette checks; public method traits.
- `game_logic`: `fork` / `fork_plan` for rollouts.