use macroquad::prelude::*;

use super::Difficulty;
use super::MonsterType;

use std::collections::HashMap;

#[derive(PartialEq, Hash, Eq)]
pub enum Backgrounds {
    Game,
    MainMenu,
    Records,
    Options,
    HowToPlay,
    About,
}

#[derive(PartialEq, Hash, Eq)]
pub enum Labels {
    Pause, GameOver, Win,
    Record,
    Normal, Practice, Unreal,
}

#[derive(PartialEq, Hash, Eq)]
pub enum Buttons {
    Back,
    About,
    HowToPlay,
    NewGame,
    Options,
    Records,
    CheckBox,
}

pub struct ButtonTextures{
    pub on: Texture2D, 
    pub off:Texture2D
}

pub struct Resources {
    platforms: HashMap<Difficulty, Texture2D>,
    monsters: HashMap<MonsterType, Texture2D>,
    backgrounds: HashMap<Backgrounds, Texture2D>,
    player: Texture2D,
    labels: HashMap<Labels, Texture2D>,
    buttons: HashMap<Buttons, ButtonTextures>,
    font: Font,
    legacy_screen_size: (f32, f32),
}

impl Resources {
    pub fn load_resources() -> Self {
        let mut r = Self {
            platforms: HashMap::new(),
            monsters: HashMap::new(),
            backgrounds: HashMap::new(),
            player: Texture2D::empty(),
            labels: HashMap::new(),
            buttons: HashMap::new(),
            font: load_ttf_font_from_bytes(include_bytes!("../../resources/fonts/Unitblock.ttf")).unwrap(),
            legacy_screen_size: (0.0, 0.0)
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
        r.backgrounds.insert(Backgrounds::Options, Texture2D::from_file_with_format(include_bytes!("../../resources/textures/background_options.png"), None));
        r.backgrounds.insert(Backgrounds::HowToPlay, Texture2D::from_file_with_format(include_bytes!("../../resources/textures/background_htp.png"), None));
        r.backgrounds.insert(Backgrounds::About, Texture2D::from_file_with_format(include_bytes!("../../resources/textures/background_about.png"), None));
        r.legacy_screen_size = r.backgrounds[&Backgrounds::Game].size().into();

        r.labels.insert(Labels::Pause, Texture2D::from_file_with_format(include_bytes!("../../resources/textures/label_pause.png"), None));
        r.labels.insert(Labels::GameOver, Texture2D::from_file_with_format(include_bytes!("../../resources/textures/label_gameover.png"), None));
        r.labels.insert(Labels::Record, Texture2D::from_file_with_format(include_bytes!("../../resources/textures/label_record.png"), None));
        r.labels.insert(Labels::Win, Texture2D::from_file_with_format(include_bytes!("../../resources/textures/label_win.png"), None));
        
        r.labels.insert(Labels::Normal, Texture2D::from_file_with_format(include_bytes!("../../resources/textures/label_normal.png"), None));
        r.labels.insert(Labels::Practice, Texture2D::from_file_with_format(include_bytes!("../../resources/textures/label_practice.png"), None));
        r.labels.insert(Labels::Unreal, Texture2D::from_file_with_format(include_bytes!("../../resources/textures/label_unreal.png"), None));

        r.buttons.insert(Buttons::CheckBox, ButtonTextures{
            on: Texture2D::from_file_with_format(include_bytes!("../../resources/textures/checkbox_on.png"), None),
            off: Texture2D::from_file_with_format(include_bytes!("../../resources/textures/checkbox_off.png"), None)
        });
        r.buttons.insert(Buttons::Back, ButtonTextures{
            on: Texture2D::from_file_with_format(include_bytes!("../../resources/textures/button_back_on.png"), None),
            off: Texture2D::from_file_with_format(include_bytes!("../../resources/textures/button_back_off.png"), None)
        });
        r.buttons.insert(Buttons::About, ButtonTextures{
            on: Texture2D::from_file_with_format(include_bytes!("../../resources/textures/button_about_on.png"), None),
            off: Texture2D::from_file_with_format(include_bytes!("../../resources/textures/button_about_off.png"), None)
        });
        r.buttons.insert(Buttons::HowToPlay, ButtonTextures{
            on: Texture2D::from_file_with_format(include_bytes!("../../resources/textures/button_htp_on.png"), None),
            off: Texture2D::from_file_with_format(include_bytes!("../../resources/textures/button_htp_off.png"), None)
        });
        r.buttons.insert(Buttons::NewGame, ButtonTextures{
            on: Texture2D::from_file_with_format(include_bytes!("../../resources/textures/button_newgame_on.png"), None),
            off: Texture2D::from_file_with_format(include_bytes!("../../resources/textures/button_newgame_off.png"), None)
        });
        r.buttons.insert(Buttons::Options, ButtonTextures{
            on: Texture2D::from_file_with_format(include_bytes!("../../resources/textures/button_options_on.png"), None),
            off: Texture2D::from_file_with_format(include_bytes!("../../resources/textures/button_options_off.png"), None)
        });
        r.buttons.insert(Buttons::Records, ButtonTextures{
            on: Texture2D::from_file_with_format(include_bytes!("../../resources/textures/button_records_on.png"), None),
            off: Texture2D::from_file_with_format(include_bytes!("../../resources/textures/button_records_off.png"), None)
        });

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

    pub fn get_button(&self, b: &Buttons) -> (&ButtonTextures, (f32, f32)) {
        let size = self.buttons[b].on.size();
        (&self.buttons[b], 
            (size.x / self.legacy_screen_size.0, size.y / self.legacy_screen_size.1))
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