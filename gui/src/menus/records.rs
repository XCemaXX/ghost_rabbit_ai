use super::Menu;
use super::{Resources, Backgrounds};
use std::ops::{Deref, DerefMut};

pub struct RecordsMenu<'a> {
    menu: Menu<'a>,
}

impl RecordsMenu<'_> {
    pub fn new<'a>(resources: &'a Resources) -> RecordsMenu<'a> {
        RecordsMenu {
            menu: Menu::new(resources, Backgrounds::Records),
        }
    }

    pub fn draw_update(&mut self) {
        self.update_size();
        self.draw_background();
        self.size_params.draw_border();
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