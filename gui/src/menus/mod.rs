mod main;
mod records;
mod options;
mod about;
mod htp;
mod controls;

use super::resources::{Resources, Backgrounds};
use super::square_screen::FixedRatioScreen;
use game_logic::{SCREEN_WEIGHT, SCREEN_HEIGHT};
use super::Options;

use macroquad::prelude::*;

pub use main::MainMenu;
pub use records::RecordsMenu;
pub use options::OptionsMenu;
pub use about::AboutMenu;
pub use htp::HowToPlayMenu;

pub struct Menu<'a> {
    size_params: FixedRatioScreen, 
    resources: &'a Resources,
    background: Backgrounds,
} 

impl Menu<'_> {
    fn new<'a>(resources: &'a Resources, background: Backgrounds) -> Menu<'a> {
        Menu {
            size_params: FixedRatioScreen::new(SCREEN_WEIGHT / SCREEN_HEIGHT),
            resources,
            background,
        }
    }

    pub fn update_size(&mut self) {
        self.size_params = FixedRatioScreen::new(SCREEN_WEIGHT / SCREEN_HEIGHT); 
    }

    fn draw_background(&self) {
        let r = self.size_params.rectangle_transform(
            (0.0, 0.0), 
            (SCREEN_WEIGHT, SCREEN_HEIGHT));
        draw_texture_ex(
            self.resources.get_background(&self.background), r.x, r.y, WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(r.w, r.h)),
                ..Default::default()
            },
        );
    }
}