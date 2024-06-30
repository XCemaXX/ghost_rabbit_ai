

use macroquad::prelude::*;
use super::FixedRatioScreen;

pub struct CheckBox<'a> {
    texture_on: &'a Texture2D,
    texture_off: &'a Texture2D,
    width: f32,
    x_to_y_ratio: f32,
    state: &'a mut bool,
    pos: (f32, f32),
}

impl CheckBox<'_> {
    pub fn new<'a>(texture_on: &'a Texture2D, texture_off: &'a Texture2D, 
    width: f32, x_to_y_ratio: f32, state: &'a mut bool, pos: (f32, f32)) -> CheckBox<'a> 
    {
        CheckBox {
            texture_on,
            texture_off,
            width,
            x_to_y_ratio,
            state,
            pos,
        }
    }

    pub fn draw(&mut self, size_params: &FixedRatioScreen, ) {
        let h = self.width / self.x_to_y_ratio;
        let r = size_params.rectangle_transform(self.pos, (self.width, h));

        if is_mouse_button_pressed(MouseButton::Left) && is_cursor_in(&r) {
            *self.state = !(*self.state);
        }
        
        let texture = if *self.state {
            self.texture_on
        } else {
            self.texture_off
        };
        draw_texture_ex(
            texture, r.x, r.y, WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(r.w, r.h)),
                ..Default::default()
            },
        );
    }
}

pub struct Button3Way<'a> {
    textures: [(&'a Texture2D, f32); 3],
    width: f32,
    state: usize,
    pos: (f32, f32),
}

impl Button3Way<'_> {
    pub fn new<'a>(textures: [(&'a Texture2D, f32); 3], 
    width: f32, current_state: usize, pos: (f32, f32)) -> Button3Way<'a> 
    {
        Button3Way {
            textures,
            width,
            state: current_state,
            pos,
        }
    }

    pub fn draw(&mut self, size_params: &FixedRatioScreen) -> usize {
        let h = self.width / self.textures[self.state].1;
        let r = size_params.rectangle_transform(self.pos, (self.width, h));

        if is_mouse_button_pressed(MouseButton::Left) && is_cursor_in(&r) {
            self.state = (self.state + 1) % 3;
        }
        
        let h = self.width / self.textures[self.state].1;
        let r = size_params.rectangle_transform(self.pos, (self.width, h));
        draw_texture_ex(
            self.textures[self.state].0, r.x, r.y, WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(r.w, r.h)),
                ..Default::default()
            },
        );
        self.state
    }
}

pub struct Button<'a> {
    texture: &'a Texture2D,
    texture_hovered: &'a Texture2D,
    width: f32,
    x_to_y_ratio: f32,
    pos: (f32, f32),
}

impl Button<'_> {
    pub fn new<'a>(texture: &'a Texture2D, texture_hovered: &'a Texture2D, 
    width: f32, x_to_y_ratio: f32, pos: (f32, f32)) -> Button<'a> 
    {
        Button {
            texture,
            texture_hovered,
            width,
            x_to_y_ratio,
            pos,
        }
    }

    pub fn draw(&mut self, size_params: &FixedRatioScreen) -> bool {
        let h = self.width / self.x_to_y_ratio;
        let r = size_params.rectangle_transform(self.pos, (self.width, h));

        let texture = if is_cursor_in(&r) {
            if is_mouse_button_pressed(MouseButton::Left) {
                return true;
            }
            self.texture_hovered
        } else {
            self.texture
        };
        draw_texture_ex(
            texture, r.x, r.y, WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(r.w, r.h)),
                ..Default::default()
            },
        );
        false
    }
}

fn is_cursor_in(r: &Rect) -> bool {
    let (x, y) = mouse_position();
    y > r.y && y < r.y + r.h &&
    x > r.x && x < r.x + r.w 
}