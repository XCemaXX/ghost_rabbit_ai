use macroquad::prelude::*;

use super::Difficulty;
use super::MonsterType;

use std::collections::HashMap;

#[derive(PartialEq, Hash, Eq)]
pub enum Backgrounds {
    Game,
    MainMenu,
    Records,
}

#[derive(PartialEq, Hash, Eq)]
pub enum Labels {
    Pause,
    GameOver,
    Record,
    Win,
}

pub struct Resources {
    platforms: HashMap<Difficulty, Texture2D>,
    monsters: HashMap<MonsterType, Texture2D>,
    backgrounds: HashMap<Backgrounds, Texture2D>,
    player: Texture2D,
    labels: HashMap<Labels, Texture2D>,
    font: Font,
}

impl Resources {
    pub fn load_resources() -> Self {
        let mut r = Self {
            platforms: HashMap::new(),
            monsters: HashMap::new(),
            backgrounds: HashMap::new(),
            player: Texture2D::empty(),
            labels: HashMap::new(),
            font: load_ttf_font_from_bytes(include_bytes!("../../resources/fonts/Unitblock.ttf")).unwrap(),
        };
        r.platforms.insert(Difficulty::Practice, Texture2D::from_file_with_format(include_bytes!("../../resources/textures/floor_practice.png"), None));
        r.platforms.insert(Difficulty::Normal, Texture2D::from_file_with_format(include_bytes!("../../resources/textures/floor_medium.png"), None));
        r.platforms.insert(Difficulty::Unreal, Texture2D::from_file_with_format(include_bytes!("../../resources/textures/floor_hard.png"), None));

        r.monsters.insert(MonsterType::Mummy, Texture2D::from_file_with_format(include_bytes!("../../resources/textures/monster_mummy.png"), None));
        r.monsters.insert(MonsterType::Jason, Texture2D::from_file_with_format(include_bytes!("../../resources/textures/monster_jason.png"), None));
        r.monsters.insert(MonsterType::Vampire, Texture2D::from_file_with_format(include_bytes!("../../resources/textures/monster_vampire.png"), None));
        r.monsters.insert(MonsterType::Frankenstein, Texture2D::from_file_with_format(include_bytes!("../../resources/textures/monster_frankenstein.png"), None));

        r.backgrounds.insert(Backgrounds::Game, Texture2D::from_file_with_format(include_bytes!("../../resources/textures/background_game.png"), None));
        r.backgrounds.insert(Backgrounds::MainMenu, Texture2D::from_file_with_format(include_bytes!("../../resources/textures/background_main.png"), None));
        r.backgrounds.insert(Backgrounds::Records, Texture2D::from_file_with_format(include_bytes!("../../resources/textures/background_records.png"), None));

        r.labels.insert(Labels::Pause, Texture2D::from_file_with_format(include_bytes!("../../resources/textures/label_pause.png"), None));
        r.labels.insert(Labels::GameOver, Texture2D::from_file_with_format(include_bytes!("../../resources/textures/label_gameover.png"), None));
        r.labels.insert(Labels::Record, Texture2D::from_file_with_format(include_bytes!("../../resources/textures/label_record.png"), None));
        r.labels.insert(Labels::Win, Texture2D::from_file_with_format(include_bytes!("../../resources/textures/label_win.png"), None));

        r.player = Texture2D::from_file_with_format(include_bytes!("../../resources/textures/rabbit.png"), None);
        return r;
    }

    pub fn get_platform_texture(&self, d: &Difficulty) -> &Texture2D {
        &self.platforms[d]
    }

    pub fn get_monster_texture(&self, m: &MonsterType) -> &Texture2D {
        &self.monsters[m]
    }

    pub fn get_background(&self, b: &Backgrounds) -> &Texture2D {
        &self.backgrounds[b]
    }

    pub fn get_label(&self, l: &Labels) -> (&Texture2D, f32) {
        let size = self.labels[l].size();
        (&self.labels[l], size.x / size.y)
    }

    pub fn get_player(&self, frame: usize) -> (&Texture2D, DrawTextureParams) {
        (&self.player, DrawTextureParams {
            source: Some(Rect { x: 50.0 * frame as f32, y: 0.0, w: 50.0, h: 88.0 }),
            ..Default::default()
        })
    }

    pub fn get_font(&self) -> &Font {
        &self.font
    }
}