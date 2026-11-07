use crate::features::wrap_dx;
use game_logic::{ACCELERATION, BOT_Y, GameState, MoveDirection, X_SPEED};
use rand_trait::GenRandFloat;

// Move to the highest floor that is still reachable on this jump. Only for training labels.
pub(crate) fn climb_move<R: GenRandFloat>(g: &GameState<R>) -> MoveDirection {
    let p = &g.player;
    let r = p.radius;
    let mut best: Option<(f32, f32)> = None;
    for f in &g.floors {
        let top = f.position.y + f.size.y / 2.0;
        if top + r < BOT_Y + 0.02 {
            continue;
        }
        let Some(t) = fall_time(p.speed.y, top + r * 0.5 - p.position.y) else {
            continue;
        };
        let dx = wrap_dx(f.position.x - p.position.x);
        let need = (dx.abs() - (f.size.x / 2.0 + r * 0.3)).max(0.0);
        if need > X_SPEED * t {
            continue;
        }
        if best.is_none_or(|(best_top, _)| top > best_top) {
            best = Some((top, dx));
        }
    }
    match best {
        Some((_, dx)) if dx > 0.01 => MoveDirection::Right,
        Some((_, dx)) if dx < -0.01 => MoveDirection::Left,
        _ => MoveDirection::None,
    }
}

// Time until the player center falls by `dy` from now, on the way down.
fn fall_time(vy: f32, dy: f32) -> Option<f32> {
    let d = vy * vy + 2.0 * ACCELERATION * dy;
    if d < 0.0 {
        return None;
    }
    let t = (-vy - d.sqrt()) / ACCELERATION;
    (t > 0.0).then_some(t)
}
