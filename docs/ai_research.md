# AI bot: research summary

This file explains why the bot in `libs/bot` is built the way it is.
The full research log (in Russian) and the experiment harness `ai_lab` are in the local branch `ai-lab` (`ai_lab/REPORT.md`).

## Goal

- The bot plays the Unreal difficulty.
- In the game, the bot sees only the screen. It gets no hints, no copy of the game state, and no hand-written rules.
- Training can use anything, including a copy of the game.
- Training runs natively. The browser only plays. (Training in the browser is a later step.)

## Metric

- Mean score on 200–500 fixed held-out seeds, Unreal, 60 s limit.
- The climb speed is about 37 points per second, so the ceiling for 60 s is about 2300.
- The AI screen shows the median life of 200 games, each limited to 3 minutes. Very long games do not distort it.

## The bot

| Part | Input → output | How it learns |
|---|---|---|
| `vision` | 17 screen numbers → for left / stay / right: height gain, risk to die by the monster, risk to die by a fall | Supervised. Labels come from rollouts on a copy of the game. |
| `policy` | the 9 `vision` outputs and its previous move → the move | `genetic_algorithm`, fitness = game score − energy |

In the game: screen → `vision` → `policy` → move, every 2 frames.

| Setting | Value |
|---|---|
| Screen input | player vertical speed and x; 2 floors below and 3 above (dx with screen wrap, dy); monster (alive, dx, dy, vx, vy) |
| `vision` | `[17, 64, 64, 64, 9]`, tanh hidden layers, linear output |
| `vision` labels | gain and fall risk: hold the move for 10 frames, then a simple climb rule, 90 frames in all; monster risk: hold the move for 40 frames. Risk label = γ^(frames to death), γ = 0.5^(1/20) |
| `vision` training | DAgger: the net plays, the game copy labels every second frame. 8 rounds × 1500 games, 8 epochs per round, Adam, batch 2048 |
| `policy` | `[12, 8, 3]`, ReLU; inputs: 9 `vision` outputs + the previous move (one-hot) |
| `policy` training | roulette, BLX-α, uniform mutation 0.1 / 0.3, elite 0.3; population 100, 4 games per individual, 100 generations |
| `policy` fitness | score − 0.2 × energy; energy per frame: move 2, stand still 1, reversal +0.5 |

## Results

60 s, Unreal, mean score.

| Bot | Sees | Score |
|---|---|---|
| Stands still | — | 18 |
| Original GA from `learn_ai.rs` | screen, poor inputs | 13–37 |
| Climb rule only (no dodging) | screen | 309 |
| Any net trained by GA on score only | screen | 175–325 |
| **`vision` + `policy` (final)** | **screen** | **~1530** (best checkpoint ~1730) |
| `vision` + fixed formula `gain − K·risk` | screen | 1867 |
| Rule + rollout on a game copy (upper bound) | full game copy | 2218 |

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
- Fitness = score − energy fixes it: `None` is found in 9 of 9 GA seeds; reversals drop to 0.1–2.3 per second.
- The policy needs its previous move as an input and 8 hidden neurons: without the previous move it cannot keep a move.
- A penalty for every move change, or for reversals only, works worse: `None` is found in about half of the seeds, or the bot dies more often.
- A stronger energy cost makes the bot stand where it must leave: deaths grow.

Does not work:

1. GA on score alone for a net that must learn the danger: ceiling ~300 (climbs, does not dodge).
2. GA instead of backprop on the supervised labels: the bot plays at the level of the climb rule.
3. Copying the teacher's discrete move (behaviour cloning of a planner, an action head): ~305–312.
4. One shared risk label for both causes of death.
5. Narrow nets: width 4 does not learn; width 8 learns only with ReLU and weakly.
6. Removing "safe" frames from the training data: −200. The gain output learns on them.
7. For `policy`: tournament selection, no crossover, normal mutation, elite 0.1 or 0.5 — all worse than the defaults.

## Cost

`bot_train` (8 rounds of `vision`, then 8 GAs of 100 generations) takes about 100 s on 16 threads.

Per unit of work, one thread:

| Work | Native | WASM (`node`) |
|---|---|---|
| GA, one game frame (mostly the `vision` forward pass) | 0.57 µs | 0.73 µs |
| `vision` label, one frame (rollouts on the game copy) | 16 µs | 25 µs |
| `vision` fit, one frame and epoch | 2.6 µs | ~9 µs |

Training in the browser (one WASM thread, estimate from the table):

| Stage | Time |
|---|---|
| `policy`, 5 generations (one button press) | ~5 s |
| `vision`, 2 rounds | ~1 min |
| `vision`, 4 rounds | ~3 min |
| `vision`, 8 rounds | ~10 min |

## Library changes

- `neural_network`: activation per layer (ReLU / tanh / linear); `trace` and `backward`; losses and Adam in `train`; `new` scales weights by fan-in; the weighted sum uses 8 partial sums, so the compiler can use SIMD (native SSE, WASM `simd128`): about 3× faster; `tanh` is a rational approximation, applied over the layer outputs as a slice (SIMD): the forward pass is about 4× faster than with `f32::tanh`; `backward_batch` takes many inputs layer by layer.
- `genetic_algorithm`: tournament selection; no crossover; `UniformMutation` (the old behaviour) and a real `GaussianMutation`; elite fraction; roulette checks; public method traits.
- `game_logic`: `fork` / `fork_plan` for rollouts.