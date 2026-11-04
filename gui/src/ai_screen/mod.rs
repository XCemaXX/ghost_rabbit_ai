mod panel;
mod widgets;

use game_logic::{Difficulty, SCREEN_HEIGHT, SCREEN_WEIGHT};
use macroquad::prelude::*;

use crate::game_screen::GameScreen;
use crate::letterbox::Letterbox;
use crate::menus::ScreenType;
use crate::options::Options;
use crate::recrord_table::RecordTables;
use crate::resources::Resources;
use panel::Panel;
use widgets::Ui;

// The bot plays Unreal, as in training, and starts again after each death.
pub async fn run(
    resources: &Resources,
    record_tables: &RecordTables,
    options: &Options,
) -> ScreenType {
    let options = Options {
        difficulty: Difficulty::Unreal,
        ..options.clone()
    };
    let vision = bot::from_bytes(
        &bot::vision_topology(),
        include_bytes!("../../../resources/bot/vision_r7.bin"),
    );
    let policy = bot::policy_at_generation(
        include_bytes!("../../../resources/bot/policy_r7.bin"),
        bot::MAX_GENERATION,
    );
    let record_table = record_tables.get_table(&options.difficulty);
    // Two game fields side by side: the game on the left, the panel on the right.
    let mut screen = Letterbox::new(
        Rect::new(0.0, 0.0, 2.0 * SCREEN_WEIGHT, SCREEN_HEIGHT),
        false,
    );
    let mut panel = Panel::new();
    loop {
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
            let move_direction = player.act(&game_loop.game_engine);
            game_loop.advance(dt, move_direction);
            if game_loop.is_game_over() {
                break;
            }
            game_loop.draw();
            panel.draw(&Ui {
                screen: &screen,
                resources,
            });
            next_frame().await
        }
    }
}
