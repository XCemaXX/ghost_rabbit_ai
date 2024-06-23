mod square_screen;

use macroquad::ui::{hash, root_ui};
use macroquad::prelude::*;
use square_screen::SquareScreen;

use rand_trait::GenRandFloat;
use neural_network::{LayerTopology, Network};
use game_logic::{GameState, SCREEN_WEIGHT, SCREEN_HEIGHT, Difficulty, Side};

fn window_conf() -> Conf {
    Conf {
        window_title: "GhostRabbitAi".to_owned(),
        ..Default::default()
    }
}

struct RandGen {}

impl GenRandFloat for RandGen {
    fn gen_range(&mut self, range: std::ops::RangeInclusive<f32>) -> f32 {
        let low = *range.start();
        let high = *range.end();
        rand::gen_range(low, high)
    }
}

#[macroquad::main(window_conf)]
async fn main() {

    //rand::srand(  );
    let layers = vec![
        LayerTopology { neurons: 8 },
        LayerTopology { neurons: 15 },
        LayerTopology { neurons: 2 },
    ];
    let mut rng = RandGen{};
    let _ = Network::new(&mut rng, &layers);

    let mut game_state = GameState::new(RandGen{}, Difficulty::Practice);
    let mut paused = false;
    
    let mut last_update = get_time();
    loop {
        clear_background(LIGHTGRAY);
        let lu = get_time();
        let dt = lu - last_update;
        last_update = lu;

        let size_params = SquareScreen::new(); 
        let (mouse_x, mouse_y) = mouse_position();
        let (mouse_x, mouse_y) = size_params.normalize_coords(mouse_x, mouse_y);

        paused = is_pause(paused);
        if !paused {
            update_game_state_by_input(&mut game_state, dt as f32);
            let _game_over = game_state.next_step(dt as f32);
        }
        
        root_ui().window(hash!(), Vec2::new(10., 10.), Vec2::new(280., 120.), |ui| {
            ui.label(None, &format!("fps: {:.3}", 1.0 / dt));
            ui.label(None, &format!("Mouse {:.3} {:.3}", mouse_x, mouse_y));
            ui.label(None, &format!("speed: {:.3}", game_state.player.speed.y));
        });
        draw_screen_border(&size_params);
        draw_game(&mut game_state, &size_params);
        next_frame().await
    }
}

fn is_pause(paused: bool) -> bool {
    for key in get_keys_pressed() {
        if key == KeyCode::P {
            return !paused;
        }
    }
    return paused;
}

fn update_game_state_by_input(game_state: &mut GameState<RandGen>, dt: f32) {
    for key in get_keys_down() {
        match key {
            KeyCode::Left => { game_state.move_player_by_x(dt, Side::Left); },
            KeyCode::Right => { game_state.move_player_by_x(dt, Side::Right); },
            _ => { },
        }
    }
}

fn draw_game(game_state: &mut GameState<RandGen>, size_params: &SquareScreen) {
    for floor in &game_state.floors {
        let f = size_params.rectangle_transform(
            floor.position.into(), 
            floor.size.into());
        draw_rectangle(f.0, f.1, f.2, f.3, GRAY);
    }

    let player = size_params.circle_transform(
        game_state.player.position.into(), 
        game_state.player.radius);
    draw_circle(player.0, player.1, player.2, GREEN);
    
    if !game_state.monster.is_dead {
        let monster = size_params.rectangle_transform(
            game_state.monster.position.into(),
            game_state.monster.size.into());
        draw_rectangle(monster.0, monster.1, monster.2, monster.3, RED);
    }
}

fn draw_screen_border(size_params: &SquareScreen) {
    //draw_rectangle_lines(size_params.offset_x, size_params.offset_y, size_params.width, size_params.width, 2.0, BLACK);
    let s = size_params.rectangle_transform(
        (0.0, 0.0), 
        (SCREEN_WEIGHT, SCREEN_HEIGHT));
    draw_rectangle(s.0, s.1, s.2, s.3, BLACK);
}