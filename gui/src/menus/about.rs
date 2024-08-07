use super::Menu;
use super::{Resources, Backgrounds};
use std::ops::{Deref, DerefMut};
use super::controls::{create_button, Button};
use crate::resources::Buttons;

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
        self.update_size();
        self.draw_background();
        self.size_params.draw_border();
        return macroquad::input::get_keys_down().contains(&macroquad::input::KeyCode::Escape) ||
           self.back_button.draw(&self.menu.size_params)
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