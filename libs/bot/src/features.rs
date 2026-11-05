use game_logic::{BOT_Y, GameState, MAX_SPEED, SCREEN_WEIGHT};
use rand_trait::GenRandFloat;

pub const INPUTS: usize = 2 + 5 * 2 + 5;

const HALF_W: f32 = SCREEN_WEIGHT / 2.0;

// Only what the player sees: on-screen floors and monster, relative to the player.
pub fn features<R: GenRandFloat>(g: &GameState<R>) -> [f32; INPUTS] {
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
    // Two floors right below the player, three above; missing ones are far away.
    let above = floors
        .iter()
        .position(|f| f.1 > 0.0)
        .unwrap_or(floors.len());
    for i in 0..5 {
        let j = above as isize - 2 + i as isize;
        let (dx, dy) = match (j >= 0).then(|| floors.get(j as usize)).flatten() {
            Some(&floor) => floor,
            None => (0.0, if i < 2 { -2.0 } else { 2.0 }),
        };
        out[2 + 2 * i] = dx;
        out[3 + 2 * i] = dy;
    }

    let m = &g.monster;
    if !m.is_dead && m.position.y < 1.2 {
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
pub fn wrap_dx(dx: f32) -> f32 {
    let w = SCREEN_WEIGHT;
    let mut d = dx % w;
    if d > w / 2.0 {
        d -= w;
    }
    if d < -w / 2.0 {
        d += w;
    }
    d
}
