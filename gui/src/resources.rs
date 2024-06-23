use macroquad::prelude::*;

use super::Difficulty;
use super::MonsterType;

pub struct Resources {
    platforms: Vec<Texture2D>,
    monsters: Vec<Texture2D>,
    backgrounds: Vec<Texture2D>,
}

impl Resources {
    pub fn load_resources() -> Self {
        let mut r = Self {
            platforms: Vec::new(),
            monsters: Vec::new(),
            backgrounds: Vec::new(),
        };
        r.platforms.push(Texture2D::from_file_with_format(include_bytes!("../../resources/textures/floor_practice.png"), None));
        r.platforms.push(Texture2D::from_file_with_format(include_bytes!("../../resources/textures/floor_medium.png"), None));
        r.platforms.push(Texture2D::from_file_with_format(include_bytes!("../../resources/textures/floor_hard.png"), None));

        r.monsters.push(Texture2D::from_file_with_format(include_bytes!("../../resources/textures/monster_mummy.png"), None));
        r.monsters.push(Texture2D::from_file_with_format(include_bytes!("../../resources/textures/monster_jason.png"), None));
        r.monsters.push(Texture2D::from_file_with_format(include_bytes!("../../resources/textures/monster_vampire.png"), None));
        r.monsters.push(Texture2D::from_file_with_format(include_bytes!("../../resources/textures/monster_frankenstein.png"), None));

        r.backgrounds.push(Texture2D::from_file_with_format(include_bytes!("../../resources/textures/background_game.png"), None));
        return r;
    }

    pub fn get_platform_texture(&self, d: &Difficulty) -> &Texture2D {
        match d {
            Difficulty::Practice => { &self.platforms[0] },
            Difficulty::Medium => { &self.platforms[1] },
            Difficulty::Hard => { &self.platforms[2] },
        }
    }

    pub fn get_monster_texture(&self, m: &MonsterType) -> &Texture2D {
        match m {
            MonsterType::Mummy => { &self.monsters[0] },
            MonsterType::Jason => { &self.monsters[1] },
            MonsterType::Vampire => { &self.monsters[2] },
            MonsterType::Frankenstein => { &self.monsters[3] },
        }
    }

    pub fn get_background(&self) -> &Texture2D {
        &self.backgrounds[0]
    }
}