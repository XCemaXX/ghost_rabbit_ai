mod square_screen;

use macroquad::ui::{hash, root_ui};
use macroquad::prelude::*;
use square_screen::SquareScreen;

use rand_trait::GenRandFloat;
use neural_network::{LayerTopology, Network};

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
    let layers = vec![
        LayerTopology { neurons: 8 },
        LayerTopology { neurons: 15 },
        LayerTopology { neurons: 2 },
    ];
    let mut rng = RandGen{};
    let _ = Network::new(&mut rng, &layers);

    let mut last_update = get_time();
    loop {
        clear_background(LIGHTGRAY);
        let lu = get_time();
        let dt = lu - last_update;
        last_update = lu;

        let size_params = SquareScreen::new(); 
        let (mouse_x, mouse_y) = mouse_position();
        let (mouse_x, mouse_y) = size_params.normalize_coords(mouse_x, mouse_y);
        
        root_ui().window(hash!(), Vec2::new(10., 10.), Vec2::new(280., 120.), |ui| {
            ui.label(None, &format!("fps: {:.3}", 1.0 / dt));
            ui.label(None, &format!("Mouse {:.3} {:.3}", mouse_x, mouse_y));
        });
        draw_screen_border(&size_params);

        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);
        draw_text("HELLO", 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await
    }
}

fn draw_screen_border(size_params: &SquareScreen) {
    draw_rectangle_lines(size_params.offset_x, size_params.offset_y, size_params.width, size_params.width, 2.0, BLACK);
}