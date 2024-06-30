// center in middle
pub struct Circle {
    pub x: f32,
    pub y: f32,
    pub r: f32,
}

// center in middle
pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

#[derive(Default, Copy, Clone)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl From<(f32, f32)> for Vec2 {
    fn from(val: (f32, f32)) -> Self {
        Self { x: val.0, y: val.1 }
    }
}

impl From<Vec2> for (f32, f32) {
    fn from(val: Vec2) -> Self {
        (val.x, val.y)
    }
}