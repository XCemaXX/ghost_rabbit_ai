mod square_screen;
mod resources;
mod game_screen;
mod menus;
mod recrord_table;
mod rand_gen;
mod learn_ai;
mod options;

use macroquad::prelude::*;
use quad_snd::Playback;

use game_logic::{Difficulty, MonsterType};
use resources::Resources;
use menus::ScreenType;
use recrord_table::RecordTables;
use options::Options;

use learn_ai::{generate_inputs_for_ai, learn_ai, get_next_move_by_ai};

fn window_conf() -> Conf {
    Conf {
        window_title: "GhostRabbitAi".to_owned(),
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut record_tables = RecordTables::new();
    let resources = Resources::load_resources();
    let mut options = Options::new();
    let mut background_music: Option<Playback> = None;
    let mut current_screen = ScreenType::MainMenu;
    loop {
        current_screen= match current_screen {
            ScreenType::Game => { 
                if options.music_on && background_music.is_none() {
                    background_music = Some(resources.play_background());
                }
                run_game_loop(&resources, &mut record_tables, &options).await 
            },
            ScreenType::MainMenu => { run_main_menu_loop(&resources).await },
            ScreenType::RecordsMenu => { run_records_menu_loop(&resources, &mut record_tables).await },
            ScreenType::HtpMenu => { run_htp_menu_loop(&resources).await },
            ScreenType::AboutMenu => { run_about_menu_loop(&resources).await },
            ScreenType::OptionsMenu => { 
                let r = run_options_menu_loop(&resources, &mut options).await;
                if !options.music_on && background_music.is_some() {
                    resources.stop_background(background_music.take().unwrap())
                } else if options.music_on && background_music.is_none() {
                    background_music = Some(resources.play_background());
                };
                r
            },
            ScreenType::GameAi => { run_ai_game_loop(&resources, &mut record_tables, &options).await }
        };
    }
}

async fn run_records_menu_loop(resources: &Resources, record_tables: &mut RecordTables) -> ScreenType {
    let mut menu = menus::RecordsMenu::new(resources, record_tables);
    while !menu.draw_update() {
        next_frame().await;
    }
    next_frame().await;
    ScreenType::MainMenu
}

async fn run_main_menu_loop(resources: &Resources) -> ScreenType {
    let mut menu = menus::MainMenu::new(resources);
    let next_state = loop {
        let next_state = menu.draw_update();
        if next_state != ScreenType::MainMenu {
            break next_state;
        }
        next_frame().await;
    };
    next_frame().await;
    next_state
}

async fn run_options_menu_loop(resources: &Resources, options: &mut Options) -> ScreenType {
    let mut menu = menus::OptionsMenu::new(resources, options);
    while !menu.draw_update() {
        next_frame().await;
    }
    options.save();
    next_frame().await;
    ScreenType::MainMenu
}

async fn run_htp_menu_loop(resources: &Resources) -> ScreenType {
    let mut menu = menus::HowToPlayMenu::new(resources);
    while !menu.draw_update() {
        next_frame().await;
    }
    next_frame().await;
    ScreenType::MainMenu
}

async fn run_about_menu_loop(resources: &Resources) -> ScreenType {
    let mut menu = menus::AboutMenu::new(resources);
    while !menu.draw_update() {
        next_frame().await;
    }
    next_frame().await;
    ScreenType::MainMenu
}

async fn run_game_loop(resources: &Resources, record_tables: &mut RecordTables, options: &Options) -> ScreenType {
    let mut last_update = get_time();
    let difficulty = options.difficulty;
    let record_table = record_tables.get_table(&difficulty);
    let mut game_loop = game_screen::GameScreen::new(&resources, options, &record_table.scores);
    let score = loop {
        //clear_background(LIGHTGRAY);
        let lu = get_time();
        let dt = lu - last_update;
        last_update = lu;

        game_loop.update_size();

        game_loop.update_game_state_by_input();
        let move_direction = game_screen::get_move_direction_by_input();
        game_loop.move_player(dt, move_direction);
        game_loop.next_step(dt);
        if game_loop.is_game_over() {
            break game_loop.get_score();
        }
        game_loop.draw();
        next_frame().await
    };
    if difficulty == Difficulty::Practice || &score < &record_table.scores[0] {
        return ScreenType::MainMenu
    }
    record_tables.insert(&difficulty, options.nickname.clone(), score);
    ScreenType::RecordsMenu
}

async fn run_ai_game_loop(resources: &Resources, record_tables: &mut RecordTables, options: &Options) -> ScreenType {
    let difficulty = Difficulty::Unreal;
    let ai_player = learn_ai(difficulty);
    let mut last_update = get_time();
    let record_table = record_tables.get_table(&difficulty);
    let mut game_loop = game_screen::GameScreen::new(&resources, options, &record_table.scores);
    loop {
        let lu = get_time();
        let dt = lu - last_update;
        last_update = lu;

        game_loop.update_size();
        game_loop.update_game_state_by_input();

        let inputs_ai = generate_inputs_for_ai(&game_loop.game_engine);
        let move_direction = get_next_move_by_ai(&ai_player, inputs_ai);

        game_loop.move_player(dt, move_direction);
        game_loop.next_step(dt);
        if game_loop.is_game_over() {
            break;
        }
        game_loop.draw();
        next_frame().await
    };
    ScreenType::MainMenu
}