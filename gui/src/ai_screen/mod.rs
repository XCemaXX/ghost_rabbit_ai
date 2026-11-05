mod models;
mod panel;
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
use models::{PolicyKey, Selection};
use panel::{Action, Panel, Training};
use widgets::{Align, Ui, VALUE};

const TEXT_SIZE: f32 = SCREEN_WEIGHT / 16.0;
const TRAIN_SEC_PER_FRAME: f64 = 0.012;

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
        let selection = panel.selection(models);
        let (vision, policy) = models.load(selection);
        let mut player = bot::Bot::new(&vision, &policy);
        let mut game_loop = GameScreen::new(resources, &options, &record_table.scores);
        let mut last_update = get_time();
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
                            models.finish_vision_job(vision_job);
                            panel.select_my_vision();
                        }
                        Job::Policy(..) => panel.select_my_policy(),
                    }
                    new_game = true;
                }
            } else {
                // Whole steps, not a longer dt: the bot must decide as often as in training.
                for _ in 0..panel.speed() {
                    let move_direction = player.act(&game_loop.game_engine);
                    game_loop.advance(dt, move_direction);
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
            draw_models(&ui, selection);
            let training = job.as_ref().map(Job::training);
            match panel.draw(&ui, models, training) {
                Action::None => {}
                Action::NewGame => new_game = true,
                Action::TrainVision => job = Some(Job::Vision(models.start_vision_job())),
                Action::TrainPolicy(key) => {
                    job = Some(Job::Policy(models.start_policy_job(key), key));
                }
                // The game may stand on a net that training has just replaced.
                Action::Stop => {
                    stop_job(job.take(), models);
                    new_game = true;
                }
            }
            next_frame().await;
            if new_game {
                break;
            }
        }
    }
}

enum Job {
    Vision(VisionJob),
    Policy(PolicyJob, PolicyKey),
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
            Job::Vision(job) => Training::Vision(job.progress()),
            Job::Policy(job, _) => Training::Policy(job.progress()),
        }
    }
}

// The results already finished stay.
fn stop_job(job: Option<Job>, models: &mut Models) {
    if let Some(Job::Vision(job)) = job {
        models.stop_vision_job(job);
    }
}

// The game stands while training: the frame belongs to the job.
fn train_in_frame(job: &mut Job, models: &mut Models) {
    let start = get_time();
    while !job.is_done() && get_time() - start < TRAIN_SEC_PER_FRAME {
        match job {
            Job::Vision(job) => job.step(),
            Job::Policy(job, key) => {
                if let Some(point) = job.step() {
                    models.add_point(*key, point);
                }
            }
        }
    }
}

// In the game field, between the exit and pause buttons.
fn draw_models(ui: &Ui, s: Selection) {
    let r = Rect::new(
        SCREEN_WEIGHT * 0.14,
        SCREEN_HEIGHT * 0.93,
        SCREEN_WEIGHT * 0.72,
        SCREEN_HEIGHT * 0.06,
    );
    let mark = |mine| if mine { "*" } else { "" };
    let vision = format!("Vision: r{}{}", s.round, mark(s.my_vision));
    let control = format!("Control: g{}{}", s.generation, mark(s.my_policy));
    ui.text(r, &vision, TEXT_SIZE, VALUE, Align::Left);
    ui.text(r, &control, TEXT_SIZE, VALUE, Align::Right);
}
