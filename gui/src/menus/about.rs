use super::Menu;
use super::controls::{Button, create_button};
use super::{Backgrounds, Resources};
use crate::resources::Buttons;
use std::ops::{Deref, DerefMut};

pub struct AboutMenu<'a> {
    menu: Menu<'a>,
    back_button: Button<'a>,
}

impl AboutMenu<'_> {
    pub fn new<'a>(resources: &'a Resources) -> AboutMenu<'a> {
        AboutMenu {
            menu: Menu::new(resources, Backgrounds::About),
            back_button: create_button(&resources, &Buttons::Back, (0.0, -0.8)),
        }
    }

    pub fn draw_update(&mut self) -> bool {
        self.menu.begin_frame();
        let letterbox = &self.menu.letterbox;
        return macroquad::input::get_keys_down().contains(&macroquad::input::KeyCode::Escape)
            || self.back_button.draw(letterbox);
    }
}

impl<'a> Deref for AboutMenu<'a> {
    type Target = Menu<'a>;

    fn deref(&self) -> &Self::Target {
        &self.menu
    }
}

impl<'a> DerefMut for AboutMenu<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.menu
    }
}
