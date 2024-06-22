
use macroquad::prelude::{screen_height, screen_width};

pub struct SquareScreen {
    pub offset_x: f32,
    pub offset_y: f32,
    pub width: f32,
    pub font_scale: f32, 
}

impl SquareScreen {
    pub fn new() -> Self {
        let width = screen_width().min(screen_height());
        let offset_x = (screen_width() - width) / 2.;
        let offset_y = (screen_height() - width) / 2.;
        let font_scale = width / 800.0;
        Self { offset_x, offset_y, width, font_scale }
    }

    pub fn normalize_coords(&self, x: f32, y: f32) -> (f32, f32) {
        (
            (x - self.offset_x) / self.width * 2.0 - 1.0,
            (-y + self.offset_y + self.width) * 2.0 / self.width - 1.0
        )
    }

    pub fn get_pixel_coords(&self, x_normalized: f32, y_normalized: f32) -> (f32, f32) {
        (
            self.offset_x + (x_normalized + 1.0) / 2.0 * self.width,
            self.offset_y + self.width - (y_normalized + 1.0) / 2.0 * self.width,
        )
    }

    pub fn rectangle_transform(&self, pos: (f32, f32), size: (f32, f32)) -> (f32, f32, f32, f32) {
        let (x, y) = self.get_pixel_coords(pos.0, pos.1);
        let (w, h) = (size.0 * self.width / 2.0, size.1 * self.width / 2.0);
        (
            x - w / 2.0, y - h / 2.0,
            w, h,
        )
    }

    pub fn circle_transform(&self, pos: (f32, f32), radius: f32) -> (f32, f32, f32) {
        let (x, y) = self.get_pixel_coords(pos.0, pos.1);
        (
            x, y,
            radius * self.width / 2.0
        )
    }
}

