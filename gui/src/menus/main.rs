use super::Menu;
use super::{Resources, Backgrounds}; //, Labels, SCREEN_WEIGHT, SCREEN_HEIGHT
use std::ops::{Deref, DerefMut};

pub struct MainMenu<'a> {
    menu: Menu<'a>,
}


impl MainMenu<'_> {
    pub fn new<'a>(resources: &'a Resources) -> MainMenu<'a> {
        MainMenu {
            menu: Menu::new(resources, Backgrounds::MainMenu),
        }
    }

    pub fn draw_update(&mut self) {
        self.update_size();
        self.draw_background();
        self.size_params.draw_border();
    }
}

impl<'a> Deref for MainMenu<'a> {
    type Target = Menu<'a>;

    fn deref(&self) -> &Self::Target {
        &self.menu
    }
}

impl<'a> DerefMut for MainMenu<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.menu
    }
}