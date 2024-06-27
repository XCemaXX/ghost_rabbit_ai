mod square_screen;
mod resources;

use macroquad::ui::{hash, root_ui};
use macroquad::prelude::*;
use square_screen::FixedRatioScreen;

use rand_trait::GenRandFloat;
use neural_network::{LayerTopology, Network};
use game_logic::{GameState, SCREEN_WEIGHT, SCREEN_HEIGHT, Difficulty, Side, MonsterType};
use resources::{Resources, Backgrounds, Labels};

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

    let resources = Resources::load_resources();
    let mut game_state = GameState::new(RandGen{}, Difficulty::Practice);
    let mut paused = false;
    
    let mut last_update = get_time();
    loop {
        clear_background(LIGHTGRAY);
        let lu = get_time();
        let dt = lu - last_update;
        last_update = lu;

        let size_params = FixedRatioScreen::new(SCREEN_WEIGHT / SCREEN_HEIGHT); 
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
        draw_background(&size_params, &resources);
        draw_game(&mut game_state, &size_params, &resources);
        if paused {
            draw_pause(&size_params, &resources);
        }
        draw_border(&size_params);
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

fn draw_pause(size_params: &FixedRatioScreen, resources: &Resources) {
    let (pause_texture, x_to_y) = resources.get_label(&Labels::Pause);
    let x = SCREEN_WEIGHT * 0.75;
    let y = x / x_to_y;
    let p = size_params.rectangle_transform(
        (0.0, 0.0), 
        (x, y));
    draw_texture_ex(
        &pause_texture, p.0, p.1, WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(p.2, p.3)),
            ..Default::default()
        },
    );
}

fn draw_game(game_state: &mut GameState<RandGen>, size_params: &FixedRatioScreen, resources: &Resources) {
    let floor_texture = resources.get_platform_texture(&game_state.difficulty);
    for floor in &game_state.floors {
        let f = size_params.rectangle_transform(
            floor.position.into(), 
            floor.size.into());
        //draw_rectangle(f.0, f.1, f.2, f.3, GRAY);
        draw_texture_ex(
            &floor_texture, f.0, f.1, WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(f.2, f.3)),
                ..Default::default()
            },
        );
    }

    draw_player(game_state, size_params, resources);
    
    if !game_state.monster.is_dead {
        let monster_texture = resources.get_monster_texture(&game_state.monster.guise);
        let monster = size_params.rectangle_transform(
            game_state.monster.position.into(),
            game_state.monster.size.into());
        //draw_rectangle(monster.0, monster.1, monster.2, monster.3, RED);
        draw_texture_ex(
            &monster_texture, monster.0, monster.1, WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(monster.2, monster.3)),
                ..Default::default()
            },
        );
    }
}

fn draw_player(game_state: &mut GameState<RandGen>, size_params: &FixedRatioScreen, resources: &Resources) {
    let player = size_params.circle_transform(
        game_state.player.position.into(), 
        game_state.player.radius);
    //draw_circle(player.0, player.1, player.2, GREEN);

    let pos = size_params.rectangle_transform(
        (game_state.player.position.x, game_state.player.position.y), 
        ((game_state.player.radius * 2.) * 25.0 / 10.0, (game_state.player.radius*2.0) * 46.0 / 10.0));
    let  mut p = resources.get_player(game_state.player.get_stage());
    p.1.dest_size = Some(vec2(player.2 * 50.0 / 10.0, player.2 * 88.0 / 10.0));
    draw_texture_ex(p.0, pos.0, pos.1, WHITE, p.1);
}

fn draw_background(size_params: &FixedRatioScreen, resources: &Resources) {
    //draw_rectangle_lines(size_params.offset_x, size_params.offset_y, size_params.width, size_params.width, 2.0, BLACK);
    let s = size_params.rectangle_transform(
        (0.0, 0.0), 
        (SCREEN_WEIGHT, SCREEN_HEIGHT));
    //draw_rectangle(s.0, s.1, s.2, s.3, BLACK);
    draw_texture_ex(
        &resources.get_background(&Backgrounds::Game), s.0, s.1, WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(s.2, s.3)),
            ..Default::default()
        },
    );
}

fn draw_border(size_params: &FixedRatioScreen) {
    let (r1, r2) = size_params.get_border_rectangles();
    draw_rectangle(r1.0, r1.1, r1.2, r1.3, GRAY);
    draw_rectangle(r2.0, r2.1, r2.2, r2.3, GRAY);
}