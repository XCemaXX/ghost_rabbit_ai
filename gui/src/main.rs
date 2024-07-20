mod square_screen;
mod resources;
mod game_screen;
mod menus;
mod recrord_table;
mod rand_gen;

use game_screen::MoveDirection;
use macroquad::ui::{hash, root_ui};
use macroquad::prelude::*;

use game_logic::{Difficulty, GameState, MonsterType};
use rand_gen::RandGen;
use resources::Resources;
use menus::ScreenType;
use recrord_table::{RecordTables, get_default_records};

use neural_network::{LayerTopology, Network};

fn window_conf() -> Conf {
    Conf {
        window_title: "GhostRabbitAi".to_owned(),
        ..Default::default()
    }
}

struct Options {
    pub nickname: String,
    pub music_on: bool,
    pub sounds_on: bool,
    pub difficulty: Difficulty,
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut record_tables = get_default_records();
    let resources = Resources::load_resources();
    let mut options = Options {
        nickname: "XCemaXX".into(),
        music_on: true,
        sounds_on: true,
        difficulty: Difficulty::Normal,
    };

    let mut current_screen = ScreenType::MainMenu;
    loop {
        current_screen= match current_screen {
            ScreenType::Game => { run_game_loop(&resources, &mut record_tables, &options).await },
            ScreenType::MainMenu => { run_main_menu_loop(&resources).await },
            ScreenType::RecordsMenu => { run_records_menu_loop(&resources, &mut record_tables).await },
            ScreenType::HtpMenu => { run_htp_menu_loop(&resources).await },
            ScreenType::AboutMenu => { run_about_menu_loop(&resources).await },
            ScreenType::OptionsMenu => { run_options_menu_loop(&resources, &mut options).await },
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
    let record_table = record_tables.get_mut(&difficulty).unwrap();
    let mut game_loop = game_screen::GameScreen::new(&resources, difficulty, &options.nickname, &record_table.scores);
    let score = loop {
        //clear_background(LIGHTGRAY);
        let lu = get_time();
        let dt = lu - last_update;
        last_update = lu;

        game_loop.update_size();
        let (mouse_x, mouse_y) = mouse_position();

        game_loop.update_game_state_by_input();
        let move_direction = game_screen::get_move_direction_by_input();
        game_loop.move_player(dt, move_direction);
        game_loop.next_step(dt);
        if game_loop.is_game_over() {
            break game_loop.get_score();
        }
        
        root_ui().window(hash!(), Vec2::new(10., 10.), Vec2::new(280., 120.), |ui| {
            ui.label(None, &format!("fps: {:.3}", 1.0 / dt));
            ui.label(None, &format!("Mouse {:.3} {:.3}", mouse_x, mouse_y));
        });
        game_loop.draw();
        next_frame().await
    };
    if difficulty == Difficulty::Practice || &score < &record_table.scores[0] {
        return ScreenType::MainMenu
    }
    record_table.insert(options.nickname.clone(), score);
    ScreenType::RecordsMenu
}

async fn run_ai_game_loop(resources: &Resources, record_tables: &mut RecordTables, options: &Options) -> ScreenType {
    let network = learn_ai();
    let mut last_update = get_time();
    let difficulty = Difficulty::Unreal;
    let record_table = record_tables.get_mut(&difficulty).unwrap();
    let mut game_loop = game_screen::GameScreen::new(&resources, difficulty, &options.nickname, &record_table.scores);
    loop {
        let lu = get_time();
        let dt = lu - last_update;
        last_update = lu;

        game_loop.update_size();
        game_loop.update_game_state_by_input();

        let inputs_ai = generate_inputs_for_ai(&game_loop.game_engine);
        let move_direction = get_next_move_by_ai(&network, inputs_ai);

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

fn learn_ai() -> Network {
    // TODO!
    let layers = vec![
        LayerTopology { neurons: 4 + 5 + 24 },
        LayerTopology { neurons: 10 }, // will try 8-18
        LayerTopology { neurons: 2 },
    ];
    let mut rng = rand_gen::RandGen{};
    let network = Network::new(&mut rng, &layers);
    
    network
}

fn generate_inputs_for_ai(game: &GameState<RandGen>) -> Vec<f32> {
    let mut inputs = Vec::new();
    inputs.push(game.player.position.x);
    inputs.push(game.player.position.y);
    inputs.push(game.player.speed.x);
    inputs.push(game.player.speed.y);
    inputs.push(game.monster.position.x);
    inputs.push(game.monster.position.y);
    inputs.push(game.monster.speed.x);
    inputs.push(game.monster.speed.y);
    inputs.push(game.monster.is_dead as i32 as f32);
    let mut floor_positions: Vec<(f32, f32)> = game.floors.iter().map(|f| {
        (f.position.x, f.position.y)
    }).collect();
    floor_positions.sort_by(|a, b| { a.0.partial_cmp(&b.0).unwrap() });
    for (x, y) in floor_positions {
        inputs.push(x);
        inputs.push(y);
    }
    inputs
}

fn get_next_move_by_ai(network: &Network, inputs: Vec<f32>) -> MoveDirection {
    let response = network.propagate(inputs);
    if response[0] < 0.5 && response[1] < 0.5 {
        MoveDirection::None
    } else if response[0] > 0.5 {
        MoveDirection::Left
    } else {
        MoveDirection::Right
    }
}