use game_logic::{BOT_Y, LEFT_X, SCREEN_HEIGHT, SCREEN_WEIGHT};
use macroquad::prelude::*;

use crate::letterbox::Letterbox;

pub const BARS: Color = GRAY;

// Rasterized once; on usual screens text is scaled down, so it stays sharp.
const FONT_SIZE: u16 = 64;
const TEXT_SIZE: f32 = SCREEN_WEIGHT / 16.0;

/// All screens show the same world, y up.
pub fn game_letterbox() -> Letterbox {
    Letterbox::new(Rect::new(LEFT_X, BOT_Y, SCREEN_WEIGHT, SCREEN_HEIGHT), true)
}

/// World rect from its center: game objects store the center.
pub fn centered_rect(center: (f32, f32), size: (f32, f32)) -> Rect {
    Rect::new(
        center.0 - size.0 / 2.0,
        center.1 - size.1 / 2.0,
        size.0,
        size.1,
    )
}

pub fn draw_world_texture(letterbox: &Letterbox, texture: &Texture2D, rect: Rect) {
    let r = letterbox.rect_to_screen(rect);
    draw_texture_ex(
        texture,
        r.x,
        r.y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(r.w, r.h)),
            ..Default::default()
        },
    );
}

pub fn draw_background(letterbox: &Letterbox, texture: &Texture2D) {
    letterbox.clear(BARS, BLACK);
    let f = letterbox.field();
    draw_texture_ex(
        texture,
        f.x,
        f.y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(f.w, f.h)),
            ..Default::default()
        },
    );
}

/// `pos` is the start of the text baseline, in world units.
pub fn draw_world_text(
    letterbox: &Letterbox,
    text: &str,
    pos: (f32, f32),
    color: Color,
    font: &Font,
) {
    let p = letterbox.to_screen(vec2(pos.0, pos.1));
    draw_text_ex(
        text,
        p.x,
        p.y,
        TextParams {
            font_size: FONT_SIZE,
            font_scale: letterbox.font_scale(FONT_SIZE, TEXT_SIZE),
            color,
            font: Some(font),
            ..Default::default()
        },
    );
}
