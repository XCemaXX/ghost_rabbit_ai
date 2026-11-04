use game_logic::{SCREEN_HEIGHT, SCREEN_WEIGHT};
use macroquad::prelude::*;

use super::widgets::{Align, LINE, TEXT, Ui, VALUE};
use crate::resources::Backgrounds;

const LAST_ROUND: usize = 7;
const VISION_COLOR: Color = Color::new(0.25, 0.75, 0.95, 1.0);
const CONTROL_COLOR: Color = Color::new(0.95, 0.5, 0.2, 1.0);
const TRAIN_COLOR: Color = Color::new(0.55, 0.95, 0.3, 1.0);
const RESET_COLOR: Color = Color::new(1.0, 0.45, 0.45, 1.0);
// The splash is too bright for text at full brightness.
const BACKGROUND_TINT: Color = Color::new(0.6, 0.6, 0.6, 1.0);

// Parts of the options background, as shares of its height: above CLEAN_FROM there is baked text,
// and the part up to CLEAN_TO has no splash, so it can repeat.
const CLEAN_FROM: f32 = 420.0 / 640.0;
const CLEAN_TO: f32 = 490.0 / 640.0;

const MARGIN: f32 = SCREEN_WEIGHT * 0.05;
const GAP: f32 = SCREEN_HEIGHT * 0.012;
const PANEL_TITLE_H: f32 = SCREEN_HEIGHT * 0.07;
const TITLE_H: f32 = SCREEN_HEIGHT * 0.06;
const CONTROL_H: f32 = SCREEN_HEIGHT * 0.075;
const SCORE_H: f32 = SCREEN_HEIGHT * 0.05;
const PROGRESS_H: f32 = SCREEN_HEIGHT * 0.018;
const BIG: f32 = SCREEN_WEIGHT / 12.0;
const SMALL: f32 = SCREEN_WEIGHT / 17.0;

struct Card {
    title: &'static str,
    unit: &'static str,
    step: usize,
    last: usize,
    train: &'static str,
    color: Color,
    my_model: bool,
    value: usize,
}

impl Card {
    fn new(
        title: &'static str,
        unit: &'static str,
        step: usize,
        last: usize,
        train: &'static str,
        color: Color,
    ) -> Self {
        Self {
            title,
            unit,
            step,
            last,
            train,
            color,
            my_model: false,
            value: last,
        }
    }

    fn draw(&mut self, ui: &Ui, r: Rect) {
        ui.fill(Rect::new(r.x, r.y, r.w, GAP / 4.0), LINE);
        let mut y = r.y + GAP;
        let mut row = |h: f32| {
            let row = Rect::new(r.x, y, r.w, h);
            y += h + GAP;
            row
        };

        let title = row(TITLE_H);
        ui.text(title, self.title, BIG, TEXT, Align::Left);
        ui.text(title, "READY", SMALL, self.color, Align::Right);

        let choices = ["PRETRAINED", "MY MODEL"];
        let selected = self.my_model as usize;
        if let Some(i) = ui.choice(row(CONTROL_H), choices, selected, [true, false], SMALL) {
            self.my_model = i == 1;
        }

        let text = format!("{} {} / {}", self.unit, self.value, self.last);
        let can = [self.value > 0, self.value < self.last];
        match ui.stepper(row(CONTROL_H), &text, can, SMALL) {
            Some(-1) => self.value -= self.step,
            Some(_) => self.value += self.step,
            None => {}
        }

        let score = row(SCORE_H);
        ui.text(score, "Test score", SMALL, TEXT, Align::Left);
        ui.text(score, "-", SMALL, VALUE, Align::Right);

        ui.progress(row(PROGRESS_H), 0.0, self.color);

        let buttons = row(CONTROL_H);
        let half = (buttons.w - GAP) / 2.0;
        let left = Rect::new(buttons.x, buttons.y, half, buttons.h);
        let right = Rect::new(buttons.right() - half, buttons.y, half, buttons.h);
        ui.button(left, self.train, SMALL, TRAIN_COLOR, false);
        ui.button(right, "RESET", SMALL, RESET_COLOR, false);
    }
}

pub struct Panel {
    vision: Card,
    control: Card,
}

impl Panel {
    pub fn new() -> Self {
        Self {
            vision: Card::new("VISION", "ROUND", 1, LAST_ROUND, "TRAIN +1", VISION_COLOR),
            control: Card::new(
                "CONTROL",
                "GEN",
                bot::GENERATION_STEP,
                bot::MAX_GENERATION,
                "TRAIN +5",
                CONTROL_COLOR,
            ),
        }
    }

    // Goes after the game: the game paints everything outside its field as bars.
    pub fn draw(&mut self, ui: &Ui) {
        let r = Rect::new(SCREEN_WEIGHT, 0.0, SCREEN_WEIGHT, SCREEN_HEIGHT);
        draw_background(ui, r);
        let inner = Rect::new(r.x + MARGIN, r.y, r.w - 2.0 * MARGIN, r.h);
        ui.text(
            Rect::new(inner.x, inner.y, inner.w, PANEL_TITLE_H),
            "AI TRAINING",
            BIG,
            TEXT,
            Align::Left,
        );
        let card_h = (r.h - PANEL_TITLE_H - 2.0 * MARGIN) / 2.0;
        let card = |y: f32| Rect::new(inner.x, y, inner.w, card_h);
        self.vision.draw(ui, card(r.y + PANEL_TITLE_H));
        self.control
            .draw(ui, card(r.y + PANEL_TITLE_H + card_h + MARGIN));
    }
}

// The options background without its baked text: a clean strip repeats down to the splash.
fn draw_background(ui: &Ui, r: Rect) {
    let texture = ui.resources.get_background(&Backgrounds::Options);
    let (w, h) = (texture.width(), texture.height());
    let strip = Rect::new(0.0, h * CLEAN_FROM, w, h * (CLEAN_TO - CLEAN_FROM));
    let strip_h = r.h * (CLEAN_TO - CLEAN_FROM);
    let strips = (CLEAN_FROM / (CLEAN_TO - CLEAN_FROM)).round() as usize;
    for i in 0..strips {
        let dest = Rect::new(r.x, r.y + strip_h * i as f32, r.w, strip_h);
        ui.image(dest, texture, Some(strip), BACKGROUND_TINT);
    }
    let bottom = Rect::new(0.0, h * CLEAN_FROM, w, h * (1.0 - CLEAN_FROM));
    let dest_h = r.h * (1.0 - CLEAN_FROM);
    ui.image(
        Rect::new(r.x, r.bottom() - dest_h, r.w, dest_h),
        texture,
        Some(bottom),
        BACKGROUND_TINT,
    );
}
