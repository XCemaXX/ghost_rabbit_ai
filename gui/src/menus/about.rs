use super::Menu;
use super::{Resources, Backgrounds};
use std::ops::{Deref, DerefMut};

pub struct AboutMenu<'a> {
    menu: Menu<'a>,
}

impl AboutMenu<'_> {
    pub fn new<'a>(resources: &'a Resources) -> AboutMenu<'a> {
        AboutMenu {
            menu: Menu::new(resources, Backgrounds::About),
        }
    }

    pub fn draw_update(&mut self) {
        self.update_size();
        self.draw_background();
        self.size_params.draw_border();
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