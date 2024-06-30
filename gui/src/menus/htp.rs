use super::Menu;
use super::{Resources, Backgrounds};
use std::ops::{Deref, DerefMut};

pub struct HowToPlayMenu<'a> {
    menu: Menu<'a>,
}

impl HowToPlayMenu<'_> {
    pub fn new<'a>(resources: &'a Resources) -> HowToPlayMenu<'a> {
        HowToPlayMenu {
            menu: Menu::new(resources, Backgrounds::HowToPlay),
        }
    }

    pub fn draw_update(&mut self) {
        self.update_size();
        self.draw_background();
        self.size_params.draw_border();
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