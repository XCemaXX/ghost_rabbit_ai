
use game_logic::{Difficulty, GameState, MoveDirection, SCREEN_HEIGHT, SCREEN_WEIGHT};

use crate::rand_gen::RandGen;

use super::resources::{Resources, Backgrounds, Labels, Sounds};
use super::square_screen::FixedRatioScreen;
use super::options::Options;

use macroquad::prelude::*;

const GAME_OVER_DURATION_SEC: f64 = 1.3;
const WIN_DURATION_SEC: f64 = 1.0;
const P1000_DURATION_SEC: f64 = 1.5;

#[derive(PartialEq, Eq)]
enum State {
    Running,
    Paused,
    ShowNewRecord,
    ShowGameOver,
    Finished,
}

pub struct GameScreen<'a>
{
    pub game_engine: GameState<RandGen>, 
    size_params: FixedRatioScreen, 
    resources: &'a Resources,
    state: State,
    state_duration: f64,
    nick_name: &'a String,
    best_scores: &'a [usize],
    show_1000: f64,
    buttons: Buttons<'a>,
    is_sounds_on: bool,
}

impl<'a> GameScreen<'a> {
    pub fn new(resources: &'a Resources, options: &'a Options, scores: &'a [usize]) -> Self {
        let gs = GameScreen {
            game_engine: GameState::new(RandGen{}, options.difficulty),
            nick_name: &options.nickname,
            is_sounds_on: options.sounds_on,
            size_params: FixedRatioScreen::new(SCREEN_WEIGHT / SCREEN_HEIGHT),
            resources,
            state: State::Running,
            state_duration: 0.0,
            best_scores: scores,
            show_1000: -1.0,
            buttons: Buttons::new(resources),
        };
        gs.play_sound(&Sounds::Start);
        gs
    }

    pub fn update_size(&mut self) {
        self.size_params = FixedRatioScreen::new(SCREEN_WEIGHT / SCREEN_HEIGHT); 
    }

    pub fn update_game_state_by_input(&mut self) {
        if self.state != State::Running && self.state != State::Paused {
            return;
        }
        self.update_pause();
        self.update_exit();
    }

    pub fn move_player(&mut self, dt: f64, move_direction: MoveDirection) {
        if self.state != State::Running {
            return;
        }
        self.game_engine.move_player_by_x(dt as f32, move_direction);
    }

    pub fn next_step(&mut self, dt: f64) {
        self.state_duration += dt;
        match self.state {
            State::Running => {
                self.update_1000(dt);
                let step_res = self.game_engine.next_step(dt as f32);
                if step_res.hit {
                    self.play_sound(&Sounds::Hit)
                }
                if step_res.game_over {
                    self.play_sound(&Sounds::Death);
                    self.change_state(State::ShowGameOver); 
                }
            },
            State::ShowGameOver => {
                let minimal_score = *self.best_scores.into_iter().next().unwrap();
                if self.state_duration > GAME_OVER_DURATION_SEC {
                    if self.game_engine.difficulty == Difficulty::Practice
                        || self.get_score() < minimal_score {
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

    fn update_1000(&mut self, dt: f64) {
        let score = self.get_score();
        if (score % 1000 < 17) && (score > 17) {
            if self.show_1000 == -1.0 {
                self.play_sound(&Sounds::Thousand);
            }
            self.show_1000 = dt;
        } else if self.show_1000 > 0.0 {
            self.show_1000 += dt;
        }
        if self.show_1000 > P1000_DURATION_SEC {
            self.show_1000 = -1.0;
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
        self.draw_buttons();
        self.draw_floors();
        self.draw_player();
        self.draw_monster();
        self.draw_score();
        self.draw_1000_score();
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
            &format!("Score: {}", self.get_score()),
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

    fn draw_1000_score(&self) {
        if self.state != State::Running || self.show_1000 < 0.0 {
            return;
        }
        let (texture, x_to_y) = self.resources.get_label(&Labels::P1000);
        let x = SCREEN_WEIGHT * 0.75;
        let y = x / x_to_y;
        let r = self.size_params.rectangle_transform(
            (0.0, SCREEN_HEIGHT / 2.0 * 0.75), 
            (x / 2.0, y));
        let frame = (self.state_duration.fract() > 0.5) as i32;
        draw_texture_ex(
            &texture, r.x, r.y, WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(r.w, r.h)),
                source: Some(Rect { x: 175.0 * frame as f32, y: 0.0, w: 175.0, h: 53.0 }),
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
        self.draw_centered_label(&Labels::Pause);
    }

    fn draw_win(&self) {
        self.draw_centered_label(&Labels::Win);
    }

    fn draw_game_over(&self) {
        self.draw_centered_label(&Labels::GameOver);
    }

    fn draw_centered_label(&self, label: &Labels) {
        let (texture, x_to_y) = self.resources.get_label(label);
        let x = SCREEN_WEIGHT * 0.75;
        let y = x / x_to_y;
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
        for score in self.best_scores {
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

    fn draw_buttons(&self) {
        self.buttons.draw_exit_button(&self.size_params);
        self.buttons.draw_pause_button(&self.size_params);
    }

    fn update_pause(&mut self) {
        let mut change_state = self.buttons.is_pause_button_pressed(&self.size_params) 
            || self.state == State::Paused && is_mouse_button_pressed(MouseButton::Left);
        for key in get_keys_pressed() {
            if key == KeyCode::P {
                change_state = true;
                break;
            }
        }
        if change_state {
            self.change_state(
                if self.state == State::Paused { State::Running } else { State::Paused }
            );
        }
    }

    fn update_exit(&mut self) {
        let mut change_state = self.buttons.is_exit_button_pressed(&self.size_params);
        for key in get_keys_down() {
            match key {
                KeyCode::Escape => { change_state = true; }
                _ => { },
            }
        }
        if change_state {
            self.change_state(State::Finished);
        }
    }

    fn change_state(&mut self, state: State) {
        if self.state == state {
            return;
        }
        self.state_duration = 0.0;
        self.state = state;
    }

    fn play_sound(&self, sound: &Sounds) {
        if self.is_sounds_on {
            self.resources.play_sound_once(sound);
        }
    }
}

struct Buttons<'a> {
    pause: &'a Texture2D,
    exit: &'a Texture2D,
    pub w: f32,
    pub h: f32,
}

impl<'a> Buttons<'a> {
    fn new(resources: &'a Resources) -> Self {
        let (pause_button, x_to_y) = resources.get_label(&Labels::PauseButton);
        let (exit, _) = resources.get_label(&Labels::ExitButton);
        let w = SCREEN_WEIGHT * 0.08;
        let h = w / x_to_y;
        Self { pause: pause_button, exit, w, h }
    }

    fn draw_pause_button(&self, size_params: &FixedRatioScreen) {
        let r = self.get_pause_button_rect(size_params);
        Self::draw(r, &self.pause);
    }

    fn draw_exit_button(&self, size_params: &FixedRatioScreen) {
        let r = self.get_exit_button_rect(size_params);
        Self::draw(r, &self.exit);
    }

    fn get_pause_button_rect(&self, size_params: &FixedRatioScreen) -> Rect {
        size_params.rectangle_transform(
            (SCREEN_WEIGHT / 2.0 - self.w / 2.0 * 1.5, -SCREEN_HEIGHT / 2.0 + self.h / 2.0 * 1.5), (self.w, self.h))
    }

    fn get_exit_button_rect(&self, size_params: &FixedRatioScreen) -> Rect {
        size_params.rectangle_transform(
            (-SCREEN_WEIGHT / 2.0 + self.w / 2.0 * 1.5, -SCREEN_HEIGHT / 2.0 + self.h / 2.0 * 1.5), (self.w, self.h))
    }

    fn draw(r: Rect, t: &Texture2D) {
        draw_texture_ex(
            &t, r.x, r.y, WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(r.w, r.h)),
                ..Default::default()
            },
        );
    }

    fn is_pause_button_pressed(&self, size_params: &FixedRatioScreen) -> bool {
        let r = self.get_pause_button_rect(size_params);
        Self::is_cursor_in(&r) && is_mouse_button_pressed(MouseButton::Left)
    }

    fn is_exit_button_pressed(&self, size_params: &FixedRatioScreen) -> bool {
        let r = self.get_exit_button_rect(size_params);
        Self::is_cursor_in(&r) && is_mouse_button_pressed(MouseButton::Left)
    }

    fn is_cursor_in(r: &Rect) -> bool {
        let (x, y) = mouse_position();
        y > r.y && y < r.y + r.h &&
        x > r.x && x < r.x + r.w 
    }
}

pub fn get_move_direction_by_input() -> MoveDirection {
    let (mouse_x, _) = mouse_position();
    let mut move_direction = if is_mouse_button_down(MouseButton::Left) {
        if mouse_x * 2.0 > screen_width() {
            MoveDirection::Right
        } else {
            MoveDirection::Left
        }
    } else {
        MoveDirection::None
    };
    for key in get_keys_down() {
        match key {
            KeyCode::Left | KeyCode::A => { 
                move_direction = if move_direction == MoveDirection::Right {
                    MoveDirection::None
                } else {
                    MoveDirection::Left
                };
            },
            KeyCode::Right | KeyCode::D => { 
                move_direction = if move_direction == MoveDirection::Left {
                    MoveDirection::None
                } else {
                    MoveDirection::Right
                };
            },
            _ => { },
        }
    }
    move_direction
}