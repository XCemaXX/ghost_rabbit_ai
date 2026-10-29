use game_logic::{
    Difficulty, GameState, LEFT_X, MoveDirection, RIGHT_X, SCREEN_HEIGHT, SCREEN_WEIGHT,
};

use crate::rand_gen::RandGen;

use super::options::Options;
use super::resources::{Backgrounds, Labels, Resources, Sounds};
use crate::draw::{
    BARS, centered_rect, draw_background, draw_world_text, draw_world_texture, game_letterbox,
};
use crate::letterbox::Letterbox;

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

pub struct GameScreen<'a> {
    pub game_engine: GameState<RandGen>,
    resources: &'a Resources,
    state: State,
    state_duration: f64,
    nick_name: &'a String,
    best_scores: &'a [usize],
    show_1000: f64,
    buttons: Buttons<'a>,
    is_sounds_on: bool,
    letterbox: Letterbox,
}

impl<'a> GameScreen<'a> {
    pub fn new(resources: &'a Resources, options: &'a Options, scores: &'a [usize]) -> Self {
        let gs = GameScreen {
            game_engine: GameState::new(RandGen {}, options.difficulty),
            nick_name: &options.nickname,
            is_sounds_on: options.sounds_on,
            resources,
            state: State::Running,
            state_duration: 0.0,
            best_scores: scores,
            show_1000: -1.0,
            buttons: Buttons::new(resources),
            letterbox: game_letterbox(),
        };
        gs.play_sound(&Sounds::Start);
        gs
    }

    /// Call first in each frame: the window can change size at any time.
    pub fn begin_frame(&mut self) {
        self.letterbox.update();
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
            }
            State::ShowGameOver => {
                let minimal_score = *self.best_scores.iter().next().unwrap();
                if self.state_duration > GAME_OVER_DURATION_SEC {
                    if self.game_engine.difficulty == Difficulty::Practice
                        || self.get_score() < minimal_score
                    {
                        self.change_state(State::Finished);
                    } else {
                        self.change_state(State::ShowNewRecord);
                    }
                }
            }
            State::ShowNewRecord if self.state_duration > WIN_DURATION_SEC => {
                self.change_state(State::Finished);
            }
            _ => {}
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
        let letterbox = &self.letterbox;
        draw_background(letterbox, self.resources.get_background(&Backgrounds::Game));
        self.draw_records();
        self.draw_buttons();
        self.draw_floors();
        self.draw_player();
        self.draw_monster();
        self.draw_score();
        self.draw_1000_score();
        self.draw_nick();
        self.draw_state();
        letterbox.draw_bars(BARS);
    }

    fn draw_floors(&self) {
        let letterbox = &self.letterbox;
        let floor_texture = self
            .resources
            .get_platform_texture(&self.game_engine.difficulty);
        for floor in &self.game_engine.floors {
            let r = centered_rect(floor.position.into(), floor.size.into());
            draw_world_texture(letterbox, floor_texture, r);
        }
    }

    fn draw_monster(&self) {
        let letterbox = &self.letterbox;
        let monster = &self.game_engine.monster;
        if !monster.is_dead {
            let monster_texture = self.resources.get_monster_texture(&monster.guise);
            let r = centered_rect(monster.position.into(), monster.size.into());
            draw_world_texture(letterbox, monster_texture, r);
        }
    }

    fn draw_player(&self) {
        let letterbox = &self.letterbox;
        let player = &self.game_engine.player;
        let radius = player.radius;
        // Top-left comes from a taller box than the texture: keeps the old sprite offset.
        let r = letterbox.rect_to_screen(centered_rect(
            player.position.into(),
            ((radius * 2.) * 25.0 / 10.0, (radius * 2.0) * 46.0 / 10.0),
        ));
        let (texture, mut texture_params) = self.resources.get_player(player.get_stage());
        texture_params.dest_size = Some(vec2(
            letterbox.len_to_screen(radius * 50.0 / 10.0),
            letterbox.len_to_screen(radius * 88.0 / 10.0),
        ));
        draw_texture_ex(texture, r.x, r.y, WHITE, texture_params);
    }

    fn draw_score(&self) {
        let letterbox = &self.letterbox;
        draw_world_text(
            letterbox,
            &format!("Score: {}", self.get_score()),
            (SCREEN_WEIGHT / 2.0 * 0.4, SCREEN_HEIGHT / 2.0 * 0.85),
            YELLOW,
            self.resources.get_font(),
        );
    }

    fn draw_1000_score(&self) {
        let letterbox = &self.letterbox;
        if self.state != State::Running || self.show_1000 < 0.0 {
            return;
        }
        let (texture, x_to_y) = self.resources.get_label(&Labels::P1000);
        let x = SCREEN_WEIGHT * 0.75;
        let y = x / x_to_y;
        let r = letterbox.rect_to_screen(centered_rect(
            (0.0, SCREEN_HEIGHT / 2.0 * 0.75),
            (x / 2.0, y),
        ));
        let frame = (self.state_duration.fract() > 0.5) as i32;
        draw_texture_ex(
            texture,
            r.x,
            r.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(r.w, r.h)),
                source: Some(Rect {
                    x: 175.0 * frame as f32,
                    y: 0.0,
                    w: 175.0,
                    h: 53.0,
                }),
                ..Default::default()
            },
        );
    }

    fn draw_nick(&self) {
        let letterbox = &self.letterbox;
        draw_world_text(
            letterbox,
            self.nick_name,
            (-SCREEN_WEIGHT / 2.0 * 0.8, SCREEN_HEIGHT / 2.0 * 0.85),
            YELLOW,
            self.resources.get_font(),
        );
    }

    fn draw_state(&self) {
        let letterbox = &self.letterbox;
        let label = match self.state {
            State::Paused => Labels::Pause,
            State::ShowNewRecord => Labels::Win,
            State::ShowGameOver => Labels::GameOver,
            _ => return,
        };
        let (texture, x_to_y) = self.resources.get_label(&label);
        let x = SCREEN_WEIGHT * 0.75;
        let y = x / x_to_y;
        draw_world_texture(letterbox, texture, centered_rect((0.0, 0.0), (x, y)));
    }

    fn draw_records(&self) {
        let letterbox = &self.letterbox;
        for score in self.best_scores {
            let (in_screen, y) = self.game_engine.get_score_coord(*score);
            if !in_screen {
                continue;
            }
            let (texture, x_to_y) = self.resources.get_label(&Labels::Record);
            let w = SCREEN_WEIGHT * 0.15;
            let h = w / x_to_y;
            let r = centered_rect((-SCREEN_WEIGHT / 2.0 + w / 2.0, y), (w, h));
            draw_world_texture(letterbox, texture, r);
        }
    }

    fn draw_buttons(&self) {
        let letterbox = &self.letterbox;
        draw_world_texture(letterbox, self.buttons.exit, self.buttons.exit_rect());
        draw_world_texture(letterbox, self.buttons.pause, self.buttons.pause_rect());
    }

    fn update_pause(&mut self) {
        let letterbox = &self.letterbox;
        let click = clicked_at(letterbox);
        let mut change_state = click.is_some_and(|p| self.buttons.pause_rect().contains(p))
            || self.state == State::Paused && click.is_some();
        for key in get_keys_pressed() {
            if key == KeyCode::P {
                change_state = true;
                break;
            }
        }
        if change_state {
            self.change_state(if self.state == State::Paused {
                State::Running
            } else {
                State::Paused
            });
        }
    }

    fn update_exit(&mut self) {
        let letterbox = &self.letterbox;
        let mut change_state =
            clicked_at(letterbox).is_some_and(|p| self.buttons.exit_rect().contains(p));
        for key in get_keys_down() {
            if key == KeyCode::Escape {
                change_state = true;
            }
        }
        if change_state {
            self.change_state(State::Finished);
        }
    }

    pub fn move_direction_by_input(&self) -> MoveDirection {
        let letterbox = &self.letterbox;
        // A press on a bar moves the player too: the side bars are natural thumb zones.
        let mut move_direction = if !is_mouse_button_down(MouseButton::Left) {
            MoveDirection::None
        } else if letterbox.mouse_world_unclamped().x > (LEFT_X + RIGHT_X) / 2.0 {
            MoveDirection::Right
        } else {
            MoveDirection::Left
        };
        for key in get_keys_down() {
            match key {
                KeyCode::Left | KeyCode::A => {
                    move_direction = if move_direction == MoveDirection::Right {
                        MoveDirection::None
                    } else {
                        MoveDirection::Left
                    };
                }
                KeyCode::Right | KeyCode::D => {
                    move_direction = if move_direction == MoveDirection::Left {
                        MoveDirection::None
                    } else {
                        MoveDirection::Right
                    };
                }
                _ => {}
            }
        }
        move_direction
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
        Self {
            pause: pause_button,
            exit,
            w,
            h,
        }
    }

    fn pause_rect(&self) -> Rect {
        centered_rect(
            (
                SCREEN_WEIGHT / 2.0 - self.w / 2.0 * 1.5,
                -SCREEN_HEIGHT / 2.0 + self.h / 2.0 * 1.5,
            ),
            (self.w, self.h),
        )
    }

    fn exit_rect(&self) -> Rect {
        centered_rect(
            (
                -SCREEN_WEIGHT / 2.0 + self.w / 2.0 * 1.5,
                -SCREEN_HEIGHT / 2.0 + self.h / 2.0 * 1.5,
            ),
            (self.w, self.h),
        )
    }
}

/// Click point in world units; a click on the bars is ignored.
fn clicked_at(letterbox: &Letterbox) -> Option<Vec2> {
    if is_mouse_button_pressed(MouseButton::Left) {
        letterbox.mouse_world()
    } else {
        None
    }
}
