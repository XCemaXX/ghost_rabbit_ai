
use macroquad::prelude::{screen_height, screen_width, draw_rectangle, GRAY, Rect, Circle};

pub struct FixedRatioScreen {
    offset_x: f32,
    offset_y: f32,
    height: f32,
    width: f32,
    pub font_scale: f32, 
    square_around: f32,
}

impl FixedRatioScreen {
    pub fn new(x_to_y_ratio: f32) -> Self {
        let (width, height) =  if screen_height() * x_to_y_ratio > screen_width() {
            (screen_width(), screen_width() / x_to_y_ratio)
        } else {
            (screen_height() * x_to_y_ratio, screen_height())
        };
        let square_around = height.max(width);
        let offset_x = (screen_width() - square_around) / 2.;
        let offset_y = (screen_height() - square_around) / 2.;
        let font_scale = width / 800.0;
        Self { offset_x, offset_y, height, width, font_scale, square_around }
    }

    pub fn normalize_coords(&self, x: f32, y: f32) -> (f32, f32) {
        (
            (x - self.offset_x) / self.square_around * 2.0 - 1.0,
            (-y + self.offset_y + self.square_around) * 2.0 / self.square_around - 1.0
        )
    }

    pub fn get_pixel_coords(&self, x_normalized: f32, y_normalized: f32) -> (f32, f32) {
        (
            self.offset_x + (x_normalized + 1.0) / 2.0 * self.square_around,
            self.offset_y + self.square_around - (y_normalized + 1.0) / 2.0 * self.square_around,
        )
    }

    pub fn rectangle_transform(&self, pos: (f32, f32), size: (f32, f32)) -> Rect {
        let (x, y) = self.get_pixel_coords(pos.0, pos.1);
        let (w, h) = (size.0 * self.square_around / 2.0, size.1 * self.square_around / 2.0);
        Rect{x: x - w / 2.0, y: y - h / 2.0, w, h}
    }

    pub fn circle_transform(&self, pos: (f32, f32), radius: f32) -> Circle {
        let (x, y) = self.get_pixel_coords(pos.0, pos.1);
        Circle{x, y, r: radius * self.square_around / 2.0}
    }

    fn get_border_rectangles(&self) -> ((f32, f32, f32, f32), (f32, f32, f32, f32)) {
        if screen_height() == self.height {
            let w = self.offset_x + (self.square_around - self.width) / 2.0;
            let h = self.height;
            (  
                (0.0, 0.0, w, h),
                (self.offset_x + (self.square_around + self.width) / 2.0, 0.0, w, h)
            )
        } else {
            let w = self.width;
            let h = self.offset_y + (self.square_around - self.height) / 2.0;
            (  
                (0.0, 0.0, w, h),
                (0.0, self.offset_y + (self.square_around + self.height) / 2.0, w, h)
            )
        }
    }

    pub fn draw_border(&self) {
        let (r1, r2) = self.get_border_rectangles();
        draw_rectangle(r1.0, r1.1, r1.2, r1.3, GRAY);
        draw_rectangle(r2.0, r2.1, r2.2, r2.3, GRAY);
    }
}

