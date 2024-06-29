mod square_screen;
mod resources;
mod game_screen;
mod main_menu;
mod records_menu;

use macroquad::ui::{hash, root_ui};
use macroquad::prelude::*;

use game_logic::{Difficulty, MonsterType};

//use neural_network::{LayerTopology, Network};

use resources::Resources;

fn window_conf() -> Conf {
    Conf {
        window_title: "GhostRabbitAi".to_owned(),
        ..Default::default()
    }
}

enum ScreenType {
    Game,
    MainMenu,
    RecordsMenu,
}

pub type ScoreArray = [(String, usize); 3];

#[macroquad::main(window_conf)]
async fn main() {

    //rand::srand(  );
    /*let layers = vec![
        LayerTopology { neurons: 8 },
        LayerTopology { neurons: 15 },
        LayerTopology { neurons: 2 },
    ];
    let mut rng = RandGen{};
    let _ = Network::new(&mut rng, &layers);*/
    let best_scores: ScoreArray = [("a".into(), 75), ("b".into(), 300), ("c".into(), 450)];
    let resources = Resources::load_resources();

    let mut current_screen = ScreenType::Game; // TODO: change MainMenu
    loop {
        match current_screen {
            ScreenType::Game => { current_screen = run_game_loop(&resources, &best_scores).await },
            ScreenType::MainMenu => { current_screen = run_main_menu_loop(&resources).await },
            ScreenType::RecordsMenu => { current_screen = run_records_menu_loop(&resources, &best_scores).await },
        }
    }
}

async fn run_records_menu_loop(resources: &Resources, scores: &ScoreArray) -> ScreenType {
    let mut menu_loop = records_menu::RecordsMenu::new(resources);
    loop {
        menu_loop.update_size();
        if is_mouse_button_pressed(MouseButton::Left) {
            break ScreenType::Game; // TODO: change logic with buttons
        }
        menu_loop.draw();
        next_frame().await
    }
}

async fn run_main_menu_loop(resources: &Resources) -> ScreenType {
    let mut menu_loop = main_menu::MainMenu::new(resources);
    loop {
        menu_loop.update_size();
        if is_mouse_button_pressed(MouseButton::Left) { // TODO: change logic with buttons
            break ScreenType::Game;
        }
        menu_loop.draw();
        next_frame().await
    }
}

async fn run_game_loop(resources: &Resources, scores: &ScoreArray) -> ScreenType {
    let mut last_update = get_time();
    let difficulty = Difficulty::Practice; //Difficulty::Medium Practice
    let mut game_loop = game_screen::GameScreen::new(&resources, difficulty, "XCemaXX".into(), scores);
    loop {
        //clear_background(LIGHTGRAY);
        let lu = get_time();
        let dt = lu - last_update;
        last_update = lu;

        game_loop.update_size();
        let (mouse_x, mouse_y) = mouse_position();

        game_loop.update_game_state_by_input(dt);
        game_loop.next_step(dt);
        if game_loop.is_game_over() {
            if difficulty == Difficulty::Practice ||
            game_loop.get_score() < scores.first().unwrap().1 {
                break ScreenType::MainMenu;
            }
            break ScreenType::RecordsMenu;
        }
        
        root_ui().window(hash!(), Vec2::new(10., 10.), Vec2::new(280., 120.), |ui| {
            ui.label(None, &format!("fps: {:.3}", 1.0 / dt));
            ui.label(None, &format!("Mouse {:.3} {:.3}", mouse_x, mouse_y));
            //ui.label(None, &format!("speed: {:.3}", game_state.player.speed.y));
        });
        game_loop.draw();
        next_frame().await
    }
}