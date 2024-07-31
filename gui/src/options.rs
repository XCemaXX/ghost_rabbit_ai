use game_logic::Difficulty;
pub struct Options {
    pub nickname: String,
    pub music_on: bool,
    pub sounds_on: bool,
    pub difficulty: Difficulty,
}

impl Options {
    pub fn new() -> Self {
        let mut options = Self {
            nickname: "XCemaXX".into(),
            music_on: false,
            sounds_on: false,
            difficulty: Difficulty::Normal,
        };
        let storage = &mut quad_storage::STORAGE.lock().unwrap();
        if let Some(nickname) = storage.get("nickname") {
            options.nickname = nickname;
        }
        if let Some(music_on) = storage.get("music_on") {
            options.music_on = music_on == "t";
        }
        if let Some(sounds_on) = storage.get("sounds_on") {
            options.sounds_on = sounds_on == "t";
        }
        if let Some(difficulty) = storage.get("difficulty") {
            options.difficulty = Difficulty::from_str(difficulty.as_str());
        }
        options
    }

    pub fn save(&self) {
        let storage = &mut quad_storage::STORAGE.lock().unwrap();
        storage.set("nickname", &self.nickname);
        let music_on = if self.music_on {"t" } else {"f"};
        storage.set("music_on", music_on);
        let sounds_on = if self.sounds_on {"t" } else {"f"};
        storage.set("sounds_on", sounds_on);
        storage.set("difficulty", self.difficulty.as_str());
    }
}