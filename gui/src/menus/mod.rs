mod about;
mod controls;
mod htp;
mod main;
mod options;
mod records;

use super::resources::{Backgrounds, Resources};
use super::{Options, RecordTables};
use crate::draw::{draw_background, game_letterbox};
use crate::letterbox::Letterbox;
use game_logic::{SCREEN_HEIGHT, SCREEN_WEIGHT};

pub use about::AboutMenu;
pub use htp::HowToPlayMenu;
pub use main::{MainMenu, ScreenType};
pub use options::OptionsMenu;
pub use records::RecordsMenu;

pub struct Menu<'a> {
    letterbox: Letterbox,
    resources: &'a Resources,
    background: Backgrounds,
}

impl Menu<'_> {
    fn new<'a>(resources: &'a Resources, background: Backgrounds) -> Menu<'a> {
        Menu {
            letterbox: game_letterbox(),
            resources,
            background,
        }
    }

    /// Call first in each frame: the window can change size at any time.
    fn begin_frame(&mut self) {
        self.letterbox.update();
        draw_background(
            &self.letterbox,
            self.resources.get_background(&self.background),
        );
    }
}
