use super::primitives::{Vec2, Rectangle};
use super::{SCREEN_HEIGHT, SCREEN_WEIGHT, RIGHT_X, LEFT_X, BOT_Y, TOP_Y, CENTER};
use rand_trait::GenRandFloat;

const MONSTER_WIDTH: f32 = SCREEN_WEIGHT * (56.0 - 5.0) / 420.0;
const MONSTER_HIGHT: f32 = SCREEN_HEIGHT * (68.0 - 5.0) / 640.0;

#[derive(PartialEq, Hash, Eq)]
pub enum MonsterType {
    Mummy,
    Jason,
    Vampire,
    Frankenstein,
}

pub struct Monster {
    pub position: Vec2,
    pub size: Vec2,
    speed: Vec2,
    pub dead_time: f32,
    pub guise: MonsterType,
    pub is_dead: bool,
}

impl Monster {
    pub fn new<T:GenRandFloat>(rng: &mut T, player_pos: &Vec2, speed_abs: f32) -> Self {
        let guise = match rng.gen_range(0.0..=4.0) {
            0.0..=1.0 => { MonsterType::Mummy },
            1.0..=2.0 => { MonsterType::Jason },
            2.0..=3.0 => { MonsterType::Vampire },
            _ =>  { MonsterType::Frankenstein },
        };
        let (position, speed) = gen_pos(rng, player_pos, speed_abs);
        Self { 
            position,
            size: Vec2 { x: MONSTER_WIDTH, y: MONSTER_HIGHT },
            speed,
            dead_time: 0.0,
            guise,
            is_dead: false,
        }
    }

    pub fn new_dead() -> Self {
        Self { 
            position: Vec2 { x: 0.0, y: 0.0 },
            size: Vec2 { x: MONSTER_WIDTH, y: MONSTER_HIGHT },
            speed: Vec2 { x: 0.0, y: 0.0 },
            dead_time: 0.0,
            guise: MonsterType::Mummy,
            is_dead: true,
        }
    }

    pub fn move_monster(&mut self, dt: f32) {
        if self.is_dead {
            self.dead_time += dt;
            return;
        }
        self.position.y = self.position.y + self.speed.y * dt;
        self.position.x = self.position.x + self.speed.x * dt;

        if (self.position.x - self.size.x / 2.0) > RIGHT_X && self.speed.x > 0.0
        || (self.position.x + self.size.x / 2.0) < LEFT_X && self.speed.x < 0.0
        || (self.position.y + self.size.y / 2.0) < BOT_Y {
            self.is_dead = true;
        }
    }

    pub fn get_bounding_box(&self) -> Rectangle {
        Rectangle {
            x: self.position.x,
            y: self.position.y,
            w: self.size.x * 0.8,
            h: self.size.y * 0.8,
        }
    }
}

fn gen_pos<T:GenRandFloat>(rng: &mut T, player_pos: &Vec2, speed_abs: f32) -> (Vec2, Vec2) {
    let pos = match rng.gen_range(0.0..=3.5) {
        0.0..=1.5 => { // Top
            Vec2 {
                x: rng.gen_range(LEFT_X..=RIGHT_X),
                y: TOP_Y + MONSTER_HIGHT, 
            }
        },
        1.5..=2.5 => { // Left
            Vec2 {
                x: LEFT_X - MONSTER_WIDTH,
                y: rng.gen_range(CENTER..=TOP_Y) + MONSTER_HIGHT, 
            }
        },
        _ =>  { // Right
            Vec2 {
                x: RIGHT_X + MONSTER_WIDTH,
                y: rng.gen_range(CENTER..=TOP_Y) + MONSTER_HIGHT, 
            }
        },
    };
    let x_speed = speed_abs * (player_pos.x - pos.x) / (SCREEN_WEIGHT);
    let speed = Vec2 {
        x: x_speed,
        y: -f32::sqrt(speed_abs * speed_abs - x_speed * x_speed),
    };
    return (pos, speed);
}