use super::primitives::{Vec2, Circle};
use super::SCREEN_HEIGHT;

const SECS_TO_JUMP_UP: f32 = 0.6;
pub const MAX_JUMP_Y: f32 = SCREEN_HEIGHT / 2.0 * 0.9;
const ACCELERATION: f32 = - MAX_JUMP_Y * 2.0 / SECS_TO_JUMP_UP / SECS_TO_JUMP_UP;
const MAX_SPEED: f32 = MAX_JUMP_Y / SECS_TO_JUMP_UP - ACCELERATION * SECS_TO_JUMP_UP / 2.0;

pub struct Player {
    pub position: Vec2,
    pub radius: f32,
    pub speed: Vec2,
    acceleration: f32,
    y_move_limit: f32,
}

impl Player {
    pub fn new(start_position: Vec2, y_move_limit: f32) -> Self {
        Self {
            position: Vec2{ x: start_position.x, y: start_position.y },
            radius: SCREEN_HEIGHT / 32.0 / 2.0,
            speed: Vec2{ x: 0.0, y: 0.0 },
            acceleration: ACCELERATION,
            y_move_limit,
        }
    }

    pub fn get_bounding_box(&self) -> Circle {
        Circle {
            x: self.position.x,
            y: self.position.y,
            r: self.radius,
        }
    }

    pub fn set_max_speed(&mut self) {
        self.speed.y = MAX_SPEED;
    }

    pub fn move_player(&mut self, dt: f32) -> f32 {
        self.speed.y += self.acceleration * dt;
        let new_pos = self.position.y + self.speed.y * dt;
        let d_pos = new_pos - self.y_move_limit;

        return if d_pos > 0.0 {
            self.position.y = self.y_move_limit;
            d_pos
        } else {
            self.position.y = new_pos;
            0.0
        }
    }
}