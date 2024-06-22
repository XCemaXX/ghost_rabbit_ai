mod primitives;
mod collide;
mod player;
mod floor;
mod monster;

use rand_trait::GenRandFloat;
use primitives::{Vec2, Circle, Rect};
use player::{Player, MAX_JUMP_Y};
use floor::Floor;
use monster::Monster;

const TOP_Y: f32 = 1.0;
const BOT_Y: f32 = -1.0;
const RIGHT_X: f32 = 0.65625;
const LEFT_X: f32 = -0.65625;
const CENTER: f32 = 0.0;

pub const SCREEN_HEIGHT: f32 = TOP_Y - BOT_Y;
pub const SCREEN_WEIGHT: f32 = RIGHT_X - LEFT_X;

const MAX_FLOORS: usize = 12;

#[derive(PartialEq)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

#[derive(PartialEq)]
pub enum Side {
    Left,
    Right,
}

pub type GameOver = bool;

pub struct GameState<T:GenRandFloat> {
    pub floors: [Floor; MAX_FLOORS],
    pub player: Player,
    pub monster: Monster,
    rng: T,
    difficulty: Difficulty,
}

impl<T:GenRandFloat> GameState<T> {
    pub fn new(rng: T, difficulty: Difficulty) -> Self {
        let mut s = Self {
            floors: [(); MAX_FLOORS].map(|_| floor::Floor::new(Vec2{ x: CENTER, y: BOT_Y - 1.0 })),
            player: Player::new( Vec2{ x:CENTER, y: CENTER }, CENTER),
            monster: Monster::new(Vec2 { x: RIGHT_X, y: TOP_Y - 0.2 }),
            rng,
            difficulty,
        };
        s.create_first_floor_under_player();
        s.next_step(0.0);
        s
    }

    pub fn next_step(&mut self, dt: f32) -> GameOver {
        let dt = dt / 2.0;
        for _ in 0..2 { // make collide more precise
            let dy = self.player.move_player(dt);
            self.move_objects_down(dy);
            self.collide_player_floors();
        }
        self.recreate_floors();

        return self.is_game_over();
    }

    pub fn move_by_x(&mut self, dt: f32, side: Side) {
        let mut ds = dt * 60.0 * 7.0 / 420.0;
        if side == Side::Left {
            ds = -ds;
        }
        self.player.position.x += ds;
        if self.player.position.x < LEFT_X {
            self.player.position.x = RIGHT_X - (LEFT_X - self.player.position.x);
        } else if self.player.position.x > RIGHT_X {
            self.player.position.x = LEFT_X + (self.player.position.x - RIGHT_X);
        }
    }

    fn is_game_over(&mut self) -> GameOver {
        if self.player.position.y < BOT_Y {
            if self.difficulty == Difficulty::Easy {
                self.player = Player::new( Vec2{ x:CENTER, y: CENTER }, CENTER);
                return false;
            }
            return true;
        }
        return false;
    }

    fn create_first_floor_under_player(&mut self) {
        self.floors[0].position = Vec2{ x: CENTER , y: BOT_Y + SCREEN_HEIGHT / 16.0 };
    }

    fn move_objects_down(&mut self, dy: f32) {
        let floors = &mut self.floors;
        for floor in floors {
            floor.position.y -= dy;
        }
        // todo add monster
    }

    fn recreate_floors(&mut self) {
        let floors = &mut self.floors;
        let mut top_floor_y = get_max_floor(floors);
        for floor in floors {
            if floor.position.y + floor.size.y < BOT_Y {
                top_floor_y = create_floor(&mut self.rng, floor, top_floor_y);
            }
        }
    }

    fn collide_player_floors(&mut self) -> bool {
        if self.player.speed.y >= 0.0 {
            return false;
        }
        for floor in &self.floors {
            if collide::is_collide(&self.player.get_bounding_box(), &floor.get_bounding_box())
            && is_player_over_floor(&self.player, &floor) {
                self.player.set_max_speed();
                return true;
            }
        }
        return false;
    }
}

fn is_player_over_floor(p: &Player, f: &Floor) -> bool {
    return p.position.y > (f.position.y + f.size.y / 2.0);
}

fn create_floor<T:GenRandFloat>(rng: &mut T, floor: &mut Floor, top_floor_y: f32) -> f32 {
    const MIN_FLOOR_DIST: f32 = SCREEN_HEIGHT / MAX_FLOORS as f32;
    floor.position.x = rng.gen_range((LEFT_X + floor.size.x / 2.0)..=(RIGHT_X - floor.size.x / 2.0));
    floor.position.y = rng.gen_range((top_floor_y + MIN_FLOOR_DIST)..=(top_floor_y + MAX_JUMP_Y));
    floor.position.y
}

fn get_max_floor(floors: &[Floor; MAX_FLOORS]) -> f32 {
    let mut top_floor_pos = BOT_Y;
    for floor in floors {
        top_floor_pos = top_floor_pos.max(floor.position.y);
    }
    top_floor_pos
}

