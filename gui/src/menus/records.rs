use game_logic::Difficulty;

use super::Menu;
use super::SCREEN_WEIGHT;
use super::controls::{Button, create_button};
use super::{Backgrounds, RecordTables, Resources};
use crate::draw::draw_world_text;
use crate::resources::Buttons;
use std::ops::{Deref, DerefMut};

use macroquad::prelude::*;

pub struct RecordsMenu<'a> {
    menu: Menu<'a>,
    back_button: Button<'a>,
    clear_button: Button<'a>,
    record_tables: &'a mut RecordTables,
}

impl RecordsMenu<'_> {
    pub fn new<'a>(
        resources: &'a Resources,
        record_tables: &'a mut RecordTables,
    ) -> RecordsMenu<'a> {
        RecordsMenu {
            menu: Menu::new(resources, Backgrounds::Records),
            back_button: create_button(&resources, &Buttons::Back, (0.0, -0.8)),
            clear_button: create_button(&resources, &Buttons::ClearRecords, (0.38, -0.63)),
            record_tables,
        }
    }

    pub fn draw_update(&mut self) -> bool {
        self.menu.begin_frame();
        self.draw_records();
        let letterbox = &self.menu.letterbox;
        if self.clear_button.draw(letterbox) {
            self.record_tables.set_default();
        }
        return macroquad::input::get_keys_down().contains(&macroquad::input::KeyCode::Escape)
            || self.back_button.draw(letterbox);
    }

    fn draw_records(&self) {
        let record_table = self.record_tables.get_table(&Difficulty::Normal);
        let count = record_table.names.len();
        for i in 0..count {
            self.draw_nick(
                &record_table.names[i],
                record_table.scores[i],
                count - i - 1,
                YELLOW,
                0.03,
            );
        }
        let record_table = self.record_tables.get_table(&Difficulty::Unreal);
        for i in 0..count {
            self.draw_nick(
                &record_table.names[i],
                record_table.scores[i],
                count - i - 1,
                RED,
                0.67,
            );
        }
    }

    fn draw_nick(&self, name: &String, score: usize, pos: usize, color: Color, start_pos: f32) {
        let letterbox = &self.menu.letterbox;
        let y = start_pos - 0.10 * (pos + 1) as f32;
        let font = self.resources.get_font();
        let name = format!("{}. {}", pos + 1, name);
        draw_world_text(
            letterbox,
            &name,
            (-SCREEN_WEIGHT / 2.0 * 0.7, y),
            color,
            font,
        );
        let score = score.to_string();
        draw_world_text(
            letterbox,
            &score,
            (SCREEN_WEIGHT / 2.0 * 0.4, y),
            color,
            font,
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
