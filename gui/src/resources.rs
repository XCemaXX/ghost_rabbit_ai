use macroquad::prelude::*;
use quad_snd::{Playback, AudioContext, Sound};
use super::{Difficulty, MonsterType};
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
    Record, P1000, PauseButton, ExitButton,
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
    ClearRecords,
    Ai,
}

pub struct ButtonTextures{
    pub on: Texture2D, 
    pub off:Texture2D
}

#[derive(PartialEq, Hash, Eq)]
pub enum Sounds {
    Background,
    Death,
    Hit,
    Start,
    Thousand,
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
    sounds: HashMap<Sounds, Sound>,
    sounds_context: AudioContext,
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
            legacy_screen_size: (0.0, 0.0),
            sounds: HashMap::new(),
            sounds_context: AudioContext::new(),
        };
        r.load_textures();
        r.load_sounds();
        r
    }

    fn load_textures(&mut self) {
        self.platforms.insert(Difficulty::Practice, Texture2D::from_file_with_format(include_bytes!("../../resources/textures/floor_practice.png"), None));
        self.platforms.insert(Difficulty::Normal, Texture2D::from_file_with_format(include_bytes!("../../resources/textures/floor_medium.png"), None));
        self.platforms.insert(Difficulty::Unreal, Texture2D::from_file_with_format(include_bytes!("../../resources/textures/floor_hard.png"), None));

        self.monsters.insert(MonsterType::Mummy, Texture2D::from_file_with_format(include_bytes!("../../resources/textures/monster_mummy.png"), None));
        self.monsters.insert(MonsterType::Jason, Texture2D::from_file_with_format(include_bytes!("../../resources/textures/monster_jason.png"), None));
        self.monsters.insert(MonsterType::Vampire, Texture2D::from_file_with_format(include_bytes!("../../resources/textures/monster_vampire.png"), None));
        self.monsters.insert(MonsterType::Frankenstein, Texture2D::from_file_with_format(include_bytes!("../../resources/textures/monster_frankenstein.png"), None));

        self.backgrounds.insert(Backgrounds::Game, Texture2D::from_file_with_format(include_bytes!("../../resources/textures/background_game.png"), None));
        self.backgrounds.insert(Backgrounds::MainMenu, Texture2D::from_file_with_format(include_bytes!("../../resources/textures/background_main.png"), None));
        self.backgrounds.insert(Backgrounds::Records, Texture2D::from_file_with_format(include_bytes!("../../resources/textures/background_records.png"), None));
        self.backgrounds.insert(Backgrounds::Options, Texture2D::from_file_with_format(include_bytes!("../../resources/textures/background_options.png"), None));
        self.backgrounds.insert(Backgrounds::HowToPlay, Texture2D::from_file_with_format(include_bytes!("../../resources/textures/background_htp.png"), None));
        self.backgrounds.insert(Backgrounds::About, Texture2D::from_file_with_format(include_bytes!("../../resources/textures/background_about.png"), None));
        self.legacy_screen_size = self.backgrounds[&Backgrounds::Game].size().into();

        self.labels.insert(Labels::Pause, Texture2D::from_file_with_format(include_bytes!("../../resources/textures/label_pause.png"), None));
        self.labels.insert(Labels::GameOver, Texture2D::from_file_with_format(include_bytes!("../../resources/textures/label_gameover.png"), None));
        self.labels.insert(Labels::Record, Texture2D::from_file_with_format(include_bytes!("../../resources/textures/label_record.png"), None));
        self.labels.insert(Labels::Win, Texture2D::from_file_with_format(include_bytes!("../../resources/textures/label_win.png"), None));
        self.labels.insert(Labels::P1000, Texture2D::from_file_with_format(include_bytes!("../../resources/textures/label_1000.png"), None));
        self.labels.insert(Labels::PauseButton, Texture2D::from_file_with_format(include_bytes!("../../resources/textures/label_pause_button.png"), None));
        self.labels.insert(Labels::ExitButton, Texture2D::from_file_with_format(include_bytes!("../../resources/textures/label_exit_button.png"), None));

        self.labels.insert(Labels::Normal, Texture2D::from_file_with_format(include_bytes!("../../resources/textures/label_normal.png"), None));
        self.labels.insert(Labels::Practice, Texture2D::from_file_with_format(include_bytes!("../../resources/textures/label_practice.png"), None));
        self.labels.insert(Labels::Unreal, Texture2D::from_file_with_format(include_bytes!("../../resources/textures/label_unreal.png"), None));

        self.buttons.insert(Buttons::CheckBox, ButtonTextures{
            on: Texture2D::from_file_with_format(include_bytes!("../../resources/textures/checkbox_on.png"), None),
            off: Texture2D::from_file_with_format(include_bytes!("../../resources/textures/checkbox_off.png"), None)
        });
        self.buttons.insert(Buttons::Back, ButtonTextures{
            on: Texture2D::from_file_with_format(include_bytes!("../../resources/textures/button_back_on.png"), None),
            off: Texture2D::from_file_with_format(include_bytes!("../../resources/textures/button_back_off.png"), None)
        });
        self.buttons.insert(Buttons::About, ButtonTextures{
            on: Texture2D::from_file_with_format(include_bytes!("../../resources/textures/button_about_on.png"), None),
            off: Texture2D::from_file_with_format(include_bytes!("../../resources/textures/button_about_off.png"), None)
        });
        self.buttons.insert(Buttons::HowToPlay, ButtonTextures{
            on: Texture2D::from_file_with_format(include_bytes!("../../resources/textures/button_htp_on.png"), None),
            off: Texture2D::from_file_with_format(include_bytes!("../../resources/textures/button_htp_off.png"), None)
        });
        self.buttons.insert(Buttons::NewGame, ButtonTextures{
            on: Texture2D::from_file_with_format(include_bytes!("../../resources/textures/button_newgame_on.png"), None),
            off: Texture2D::from_file_with_format(include_bytes!("../../resources/textures/button_newgame_off.png"), None)
        });
        self.buttons.insert(Buttons::Options, ButtonTextures{
            on: Texture2D::from_file_with_format(include_bytes!("../../resources/textures/button_options_on.png"), None),
            off: Texture2D::from_file_with_format(include_bytes!("../../resources/textures/button_options_off.png"), None)
        });
        self.buttons.insert(Buttons::Records, ButtonTextures{
            on: Texture2D::from_file_with_format(include_bytes!("../../resources/textures/button_records_on.png"), None),
            off: Texture2D::from_file_with_format(include_bytes!("../../resources/textures/button_records_off.png"), None)
        });
        self.buttons.insert(Buttons::ClearRecords, ButtonTextures{
            on: Texture2D::from_file_with_format(include_bytes!("../../resources/textures/button_clear_on.png"), None),
            off: Texture2D::from_file_with_format(include_bytes!("../../resources/textures/button_clear_off.png"), None)
        });
        self.buttons.insert(Buttons::Ai, ButtonTextures{
            on: Texture2D::from_file_with_format(include_bytes!("../../resources/textures/button_ai_on.png"), None),
            off: Texture2D::from_file_with_format(include_bytes!("../../resources/textures/button_ai_off.png"), None)
        });

        self.player = Texture2D::from_file_with_format(include_bytes!("../../resources/textures/rabbit.png"), None);
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

    pub fn load_sounds(&mut self) {
        self.sounds.insert(Sounds::Background, 
            Sound::load(&self.sounds_context, include_bytes!("../../resources/sounds/background.wav")));
        self.sounds.insert(Sounds::Start, 
            Sound::load(&self.sounds_context, include_bytes!("../../resources/sounds/start.wav")));
        self.sounds.insert(Sounds::Death, 
            Sound::load(&self.sounds_context, include_bytes!("../../resources/sounds/death.wav")));
        self.sounds.insert(Sounds::Hit, 
            Sound::load(&self.sounds_context, include_bytes!("../../resources/sounds/hit.wav")));
        self.sounds.insert(Sounds::Thousand, 
            Sound::load(&self.sounds_context, include_bytes!("../../resources/sounds/thousand.wav")));
    }

    pub fn play_sound_once(&self, s: &Sounds) {
        self.sounds[s].play(&self.sounds_context, quad_snd::PlaySoundParams{
            looped: false,
            volume: 0.3,
        });
    }

    pub fn play_background(&self) -> Playback {
        self.sounds[&Sounds::Background].play(&self.sounds_context, quad_snd::PlaySoundParams{
            looped: true,
            volume: 0.3,
        })
    }

    pub fn stop_background(&self, p: Playback) {
        p.stop(&self.sounds_context);
    }
}