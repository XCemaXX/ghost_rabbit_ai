use macroquad::prelude::*;

use crate::draw::FONT_SIZE;
use crate::letterbox::Letterbox;
use crate::resources::Resources;

pub const TEXT: Color = WHITE;
pub const VALUE: Color = YELLOW;
const MUTED: Color = Color::new(0.75, 0.75, 0.75, 1.0);
pub const LINE: Color = Color::new(1.0, 1.0, 1.0, 0.25);
const BAR: Color = Color::new(0.0, 0.0, 0.0, 0.5);
const FRAME_FILL: Color = Color::new(0.0, 0.0, 0.0, 0.35);
const FRAME_HOVER: Color = Color::new(1.0, 1.0, 1.0, 0.2);
const FRAME_LINE: Color = Color::new(1.0, 1.0, 1.0, 0.7);
const DISABLED_LINE: Color = Color::new(0.5, 0.5, 0.5, 1.0);

/// Widgets draw and read the mouse in one call. All rects are in world units.
pub struct Ui<'a> {
    pub screen: &'a Letterbox,
    pub resources: &'a Resources,
}

#[derive(Clone, Copy)]
pub enum Align {
    Left,
    Center,
    Right,
}

impl Ui<'_> {
    pub fn fill(&self, r: Rect, color: Color) {
        let r = self.screen.rect_to_screen(r);
        draw_rectangle(r.x, r.y, r.w, r.h, color);
    }

    /// `source` is in texture pixels.
    pub fn image(&self, r: Rect, texture: &Texture2D, source: Option<Rect>, tint: Color) {
        let r = self.screen.rect_to_screen(r);
        draw_texture_ex(
            texture,
            r.x,
            r.y,
            tint,
            DrawTextureParams {
                dest_size: Some(vec2(r.w, r.h)),
                source,
                ..Default::default()
            },
        );
    }

    /// `size` is the text height in world units.
    pub fn text(&self, r: Rect, text: &str, size: f32, color: Color, align: Align) {
        let r = self.screen.rect_to_screen(r);
        let font = self.resources.get_font();
        let font_scale = self.screen.font_scale(FONT_SIZE, size);
        let d = measure_text(text, Some(font), FONT_SIZE, font_scale);
        let x = match align {
            Align::Left => r.x,
            Align::Center => r.x + (r.w - d.width) / 2.0,
            Align::Right => r.right() - d.width,
        };
        let y = r.y + (r.h - d.height) / 2.0 + d.offset_y;
        draw_text_ex(
            text,
            x,
            y,
            TextParams {
                font: Some(font),
                font_size: FONT_SIZE,
                font_scale,
                color,
                ..Default::default()
            },
        );
    }

    /// A word in a frame. Returns true on a click.
    pub fn button(&self, r: Rect, text: &str, size: f32, color: Color, enabled: bool) -> bool {
        let hovered = self.frame(r, enabled);
        let color = match (enabled, hovered) {
            (false, _) => MUTED,
            (true, true) => WHITE,
            (true, false) => color,
        };
        self.text(r, text, size, color, Align::Center);
        hovered && is_mouse_button_pressed(MouseButton::Left)
    }

    /// Two choices, each on half of `r`. Returns the choice clicked.
    pub fn choice(
        &self,
        r: Rect,
        choices: [&str; 2],
        selected: usize,
        enabled: [bool; 2],
        size: f32,
    ) -> Option<usize> {
        let w = r.w / 2.0;
        let mut clicked = None;
        for i in 0..2 {
            let part = Rect::new(r.x + w * i as f32, r.y, w, r.h);
            if i == selected {
                self.fill(part, VALUE);
                self.text(part, choices[i], size, BLACK, Align::Center);
            } else if self.button(part, choices[i], size, TEXT, enabled[i]) {
                clicked = Some(i);
            }
        }
        clicked
    }

    /// A value over two halves: a click on a half steps down or up. Returns -1 or 1.
    pub fn stepper(&self, r: Rect, text: &str, can: [bool; 2], size: f32) -> Option<i32> {
        let w = r.w / 2.0;
        let left = Rect::new(r.x, r.y, w, r.h);
        let right = Rect::new(r.x + w, r.y, w, r.h);
        let left_hovered = self.frame(left, can[0]);
        let right_hovered = self.frame(right, can[1]);
        self.arrow(left, -1.0, can[0]);
        self.arrow(right, 1.0, can[1]);
        self.text(r, text, size, VALUE, Align::Center);
        if !is_mouse_button_pressed(MouseButton::Left) {
            None
        } else if left_hovered {
            Some(-1)
        } else if right_hovered {
            Some(1)
        } else {
            None
        }
    }

    pub fn progress(&self, r: Rect, fraction: f32, color: Color) {
        self.fill(r, BAR);
        self.fill(
            Rect::new(r.x, r.y, r.w * fraction.clamp(0.0, 1.0), r.h),
            color,
        );
    }

    // Draws the frame. Returns true if an enabled widget is under the cursor.
    fn frame(&self, r: Rect, enabled: bool) -> bool {
        let hovered = enabled && self.hovered(r);
        self.fill(r, if hovered { FRAME_HOVER } else { FRAME_FILL });
        let s = self.screen.rect_to_screen(r);
        let line = if enabled { FRAME_LINE } else { DISABLED_LINE };
        draw_rectangle_lines(s.x, s.y, s.w, s.h, 2.0, line);
        hovered
    }

    // A filled triangle at the outer edge of `r`. `direction` is -1 for left, 1 for right.
    fn arrow(&self, r: Rect, direction: f32, enabled: bool) {
        let s = self.screen.rect_to_screen(r);
        let half = s.h * 0.3;
        let center = if direction < 0.0 {
            s.x + s.h * 0.6
        } else {
            s.right() - s.h * 0.6
        };
        let y = s.y + s.h / 2.0;
        draw_triangle(
            vec2(center + direction * half, y),
            vec2(center - direction * half, y - half),
            vec2(center - direction * half, y + half),
            if enabled { WHITE } else { DISABLED_LINE },
        );
    }

    fn hovered(&self, r: Rect) -> bool {
        self.screen.mouse_world().is_some_and(|p| r.contains(p))
    }
}
