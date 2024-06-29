use super::resources::{Resources, Backgrounds};
use super::square_screen::FixedRatioScreen;
use game_logic::{SCREEN_WEIGHT, SCREEN_HEIGHT};

use macroquad::prelude::*;

pub struct RecordsMenu<'a> {
    size_params: FixedRatioScreen, 
    resources: &'a Resources,
}

impl RecordsMenu<'_> {
    pub fn new<'a>(resources: &'a Resources) -> RecordsMenu<'a> {
        RecordsMenu {
            size_params: FixedRatioScreen::new(SCREEN_WEIGHT / SCREEN_HEIGHT),
            resources,
        }
    }

    pub fn update_size(&mut self) {
        self.size_params = FixedRatioScreen::new(SCREEN_WEIGHT / SCREEN_HEIGHT); 
    }

    pub fn draw(&self) {
        self.draw_background();
        self.size_params.draw_border();
    }

    fn draw_background(&self) {
        //draw_rectangle_lines(size_params.offset_x, size_params.offset_y, size_params.width, size_params.width, 2.0, BLACK);
        let s = self.size_params.rectangle_transform(
            (0.0, 0.0), 
            (SCREEN_WEIGHT, SCREEN_HEIGHT));
        //draw_rectangle(s.0, s.1, s.2, s.3, BLACK);
        draw_texture_ex(
            self.resources.get_background(&Backgrounds::Records), s.0, s.1, WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(s.2, s.3)),
                ..Default::default()
            },
        );
    }
}