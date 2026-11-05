use game_logic::{BOT_Y, GameState, MAX_SPEED, RIGHT_X, SCREEN_WEIGHT};
use rand_trait::GenRandFloat;

pub(crate) const INPUTS: usize = 2 + 5 * 2 + 5;

const HALF_W: f32 = SCREEN_WEIGHT / 2.0;

// Only what the player sees: on-screen floors and monster, relative to the player.
pub(crate) fn features<R: GenRandFloat>(g: &GameState<R>) -> [f32; INPUTS] {
    let mut out = [0.0; INPUTS];
    let p = &g.player;
    out[0] = p.speed.y / MAX_SPEED;
    out[1] = p.position.x / HALF_W;

    let mut floors: Vec<(f32, f32)> = g
        .floors
        .iter()
        .filter(|f| f.position.y > BOT_Y && f.position.y < 1.0)
        .map(|f| {
            (
                wrap_dx(f.position.x - p.position.x) / HALF_W,
                f.position.y - p.position.y,
            )
        })
        .collect();
    floors.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    // The floors around the player; missing ones are far away.
    let above = floors
        .iter()
        .position(|f| f.1 > 0.0)
        .unwrap_or(floors.len());
    for i in 0..5 {
        let (dx, dy) = match (above + i).checked_sub(2).and_then(|j| floors.get(j)) {
            Some(&floor) => floor,
            None => (0.0, if i < 2 { -2.0 } else { 2.0 }),
        };
        out[2 + 2 * i] = dx;
        out[3 + 2 * i] = dy;
    }

    let m = &g.monster;
    let on_screen =
        m.position.y - m.size.y / 2.0 < 1.0 && m.position.x.abs() - m.size.x / 2.0 < RIGHT_X;
    if !m.is_dead && on_screen {
        out[12..].copy_from_slice(&[
            1.0,
            (m.position.x - p.position.x) / HALF_W,
            m.position.y - p.position.y,
            m.speed.x,
            m.speed.y,
        ]);
    }
    out
}

// Shortest horizontal distance on a screen that wraps at the sides.
pub(crate) fn wrap_dx(dx: f32) -> f32 {
    dx - SCREEN_WEIGHT * (dx / SCREEN_WEIGHT).round()
}
