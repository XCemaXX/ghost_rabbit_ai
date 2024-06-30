use super::primitives::{Vec2, Rectangle};
use super::{SCREEN_HEIGHT, SCREEN_WEIGHT};

const FLOOR_WIDTH: f32 = SCREEN_WEIGHT / 6.3;
const FLOOR_HIGHT: f32 = SCREEN_HEIGHT / 32.0;

pub struct Floor {
    pub position: Vec2,
    pub size: Vec2,
}

impl Floor {
    pub fn get_bounding_box(&self) -> Rectangle {
        Rectangle {
            x: self.position.x,
            y: self.position.y,
            w: self.size.x,
            h: self.size.y,
        }
    }

    pub fn new(position: Vec2) -> Self { 
        Self {
            position,
            size: Vec2{ x: FLOOR_WIDTH, y: FLOOR_HIGHT}
        }
    }
}