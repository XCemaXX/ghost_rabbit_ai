use game_logic::Difficulty;

use super::Menu;
use super::{Resources, Backgrounds, RecordTables};
use std::ops::{Deref, DerefMut};
use super::controls::{create_button, Button};
use crate::resources::Buttons;
use super::{SCREEN_WEIGHT};

use macroquad::prelude::*;

pub struct RecordsMenu<'a> {
    menu: Menu<'a>,
    back_button: Button<'a>,
    clear_button: Button<'a>,
    record_tables: &'a mut RecordTables,
}

impl RecordsMenu<'_> {
    pub fn new<'a>(resources: &'a Resources, record_tables: &'a mut RecordTables) -> RecordsMenu<'a> {
        RecordsMenu {
            menu: Menu::new(resources, Backgrounds::Records),
            back_button: create_button(&resources, &Buttons::Back, (0.0, -0.8)),
            clear_button: create_button(&resources, &Buttons::ClearRecords, (0.38, -0.63)),
            record_tables
        }
    }

    pub fn draw_update(&mut self) -> bool {
        self.update_size();
        self.draw_background();
        self.draw_records();
        self.size_params.draw_border();
        if self.clear_button.draw(&self.menu.size_params) {
            self.record_tables.set_default();
        }
        return macroquad::input::get_keys_down().contains(&macroquad::input::KeyCode::Escape) ||
           self.back_button.draw(&self.menu.size_params)
    }

    fn draw_records(&self) {
        let record_table = self.record_tables.get_table(&Difficulty::Normal);
        let count = record_table.names.len();
        for i in 0..count {
            self.draw_nick(&record_table.names[i], record_table.scores[i], count - i - 1, YELLOW, 0.03);
        }
        let record_table = self.record_tables.get_table(&Difficulty::Unreal);
        for i in 0..count {
            self.draw_nick(&record_table.names[i], record_table.scores[i], count - i - 1, RED, 0.67);
        }
    }

    fn draw_nick(&self, name: &String, score: usize, pos: usize, color: Color, start_pos: f32) {
        let coords = self.size_params.get_pixel_coords(-SCREEN_WEIGHT / 2.0 * 0.7, 
            start_pos - 0.10 * (pos + 1) as f32);
        draw_text_ex(
            &format!("{}. {}", pos + 1, name),
            coords.0, coords.1,
            TextParams {
                font_size: 50,
                font_scale: self.size_params.font_scale,
                color: color,
                font: Some(self.resources.get_font()),
                ..Default::default()
            },
        );
        let coords = self.size_params.get_pixel_coords(SCREEN_WEIGHT / 2.0 * 0.4, 
            start_pos - 0.10 * (pos + 1) as f32);
        draw_text_ex(
            &format!("{}", score),
            coords.0, coords.1,
            TextParams {
                font_size: 50,
                font_scale: self.size_params.font_scale,
                color: color,
                font: Some(self.resources.get_font()),
                ..Default::default()
            },
        );
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