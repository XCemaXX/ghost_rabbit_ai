use game_logic::{SCREEN_HEIGHT, SCREEN_WEIGHT};
use macroquad::prelude::*;

use super::models::{GENERATIONS, ROUNDS, Scores, Selection};
use super::widgets::{Align, LINE, TEXT, Ui, VALUE};
use crate::resources::Backgrounds;

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

const SPEEDS: [usize; 3] = [1, 2, 4];

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
    points: &'static [usize],
    color: Color,
    my_model: bool,
    index: usize,
}

impl Card {
    fn new(
        title: &'static str,
        unit: &'static str,
        points: &'static [usize],
        color: Color,
    ) -> Self {
        Self {
            title,
            unit,
            points,
            color,
            my_model: false,
            index: points.len() - 1,
        }
    }

    fn value(&self) -> usize {
        self.points[self.index]
    }

    // Returns true if the value changed. `score` belongs to the pair of nets, so one card shows it.
    fn draw(&mut self, ui: &Ui, r: Rect, score: Option<u32>) -> bool {
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

        let last = self.points.len() - 1;
        let text = format!("{} {} / {}", self.unit, self.value(), self.points[last]);
        let step = ui.stepper(row(CONTROL_H), &text, [true, true], SMALL);
        match step {
            Some(-1) => {
                self.index = if self.index == 0 {
                    last
                } else {
                    self.index - 1
                }
            }
            Some(_) => {
                self.index = if self.index == last {
                    0
                } else {
                    self.index + 1
                }
            }
            None => {}
        }

        let score_row = row(SCORE_H);
        if let Some(score) = score {
            ui.text(score_row, "Test score", SMALL, TEXT, Align::Left);
            let score = score.to_string();
            ui.text(score_row, &score, SMALL, VALUE, Align::Right);
        }

        ui.progress(row(PROGRESS_H), 0.0, self.color);

        let buttons = row(CONTROL_H);
        let half = (buttons.w - GAP) / 2.0;
        let left = Rect::new(buttons.x, buttons.y, half, buttons.h);
        let right = Rect::new(buttons.right() - half, buttons.y, half, buttons.h);
        let train = format!("TRAIN TO {}", self.points[(self.index + 1).min(last)]);
        ui.button(left, &train, SMALL, TRAIN_COLOR, false);
        ui.button(right, "RESET", SMALL, RESET_COLOR, false);
        step.is_some()
    }
}

pub struct Panel {
    vision: Card,
    control: Card,
    speed_index: usize,
}

impl Panel {
    pub fn new() -> Self {
        Self {
            vision: Card::new("VISION", "ROUND", &ROUNDS, VISION_COLOR),
            control: Card::new("CONTROL", "GEN", &GENERATIONS, CONTROL_COLOR),
            speed_index: 1,
        }
    }

    /// Game steps per screen frame.
    pub fn speed(&self) -> usize {
        SPEEDS[self.speed_index]
    }

    pub fn selection(&self) -> Selection {
        Selection {
            round: self.vision.value(),
            generation: self.control.value(),
        }
    }

    // Goes after the game: the game paints everything outside its field as bars.
    // Returns true if the selection changed.
    pub fn draw(&mut self, ui: &Ui, scores: &Scores) -> bool {
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
        let speed_rect = Rect::new(
            inner.x + inner.w / 2.0,
            inner.y + GAP,
            inner.w / 2.0,
            PANEL_TITLE_H - 2.0 * GAP,
        );
        let speed = format!("SPEED x{}", self.speed());
        if ui.button(speed_rect, &speed, SMALL, VALUE, true) {
            self.speed_index = (self.speed_index + 1) % SPEEDS.len();
        }
        let card_h = (r.h - PANEL_TITLE_H - 2.0 * MARGIN) / 2.0;
        let card = |y: f32| Rect::new(inner.x, y, inner.w, card_h);
        let score = scores.get(self.selection());
        let vision = self.vision.draw(ui, card(r.y + PANEL_TITLE_H), None);
        let control =
            self.control
                .draw(ui, card(r.y + PANEL_TITLE_H + card_h + MARGIN), Some(score));
        vision || control
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
