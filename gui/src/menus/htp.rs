use super::Menu;
use super::controls::{Button, create_button};
use super::{Backgrounds, Resources};
use crate::resources::Buttons;
use std::ops::{Deref, DerefMut};

pub struct HowToPlayMenu<'a> {
    menu: Menu<'a>,
    back_button: Button<'a>,
}

impl HowToPlayMenu<'_> {
    pub fn new<'a>(resources: &'a Resources) -> HowToPlayMenu<'a> {
        HowToPlayMenu {
            menu: Menu::new(resources, Backgrounds::HowToPlay),
            back_button: create_button(resources, &Buttons::Back, (0.0, -0.8)),
        }
    }

    pub fn draw_update(&mut self) -> bool {
        self.menu.begin_frame();
        let letterbox = &self.menu.letterbox;
        macroquad::input::get_keys_down().contains(&macroquad::input::KeyCode::Escape)
            || self.back_button.draw(letterbox)
    }
}

impl<'a> Deref for HowToPlayMenu<'a> {
    type Target = Menu<'a>;

    fn deref(&self) -> &Self::Target {
        &self.menu
    }
}

impl<'a> DerefMut for HowToPlayMenu<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.menu
    }
}
