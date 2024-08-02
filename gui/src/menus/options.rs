

use game_logic::Difficulty;
use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, Skin};

use super::Menu;
use super::{Resources, Backgrounds};
use super::{SCREEN_WEIGHT, SCREEN_HEIGHT};
use super::Options;
use std::ops::{Deref, DerefMut};
use crate::resources::{Buttons, Labels};

use super::controls::*;

const MAX_NICKNAME_LEN: usize = 10;

pub struct OptionsMenu<'a> {
    menu: Menu<'a>,
    nickname: &'a mut String,
    music_on: CheckBox<'a>,
    sounds_on: CheckBox<'a>,
    back_button: Button<'a>,
    difficulty: &'a mut Difficulty,
    difficulty_button: Button3Way<'a>,
    last_resize: f64,
}

impl OptionsMenu<'_> {
    pub fn new<'a>(resources: &'a Resources, options: &'a mut Options) -> OptionsMenu<'a> {
        let (cb_t, cb_size) =  resources.get_button(&Buttons::CheckBox);
        let cb_size = (cb_size.0 * SCREEN_WEIGHT, cb_size.1 * SCREEN_HEIGHT);

        let difficulty_textures = [
            resources.get_label(&Labels::Practice),
            resources.get_label(&Labels::Normal),
            resources.get_label(&Labels::Unreal),
        ];
        let difficulty_w = SCREEN_WEIGHT * 115.0 / 420.0;
        let difficulty = options.difficulty as usize;

        OptionsMenu {
            menu: Menu::new(resources, Backgrounds::Options),
            nickname: &mut options.nickname,
            music_on: CheckBox::new(&cb_t.on, &cb_t.off, cb_size, &mut options.music_on, (SCREEN_WEIGHT / 4.0, 0.26)),
            sounds_on: CheckBox::new(&cb_t.on, &cb_t.off, cb_size, &mut options.sounds_on, (SCREEN_WEIGHT / 4.0, 0.04)),
            back_button: create_button(&resources, &Buttons::Back, (SCREEN_WEIGHT / 4.0, -SCREEN_HEIGHT / 3.5)),
            difficulty_button: Button3Way::new(difficulty_textures, difficulty_w, difficulty, (SCREEN_WEIGHT / 4.0, -0.21) ),
            difficulty: &mut options.difficulty,
            last_resize: get_time() - 2.0,
        }
    }

    pub fn draw_update(&mut self) -> bool {
        self.update_size();
        self.draw_background();
        self.draw_editbox();
        self.music_on.draw(&self.menu.size_params);
        self.sounds_on.draw(&self.menu.size_params);
        *self.difficulty = Difficulty::from_usize(
            self.difficulty_button.draw(&self.menu.size_params)
        );
        self.size_params.draw_border();

        return macroquad::input::get_keys_down().contains(&macroquad::input::KeyCode::Escape) ||
           self.back_button.draw(&self.menu.size_params)
    }

    fn draw_editbox(&mut self) {
        let transparent_color = Color::from_rgba(0, 0, 0, 0);
        let lu = get_time();
        let dt = lu - self.last_resize;
        if dt > 0.2 { // optimization. Cannot update with higher freq
            self.last_resize = lu;
            root_ui().pop_skin();
            let editbox_style = root_ui()
                .style_builder()
                .text_color(WHITE)
                //.font(self.resources.get_font())
                .font(include_bytes!("../../../resources/fonts/Unitblock.ttf")).unwrap()
                .font_size((65.0 * self.size_params.font_scale) as u16)
                .color(transparent_color)
                .color_clicked(transparent_color)
                .build();
            let group_style = root_ui().style_builder().color(transparent_color).color_clicked(transparent_color).build();
            let skin = Skin {
                editbox_style,
                group_style,
                ..root_ui().default_skin()
            };
            root_ui().push_skin(&skin);
        }
        let w = SCREEN_WEIGHT / 2.0;
        let r = self.size_params.rectangle_transform(
            (SCREEN_WEIGHT / 2.0 - w / 2.0, SCREEN_HEIGHT / 4.5), 
            (w, SCREEN_HEIGHT / 15.0));
        root_ui().group(hash!(), (r.x, r.y).into(), |_| {});
        root_ui().group(hash!(), (r.x, r.h).into(), |_| {});
        root_ui().same_line(0.);
        root_ui().editbox(2, (r.w, r.h).into() , self.nickname);
        if self.nickname.len() > MAX_NICKNAME_LEN {
            *self.nickname = self.nickname[0..MAX_NICKNAME_LEN].to_string();
        }
    }
}

impl<'a> Deref for OptionsMenu<'a> {
    type Target = Menu<'a>;

    fn deref(&self) -> &Self::Target {
        &self.menu
    }
}

impl<'a> DerefMut for OptionsMenu<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.menu
    }
}