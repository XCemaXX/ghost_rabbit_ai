mod square_screen;
mod resources;
mod game_screen;
mod menus;

use macroquad::ui::{hash, root_ui};
use macroquad::prelude::*;

use game_logic::{Difficulty, MonsterType};
use resources::Resources;

//use neural_network::{LayerTopology, Network};


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
    HtpMenu,
    AboutMenu,
    OptionsMenu,
}

pub type ScoreArray = [(String, usize); 3];

struct Options {
    pub nickname: String,
    pub music_on: bool,
    pub sounds_on: bool,
    pub difficulty: Difficulty,
}

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
    let mut options = Options {
        nickname: "XCemaXX".into(),
        music_on: true,
        sounds_on: true,
        difficulty: Difficulty::Normal,
    };

    let mut current_screen = ScreenType::OptionsMenu; // TODO: change MainMenu
    loop {
        current_screen= match current_screen {
            ScreenType::Game=> { run_game_loop(&resources, &best_scores, &options).await },
            ScreenType::MainMenu=> { run_main_menu_loop(&resources).await },
            ScreenType::RecordsMenu=> { run_records_menu_loop(&resources, &best_scores).await },
            ScreenType::HtpMenu => { run_htp_menu_loop(&resources).await },
            ScreenType::AboutMenu => { run_about_menu_loop(&resources).await },
            ScreenType::OptionsMenu => { run_options_menu_loop(&resources, &mut options).await },
        };
    }
}

async fn run_records_menu_loop(resources: &Resources, scores: &ScoreArray) -> ScreenType {
    let mut menu = menus::RecordsMenu::new(resources);
    loop {
        if is_mouse_button_pressed(MouseButton::Left) {
            break ScreenType::Game; // TODO: change logic with buttons
        }
        menu.draw_update();
        next_frame().await
    }
}

async fn run_main_menu_loop(resources: &Resources) -> ScreenType {
    let mut menu = menus::MainMenu::new(resources);
    loop {
        if is_mouse_button_pressed(MouseButton::Left) { // TODO: change logic with buttons
            break ScreenType::Game;
        }
        menu.draw_update();
        next_frame().await
    }
}

async fn run_options_menu_loop(resources: &Resources, options: &mut Options) -> ScreenType {
    let mut menu = menus::OptionsMenu::new(resources, options);
    while !menu.draw_update() {
        next_frame().await
    }
    next_frame().await;
    ScreenType::MainMenu
}

async fn run_htp_menu_loop(resources: &Resources) -> ScreenType {
    let mut menu = menus::HowToPlayMenu::new(resources);
    loop {
        if is_mouse_button_pressed(MouseButton::Left) { // TODO: change logic with buttons
            break ScreenType::MainMenu;
        }
        menu.draw_update();
        next_frame().await
    }
}

async fn run_about_menu_loop(resources: &Resources) -> ScreenType {
    let mut menu = menus::AboutMenu::new(resources);
    loop {
        if is_mouse_button_pressed(MouseButton::Left) { // TODO: change logic with buttons
            break ScreenType::MainMenu;
        }
        menu.draw_update();
        next_frame().await
    }
}

async fn run_game_loop(resources: &Resources, scores: &ScoreArray, options: &Options) -> ScreenType {
    let mut last_update = get_time();
    let difficulty = options.difficulty;
    let mut game_loop = game_screen::GameScreen::new(&resources, difficulty, &options.nickname, scores);
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