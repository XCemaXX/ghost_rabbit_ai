use super::Menu;
use super::{Resources, Backgrounds};
use std::ops::{Deref, DerefMut};
use super::controls::{create_button, Button};
use crate::resources::Buttons;

pub struct RecordsMenu<'a> {
    menu: Menu<'a>,
    back_button: Button<'a>,
}

impl RecordsMenu<'_> {
    pub fn new<'a>(resources: &'a Resources) -> RecordsMenu<'a> {
        RecordsMenu {
            menu: Menu::new(resources, Backgrounds::Records),
            back_button: create_button(&resources, &Buttons::Back, (0.0, -0.8)),
        }
    }

    pub fn draw_update(&mut self) -> bool {
        self.update_size();
        self.draw_background();
        self.size_params.draw_border();
        self.back_button.draw(&self.menu.size_params)
    }
}

impl<'a> Deref for RecordsMenu<'a> {
    type Target = Menu<'a>;

    fn deref(&self) -> &Self::Target {
        &self.menu
    }
}

impl<'a> DerefMut for RecordsMenu<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.menu
    }
}