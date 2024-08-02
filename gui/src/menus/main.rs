use super::Menu;
use super::{Resources, Backgrounds}; //, Labels, SCREEN_WEIGHT, SCREEN_HEIGHT
use std::ops::{Deref, DerefMut};
use super::controls::{create_button, Button};
use crate::resources::Buttons;
use macroquad::input::{get_keys_down, KeyCode};

#[allow(dead_code)]
#[derive(PartialEq, Eq)]
pub enum ScreenType {
    Game,
    MainMenu,
    RecordsMenu,
    HtpMenu,
    AboutMenu,
    OptionsMenu,
    GameAi,
}

pub struct MainMenu<'a> {
    menu: Menu<'a>,
    options_button: Button<'a>,
    newgame_button: Button<'a>,
    htp_button: Button<'a>,
    records_button: Button<'a>,
    about_button: Button<'a>,
    // ai_button: Button<'a>,
}

impl MainMenu<'_> {
    pub fn new<'a>(resources: &'a Resources) -> MainMenu<'a> {
        MainMenu {
            menu: Menu::new(resources, Backgrounds::MainMenu),
            newgame_button: create_button(&resources, &Buttons::NewGame, (-0.37, 0.02)),
            options_button: create_button(&resources, &Buttons::Options, (-0.04, -0.17)),
            records_button: create_button(&resources, &Buttons::Records, (0.26, -0.38)),
            about_button: create_button(&resources, &Buttons::About, (0.47, -0.61)),
            htp_button: create_button(&resources, &Buttons::HowToPlay, (-0.21, -0.78)),
            // ai_button: create_button(&resources, &Buttons::Ai, (0.47, 0.02)),
        }
    }

    pub fn draw_update(&mut self) -> ScreenType {
        self.update_size();
        self.draw_background();
        self.size_params.draw_border();
        let pressed_keys = get_keys_down();

        if self.newgame_button.draw(&self.menu.size_params) 
            || pressed_keys.contains(&KeyCode::Space) || pressed_keys.contains(&KeyCode::Key1)
        {
            ScreenType::Game
        } else if self.options_button.draw(&self.menu.size_params) || pressed_keys.contains(&KeyCode::Key2) {
            ScreenType::OptionsMenu
        } else if self.records_button.draw(&self.menu.size_params) || pressed_keys.contains(&KeyCode::Key3) {
            ScreenType::RecordsMenu
        } else if self.about_button.draw(&self.menu.size_params) || pressed_keys.contains(&KeyCode::Key4) {
            ScreenType::AboutMenu
        } else if self.htp_button.draw(&self.menu.size_params) || pressed_keys.contains(&KeyCode::Key5) {
            ScreenType::HtpMenu
        // } else if self.ai_button.draw(&self.menu.size_params) {
        //     ScreenType::GameAi
        } else {
            ScreenType::MainMenu
        }
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