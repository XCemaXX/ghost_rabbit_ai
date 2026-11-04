mod models;
mod panel;
mod widgets;

use bot::job::PolicyJob;
use game_logic::{Difficulty, SCREEN_HEIGHT, SCREEN_WEIGHT};
use macroquad::prelude::*;

use crate::game_screen::GameScreen;
use crate::letterbox::Letterbox;
use crate::menus::ScreenType;
use crate::options::Options;
use crate::recrord_table::RecordTables;
use crate::resources::Resources;
use models::{Models, Selection};
use panel::{Action, Panel};
use widgets::{Align, Ui, VALUE};

const TEXT_SIZE: f32 = SCREEN_WEIGHT / 16.0;
const TRAIN_SEC_PER_FRAME: f64 = 0.012;

// The bot plays Unreal, as in training, and starts again after each death.
pub async fn run(
    resources: &Resources,
    record_tables: &RecordTables,
    options: &Options,
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
    let mut models = Models::new();
    let mut panel = Panel::new();
    let mut job: Option<PolicyJob> = None;
    loop {
        let selection = panel.selection(&models);
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
                return ScreenType::MainMenu;
            }
            let mut new_game = false;
            if let Some(running) = &mut job {
                train_in_frame(running, &mut models, selection.round);
                if running.is_done() {
                    job = None;
                    panel.select_my_policy();
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
            let progress = job.as_ref().map(PolicyJob::progress);
            match panel.draw(&ui, &models, progress) {
                Action::None => {}
                Action::NewGame => new_game = true,
                Action::Train => job = Some(models.start_job(selection.round)),
                // The points already finished stay.
                Action::Stop => job = None,
            }
            next_frame().await;
            if new_game {
                break;
            }
        }
    }
}

// The game stands while training: the frame belongs to the job.
fn train_in_frame(job: &mut PolicyJob, models: &mut Models, round: usize) {
    let start = get_time();
    while !job.is_done() && get_time() - start < TRAIN_SEC_PER_FRAME {
        if let Some(point) = job.step() {
            models.add_point(round, point);
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
    let vision = format!("Vision: r{}", s.round);
    let mine = if s.my_policy { "my " } else { "" };
    let control = format!("Control: {mine}g{}", s.generation);
    ui.text(r, &vision, TEXT_SIZE, VALUE, Align::Left);
    ui.text(r, &control, TEXT_SIZE, VALUE, Align::Right);
}
