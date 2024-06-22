use super::primitives::{Vec2, Rect};
use super::{SCREEN_HEIGHT, SCREEN_WEIGHT};

pub struct Monster {
    pub position: Vec2,
    pub size: Vec2,
    speed: Vec2,
    //type
}

impl Monster {
    pub fn new(position: Vec2) -> Self {
        // todo
        Self { 
            position,
            size: Vec2 { x: SCREEN_WEIGHT * (56.0 - 5.0) / 420.0, y: SCREEN_HEIGHT * (68.0 - 5.0) / 640.0 },
            speed: Vec2 { x: 0.0, y: 0.0 },
        }
    }

    fn get_bounding_box() -> Rect {
        todo!()
    }
}