mod models;
mod my_policy;
mod panel;
mod storage;
mod widgets;

use bot::policy_job::PolicyJob;
use bot::vision_job::VisionJob;
use game_logic::{Difficulty, SCREEN_HEIGHT, SCREEN_WEIGHT};
use macroquad::prelude::*;

use crate::game_screen::GameScreen;
use crate::letterbox::Letterbox;
use crate::menus::ScreenType;
use crate::options::Options;
use crate::recrord_table::RecordTables;
use crate::resources::Resources;
pub use models::Models;
use panel::{Action, Panel, Training};
use storage::PolicyKey;
use widgets::Ui;

const TRAIN_SEC_PER_FRAME: f64 = 0.012;
// After a long frame the game does not catch up beyond this.
const MAX_CATCH_UP_SEC: f64 = 1.0;

// The bot plays Unreal, as in training, and starts again after each death.
pub async fn run(
    resources: &Resources,
    record_tables: &RecordTables,
    options: &Options,
    models: &mut Models,
) -> ScreenType {
    let options = Options {
        nickname: "AiBot".to_owned(),
        difficulty: Difficulty::Unreal,
        // The bot dies and hits monsters too often, and even more so at high speed.
        sounds_on: false,
        ..options.clone()
    };
    let record_table = record_tables.get_table(&options.difficulty);
    // Two game fields side by side: the game on the left, the panel on the right.
    let mut screen = Letterbox::new(
        Rect::new(0.0, 0.0, 2.0 * SCREEN_WEIGHT, SCREEN_HEIGHT),
        false,
    );
    let mut panel = Panel::new();
    let mut job: Option<Job> = None;
    loop {
        let (vision, policy) = models.nets(panel.selection(models));
        let mut player = bot::Bot::new(&vision, &policy);
        let mut game_loop =
            GameScreen::new(resources, &options, &record_table.scores).without_new_record();
        let mut last_update = get_time();
        let mut game_time = 0.0;
        loop {
            let lu = get_time();
            let dt = lu - last_update;
            last_update = lu;
            screen.update();
            let field = screen.field();
            game_loop.begin_frame_in(Rect::new(field.x, field.y, field.w / 2.0, field.h));

            game_loop.update_game_state_by_input();
            // Here the game ends only by the exit button.
            if game_loop.is_game_over() {
                stop_job(job, models);
                return ScreenType::MainMenu;
            }
            let mut new_game = false;
            if let Some(running) = &mut job {
                train_in_frame(running, models);
                if running.is_done() {
                    match job.take().unwrap() {
                        Job::Vision(vision_job) => {
                            models.finish_vision_job(*vision_job);
                            panel.select_my_vision();
                        }
                        Job::Policy(..) => panel.select_my_policy(),
                    }
                    new_game = true;
                }
            } else {
                // Steps of training time: at any screen rate the bot decides as often as in training.
                game_loop.pause_after_long_frame(dt);
                let step_sec = bot::train::DT as f64;
                game_time = (game_time + dt * panel.speed() as f64).min(MAX_CATCH_UP_SEC);
                while game_time >= step_sec {
                    let move_direction = player.act(&game_loop.game_engine);
                    game_loop.step(step_sec, move_direction);
                    game_time -= step_sec;
                }
                if game_loop.is_game_over() {
                    break;
                }
            }
            game_loop.draw();
            let ui = Ui {
                screen: &screen,
                resources,
            };
            let training = job.as_ref().map(Job::training);
            let action = panel.draw(&ui, models, training);
            // Any action needs a new game: the net may have changed, and on the game over screen
            // the exit button does not work.
            new_game |= action != Action::None;
            match action {
                Action::None | Action::NewGame => {}
                Action::TrainVision => {
                    job = Some(Job::Vision(Box::new(models.start_vision_job())));
                }
                Action::TrainPolicy(key) => {
                    job = Some(Job::Policy(Box::new(models.start_policy_job(key)), key));
                }
                Action::ResetVision => models.reset_vision(),
                Action::ResetPolicy(key) => models.reset_policy(key),
                Action::Stop => stop_job(job.take(), models),
            }
            next_frame().await;
            if new_game {
                break;
            }
        }
    }
}

enum Job {
    Vision(Box<VisionJob>),
    Policy(Box<PolicyJob>, PolicyKey),
}

impl Job {
    fn is_done(&self) -> bool {
        match self {
            Job::Vision(job) => job.is_done(),
            Job::Policy(job, _) => job.is_done(),
        }
    }

    fn training(&self) -> Training {
        match self {
            Job::Vision(job) => Training::Vision {
                progress: job.progress(),
                rebuilding: job.is_rebuilding(),
            },
            Job::Policy(job, _) => Training::Policy(job.progress()),
        }
    }
}

// The results already finished stay.
fn stop_job(job: Option<Job>, models: &mut Models) {
    if let Some(Job::Vision(job)) = job {
        models.stop_vision_job(*job);
    }
}

// The game stands while training: the frame belongs to the job.
fn train_in_frame(job: &mut Job, models: &mut Models) {
    let start = get_time();
    while !job.is_done() && get_time() - start < TRAIN_SEC_PER_FRAME {
        match job {
            Job::Vision(job) => job.step(),
            Job::Policy(job, key) => {
                if let Some((point, resume)) = job.step() {
                    models.add_point(*key, point, resume);
                }
            }
        }
    }
}
