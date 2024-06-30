
use game_logic::{Difficulty, GameState, Side, SCREEN_HEIGHT, SCREEN_WEIGHT};
use rand_trait::GenRandFloat;
use crate::ScoreArray;

use super::resources::{Resources, Backgrounds, Labels};
use super::square_screen::FixedRatioScreen;

use macroquad::prelude::*;

const GAME_OVER_DURATION_SEC: f64 = 1.3;
const WIN_DURATION_SEC: f64 = 1.0;

#[derive(PartialEq, Eq)]
enum State {
    Running,
    Paused,
    ShowNewRecord,
    ShowGameOver,
    Finished,
}

pub struct GameScreen<'a> {
    game_engine: GameState<RandGen>, 
    size_params: FixedRatioScreen, 
    resources: &'a Resources,
    state: State,
    state_duration: f64,
    nick_name: &'a String,
    best_scores: &'a ScoreArray,
}

struct RandGen {}

impl GenRandFloat for RandGen {
    fn gen_range(&mut self, range: std::ops::RangeInclusive<f32>) -> f32 {
        let low = *range.start();
        let high = *range.end();
        rand::gen_range(low, high)
    }
}

impl GameScreen<'_> {
    pub fn new<'a>(resources: &'a Resources, difficulty: Difficulty, nick_name: &'a String, scores: &'a ScoreArray) -> GameScreen<'a> {
        GameScreen {
            game_engine: GameState::new(RandGen{}, difficulty),
            size_params: FixedRatioScreen::new(SCREEN_WEIGHT / SCREEN_HEIGHT),
            resources,
            state: State::Running,
            nick_name,
            state_duration: 0.0,
            best_scores: scores,
        }
    }

    pub fn update_size(&mut self) {
        self.size_params = FixedRatioScreen::new(SCREEN_WEIGHT / SCREEN_HEIGHT); 
    }

    pub fn update_game_state_by_input(&mut self, dt: f64) {
        if self.state != State::Running && self.state != State::Paused {
            return;
        }
        self.update_pause();
        if self.state == State::Paused {
            return;
        }
            
        for key in get_keys_down() {
            match key {
                KeyCode::Left => { self.game_engine.move_player_by_x(dt as f32, Side::Left); },
                KeyCode::Right => { self.game_engine.move_player_by_x(dt as f32, Side::Right); },
                KeyCode::Escape => { self.change_state(State::Finished); }
                _ => { },
            }
        }
    }

    pub fn next_step(&mut self, dt: f64) {
        self.state_duration += dt;
        match self.state {
            State::Running => {
                let is_game_over = self.game_engine.next_step(dt as f32);
                if is_game_over {
                    self.change_state(State::ShowGameOver); 
                }
            },
            State::ShowGameOver => {
                if self.state_duration > GAME_OVER_DURATION_SEC {
                    if self.game_engine.difficulty == Difficulty::Practice
                        || self.game_engine.get_score() < self.best_scores.first().unwrap().1 {
                        self.change_state(State::Finished);
                    } else {
                        self.change_state(State::ShowNewRecord);
                    }
                }
            },
            State::ShowNewRecord => {
                if self.state_duration > WIN_DURATION_SEC {
                    self.change_state(State::Finished);
                }
            },
            _ => {},
        }
    }

    pub fn is_game_over(&self) -> bool {
        self.state == State::Finished
    }

    pub fn get_score(&self) -> usize {
        self.game_engine.get_score()
    }

    pub fn draw(&self) {
        self.draw_background();
        self.draw_records();
        self.draw_floors();
        self.draw_player();
        self.draw_monster();
        self.draw_score();
        self.draw_nick();
        self.draw_state();
        self.size_params.draw_border();
    }

    fn draw_floors(&self) {
        let floor_texture = self.resources.get_platform_texture(&self.game_engine.difficulty);
        for floor in &self.game_engine.floors {
            let r = self.size_params.rectangle_transform(
                floor.position.into(), 
                floor.size.into());
            //draw_rectangle(f.0, f.1, f.2, f.3, GRAY);
            draw_texture_ex(
                &floor_texture, r.x, r.y, WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(r.w, r.h)),
                    ..Default::default()
                },
            );
        }
    }

    fn draw_monster(&self) {
        let monster = &self.game_engine.monster;
        if !monster.is_dead {
            let monster_texture = self.resources.get_monster_texture(&monster.guise);
            let r = self.size_params.rectangle_transform(
                monster.position.into(),
                monster.size.into());
            //draw_rectangle(monster.0, monster.1, monster.2, monster.3, RED);
            draw_texture_ex(
                &monster_texture, r.x, r.y, WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(r.w, r.h)),
                    ..Default::default()
                },
            );
        }
    }
    
    fn draw_player(&self) {
        let player = &self.game_engine.player;
        let player_pos = self.size_params.circle_transform(
            player.position.into(), 
            player.radius);
        //draw_circle(player_pos.x, player_pos.y, player_pos.r, GREEN);
    
        let r = self.size_params.rectangle_transform(
            (player.position.x, player.position.y), 
            ((player.radius * 2.) * 25.0 / 10.0, (player.radius*2.0) * 46.0 / 10.0));
        let (texture, mut texture_params) = self.resources.get_player(player.get_stage());
        texture_params.dest_size = Some(vec2(player_pos.r * 50.0 / 10.0, player_pos.r * 88.0 / 10.0));
        draw_texture_ex(texture, r.x, r.y, WHITE, texture_params);
    }
    
    fn draw_background(&self) {
        //draw_rectangle_lines(size_params.offset_x, size_params.offset_y, size_params.width, size_params.width, 2.0, BLACK);
        let r = self.size_params.rectangle_transform(
            (0.0, 0.0), 
            (SCREEN_WEIGHT, SCREEN_HEIGHT));
        //draw_rectangle(s.0, s.1, s.2, s.3, BLACK);
        draw_texture_ex(
            self.resources.get_background(&Backgrounds::Game), r.x, r.y, WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(r.w, r.h)),
                ..Default::default()
            },
        );
    }

    fn draw_score(&self) {
        let coords = self.size_params.get_pixel_coords(SCREEN_WEIGHT / 2.0 * 0.4, SCREEN_HEIGHT / 2.0 * 0.85);
        draw_text_ex(
            &format!("Score: {}", self.game_engine.get_score()),
            coords.0, coords.1,
            TextParams {
                font_size: 50,
                font_scale: self.size_params.font_scale,
                color: YELLOW,
                font: Some(self.resources.get_font()),
                ..Default::default()
            },
        );
    }

    fn draw_nick(&self) {
        let coords = self.size_params.get_pixel_coords(-SCREEN_WEIGHT / 2.0 * 0.8, SCREEN_HEIGHT / 2.0 * 0.85);
        draw_text_ex(
            &format!("{}", self.nick_name),
            coords.0, coords.1,
            TextParams {
                font_size: 50,
                font_scale: self.size_params.font_scale,
                color: YELLOW,
                font: Some(self.resources.get_font()),
                ..Default::default()
            },
        );
    }

    fn draw_state(&self) {
        match self.state {
            State::Paused => self.draw_pause(),
            State::ShowNewRecord => self.draw_win(),
            State::ShowGameOver => self.draw_game_over(),
            _ => {},
        }
    }

    fn draw_pause(&self) {
        let (texture, x_to_y) = self.resources.get_label(&Labels::Pause);
        self.draw_centered_label(texture, x_to_y);
    }

    fn draw_win(&self) {
        let (texture, x_to_y) = self.resources.get_label(&Labels::Win);
        self.draw_centered_label(texture, x_to_y);
    }

    fn draw_game_over(&self) {
        let (texture, x_to_y) = self.resources.get_label(&Labels::GameOver);
        self.draw_centered_label(texture, x_to_y);
    }

    fn draw_centered_label(&self, texture: &Texture2D, x_to_y_ratio: f32) {
        let x = SCREEN_WEIGHT * 0.75;
        let y = x / x_to_y_ratio;
        let r = self.size_params.rectangle_transform(
            (0.0, 0.0), 
            (x, y));
        draw_texture_ex(
            &texture, r.x, r.y, WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(r.w, r.h)),
                ..Default::default()
            },
        );
    }

    fn draw_records(&self) {
        for (_, score) in self.best_scores {
            let (in_screen, y) = self.game_engine.get_score_coord(*score);
            if !in_screen {
                continue;
            }
            let (texture, x_to_y) = self.resources.get_label(&Labels::Record);
            let w = SCREEN_WEIGHT * 0.15;
            let h = w / x_to_y;
            let r = self.size_params.rectangle_transform(
                (-SCREEN_WEIGHT / 2.0 + w / 2.0, y), (w, h));
            draw_texture_ex(
                &texture, r.x, r.y, WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(r.w, r.h)),
                    ..Default::default()
                },
            );
        }
    }

    fn update_pause(&mut self) {
        for key in get_keys_pressed() {
            if key == KeyCode::P {
                self.change_state(
                    if self.state == State::Paused { State::Running } else { State::Paused }
                );
                break;
            }
        }
    }

    fn change_state(&mut self, state: State) {
        if self.state == state {
            return;
        }
        self.state_duration = 0.0;
        self.state = state;
    }
}