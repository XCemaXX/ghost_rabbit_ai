use game_logic::SCREEN_HEIGHT;
use macroquad::prelude::*;

use crate::draw::FONT_SIZE;
use crate::letterbox::Letterbox;
use crate::resources::{Labels, Resources};

pub const TEXT: Color = Color::new(0.88, 0.88, 0.88, 1.0);
pub const SUBTLE: Color = Color::new(0.63, 0.63, 0.63, 1.0);
// What to do next and what came of it.
pub const ACTION: Color = YELLOW;
pub const DANGER: Color = Color::new(1.0, 0.37, 0.33, 1.0);
const DANGER_QUIET: Color = Color::new(0.84, 0.47, 0.43, 1.0);
const SELECTED: Color = Color::new(0.47, 0.75, 0.93, 1.0);
const ON_FILL: Color = Color::new(0.07, 0.07, 0.07, 1.0);
pub const LINE: Color = Color::new(1.0, 1.0, 1.0, 0.25);
// World units per pixel of the game's original screen height.
const ORIGINAL_PX: f32 = SCREEN_HEIGHT / 640.0;
// The corners of the frame texture, in its pixels: only the part between them stretches.
const FRAME_SIDE_PX: f32 = 32.0;
const FRAME_TOP_PX: f32 = 43.0;
const FRAME_BOTTOM_PX: f32 = 44.0;
// The drips hang below the frame line.
const FRAME_DRIPS_PX: f32 = 16.0;
const FRAME_WORLD_PER_PX: f32 = 0.27 * ORIGINAL_PX;
// Fills stay inside the cut corners of the frame.
const FRAME_INSET: f32 = 3.0 * ORIGINAL_PX;
// As shares of the control height.
const ARROW_H: f32 = 0.62;
const ARROW_MARGIN: f32 = 0.35;
// A strip at the bottom of the button, so the text stays on a dark background.
const PROGRESS_H: f32 = 0.22;
const FRAME_FILL: Color = Color::new(0.0, 0.0, 0.0, 0.35);
const HOVER_FILL: Color = Color::new(1.0, 1.0, 1.0, 0.1);
const FRAME: Color = Color::new(0.47, 0.47, 0.47, 1.0);
const DISABLED_FRAME: Color = Color::new(0.27, 0.27, 0.27, 1.0);
const DISABLED_TEXT: Color = Color::new(0.43, 0.43, 0.43, 1.0);

/// Colors of a button that can be pressed. A disabled one looks the same for all.
pub struct ButtonStyle {
    frame: Color,
    text: Color,
    hover_frame: Color,
    hover_text: Color,
    hover_fill: Color,
}

pub const NEUTRAL: ButtonStyle = ButtonStyle {
    frame: FRAME,
    text: TEXT,
    hover_frame: TEXT,
    hover_text: WHITE,
    hover_fill: HOVER_FILL,
};
pub const MAIN: ButtonStyle = ButtonStyle {
    frame: ACTION,
    text: ACTION,
    hover_frame: ACTION,
    hover_text: ACTION,
    hover_fill: HOVER_FILL,
};
pub const RESET: ButtonStyle = ButtonStyle {
    frame: FRAME,
    text: DANGER_QUIET,
    hover_frame: DANGER_QUIET,
    hover_text: DANGER,
    hover_fill: HOVER_FILL,
};
pub const SURE: ButtonStyle = ButtonStyle {
    frame: DANGER,
    text: DANGER,
    hover_frame: DANGER,
    hover_text: ON_FILL,
    hover_fill: DANGER,
};

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
    pub fn image(&self, r: Rect, texture: &Texture2D, source: Rect, tint: Color) {
        let r = self.screen.rect_to_screen(r);
        draw_texture_ex(
            texture,
            r.x,
            r.y,
            tint,
            DrawTextureParams {
                dest_size: Some(vec2(r.w, r.h)),
                source: Some(source),
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

    /// A whole texture of height `h`, with its top left corner at (`x`, `y`).
    pub fn label(&self, label: &Labels, x: f32, y: f32, h: f32) {
        let (texture, x_to_y) = self.resources.get_label(label);
        let source = Rect::new(0.0, 0.0, texture.width(), texture.height());
        self.image(Rect::new(x, y, h * x_to_y, h), texture, source, WHITE);
    }

    /// A word in a frame. Returns true on a click.
    pub fn button(
        &self,
        r: Rect,
        text: &str,
        size: f32,
        style: &ButtonStyle,
        enabled: bool,
    ) -> bool {
        self.button_with_progress(r, text, size, style, enabled, None)
    }

    /// A button of a running job: a strip at its bottom shows how far the job is.
    pub fn progress_button(&self, r: Rect, text: &str, size: f32, fraction: f32) -> bool {
        self.button_with_progress(r, text, size, &NEUTRAL, true, Some(fraction))
    }

    fn button_with_progress(
        &self,
        r: Rect,
        text: &str,
        size: f32,
        style: &ButtonStyle,
        enabled: bool,
        progress: Option<f32>,
    ) -> bool {
        let hovered = enabled && self.hovered(r);
        let (fill, frame, text_color) = match (enabled, hovered) {
            (false, _) => (FRAME_FILL, DISABLED_FRAME, DISABLED_TEXT),
            (true, false) => (FRAME_FILL, style.frame, style.text),
            (true, true) => (style.hover_fill, style.hover_frame, style.hover_text),
        };
        let inner = inside(r);
        self.fill(inner, fill);
        // Under the frame: it hides the ends of the strip, also at the cut corners.
        if let Some(fraction) = progress {
            let h = inner.h * PROGRESS_H;
            let w = inner.w * fraction.clamp(0.0, 1.0);
            self.fill(Rect::new(inner.x, inner.bottom() - h, w, h), ACTION);
        }
        self.frame(r, frame);
        self.text(r, text, size, text_color, Align::Center);
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
                self.fill(inside(part), SELECTED);
                self.frame(part, SELECTED);
                self.text(part, choices[i], size, ON_FILL, Align::Center);
            } else if self.button(part, choices[i], size, &NEUTRAL, enabled[i]) {
                clicked = Some(i);
            }
        }
        clicked
    }

    /// A value between two arrows. A click on a half steps down or up: returns -1 or 1.
    /// One frame: a line between the halves would cross the text.
    pub fn stepper(&self, r: Rect, text: &str, enabled: bool, size: f32) -> Option<i32> {
        let w = r.w / 2.0;
        let left = Rect::new(r.x, r.y, w, r.h);
        let right = Rect::new(r.x + w, r.y, w, r.h);
        let left_hovered = enabled && self.hovered(left);
        let right_hovered = enabled && self.hovered(right);
        let inner = inside(r);
        self.fill(inner, FRAME_FILL);
        if left_hovered || right_hovered {
            let half = inner.w / 2.0;
            let x = if left_hovered {
                inner.x
            } else {
                inner.x + half
            };
            self.fill(Rect::new(x, inner.y, half, inner.h), HOVER_FILL);
        }
        let (frame, text_color) = if enabled {
            (FRAME, TEXT)
        } else {
            (DISABLED_FRAME, DISABLED_TEXT)
        };
        self.frame(r, frame);
        self.arrow(left, -1.0, enabled);
        self.arrow(right, 1.0, enabled);
        self.text(r, text, size, text_color, Align::Center);
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

    // The frame texture stretched to `r`: its corners keep their size, its drips hang below `r`.
    fn frame(&self, r: Rect, color: Color) {
        let (texture, _) = self.resources.get_label(&Labels::AiFrame);
        let (w, h) = (texture.width(), texture.height());
        let world = |px: f32| px * FRAME_WORLD_PER_PX;
        let (side, top, bottom) = (FRAME_SIDE_PX, FRAME_TOP_PX, FRAME_BOTTOM_PX);
        let full_h = r.h + world(FRAME_DRIPS_PX);
        let columns = [
            (0.0, side, r.x, world(side)),
            (
                side,
                w - 2.0 * side,
                r.x + world(side),
                r.w - world(2.0 * side),
            ),
            (w - side, side, r.right() - world(side), world(side)),
        ];
        let rows = [
            (0.0, top, r.y, world(top)),
            (
                top,
                h - top - bottom,
                r.y + world(top),
                full_h - world(top + bottom),
            ),
            (
                h - bottom,
                bottom,
                r.y + full_h - world(bottom),
                world(bottom),
            ),
        ];
        for (sx, sw, x, dw) in columns {
            for (sy, sh, y, dh) in rows {
                self.image(
                    Rect::new(x, y, dw, dh),
                    texture,
                    Rect::new(sx, sy, sw, sh),
                    color,
                );
            }
        }
    }

    fn hovered(&self, r: Rect) -> bool {
        self.screen.mouse_world().is_some_and(|p| r.contains(p))
    }

    // At the outer edge of `r`. `direction` is -1 for left, 1 for right.
    fn arrow(&self, r: Rect, direction: f32, enabled: bool) {
        let (texture, x_to_y) = self.resources.get_label(&Labels::AiArrow);
        let h = r.h * ARROW_H;
        let w = h * x_to_y;
        let margin = r.h * ARROW_MARGIN;
        let x = if direction < 0.0 {
            r.x + margin
        } else {
            r.right() - margin - w
        };
        let s = self
            .screen
            .rect_to_screen(Rect::new(x, r.y + (r.h - h) / 2.0, w, h));
        draw_texture_ex(
            texture,
            s.x,
            s.y,
            if enabled { TEXT } else { DISABLED_FRAME },
            DrawTextureParams {
                dest_size: Some(vec2(s.w, s.h)),
                flip_x: direction < 0.0,
                ..Default::default()
            },
        );
    }
}

fn inside(r: Rect) -> Rect {
    Rect::new(
        r.x + FRAME_INSET,
        r.y + FRAME_INSET,
        r.w - 2.0 * FRAME_INSET,
        r.h - 2.0 * FRAME_INSET,
    )
}
