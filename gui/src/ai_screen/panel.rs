use game_logic::{SCREEN_HEIGHT, SCREEN_WEIGHT};
use macroquad::prelude::*;

use bot::job::POINTS;

use super::models::{Models, ROUNDS, Selection};
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

/// What the panel asks the screen to do.
#[derive(PartialEq, Eq)]
pub enum Action {
    None,
    NewGame,
    Train,
    Stop,
}

/// What a card shows in this frame.
struct View {
    points: Vec<usize>,
    my_enabled: bool,
    score: Option<u32>,
    note: Option<String>,
    status: &'static str,
    progress: f32,
    training: bool,
    next: Option<usize>,
    can_train: bool,
}

struct Card {
    title: &'static str,
    unit: &'static str,
    color: Color,
    my_model: bool,
    index: usize,
}

impl Card {
    fn new(title: &'static str, unit: &'static str, color: Color, last: usize) -> Self {
        Self {
            title,
            unit,
            color,
            my_model: false,
            index: last,
        }
    }

    fn draw(&mut self, ui: &Ui, r: Rect, view: &View) -> Action {
        let mut action = Action::None;
        ui.fill(Rect::new(r.x, r.y, r.w, GAP / 4.0), LINE);
        let mut y = r.y + GAP;
        let mut row = |h: f32| {
            let row = Rect::new(r.x, y, r.w, h);
            y += h + GAP;
            row
        };

        let title = row(TITLE_H);
        ui.text(title, self.title, BIG, TEXT, Align::Left);
        ui.text(title, view.status, SMALL, self.color, Align::Right);

        let choices = ["PRETRAINED", "MY MODEL"];
        let enabled = [!view.training, !view.training && view.my_enabled];
        let selected = self.my_model as usize;
        if let Some(i) = ui.choice(row(CONTROL_H), choices, selected, enabled, SMALL) {
            self.my_model = i == 1;
            self.index = usize::MAX;
            action = Action::NewGame;
        }

        let last = view.points.len() - 1;
        self.index = self.index.min(last);
        let text = format!(
            "{} {} / {}",
            self.unit, view.points[self.index], view.points[last]
        );
        let can = !view.training;
        match ui.stepper(row(CONTROL_H), &text, [can, can], SMALL) {
            Some(-1) => {
                self.index = if self.index == 0 {
                    last
                } else {
                    self.index - 1
                };
                action = Action::NewGame;
            }
            Some(_) => {
                self.index = if self.index == last {
                    0
                } else {
                    self.index + 1
                };
                action = Action::NewGame;
            }
            None => {}
        }

        let score_row = row(SCORE_H);
        if let Some(score) = view.score {
            ui.text(score_row, "Test score", SMALL, TEXT, Align::Left);
            ui.text(score_row, &score.to_string(), SMALL, VALUE, Align::Right);
        }
        if let Some(note) = &view.note {
            ui.text(score_row, note, SMALL, TEXT, Align::Center);
        }

        ui.progress(row(PROGRESS_H), view.progress, self.color);

        let buttons = row(CONTROL_H);
        let half = (buttons.w - GAP) / 2.0;
        let left = Rect::new(buttons.x, buttons.y, half, buttons.h);
        let right = Rect::new(buttons.right() - half, buttons.y, half, buttons.h);
        if view.training {
            if ui.button(left, "STOP", SMALL, RESET_COLOR, true) {
                action = Action::Stop;
            }
        } else if let Some(next) = view.next {
            let train = format!("TRAIN TO {next}");
            if ui.button(left, &train, SMALL, TRAIN_COLOR, view.can_train) {
                action = Action::Train;
            }
        } else {
            ui.button(left, "DONE", SMALL, TRAIN_COLOR, false);
        }
        ui.button(right, "RESET", SMALL, RESET_COLOR, false);
        action
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
            vision: Card::new("VISION", "ROUND", VISION_COLOR, ROUNDS.len() - 1),
            control: Card::new("CONTROL", "GEN", CONTROL_COLOR, POINTS.len() - 1),
            speed_index: 1,
        }
    }

    /// Game steps per screen frame.
    pub fn speed(&self) -> usize {
        SPEEDS[self.speed_index]
    }

    pub fn selection(&self, models: &Models) -> Selection {
        let round = ROUNDS[self.vision.index];
        let generations = self.control_points(models, round);
        Selection {
            round,
            generation: generations[self.control.index.min(generations.len() - 1)],
            my_policy: self.control.my_model,
        }
    }

    /// Shows the last point of the player's policy.
    pub fn select_my_policy(&mut self) {
        self.control.my_model = true;
        self.control.index = usize::MAX;
    }

    fn control_points(&self, models: &Models, round: usize) -> Vec<usize> {
        if !self.control.my_model {
            return POINTS.to_vec();
        }
        let generations = models.my_generations(round);
        // An untrained policy is generation 0.
        if generations.is_empty() {
            vec![0]
        } else {
            generations
        }
    }

    // Goes after the game: the game paints everything outside its field as bars.
    // `progress` is the state of the running job.
    pub fn draw(&mut self, ui: &Ui, models: &Models, progress: Option<f32>) -> Action {
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
        let training = progress.is_some();

        let vision_view = View {
            points: ROUNDS.to_vec(),
            my_enabled: false,
            score: None,
            note: None,
            status: "READY",
            progress: 0.0,
            training,
            next: ROUNDS.get(self.vision.index + 1).copied(),
            can_train: false,
        };
        let vision = self
            .vision
            .draw(ui, card(r.y + PANEL_TITLE_H), &vision_view);
        let round = ROUNDS[self.vision.index];

        let selection = self.selection(models);
        let next = models.next_point(round);
        let control_view = View {
            points: self.control_points(models, round),
            my_enabled: true,
            score: models.score(selection),
            note: models
                .inherited(round)
                .filter(|_| selection.my_policy)
                .map(|(r, g)| format!("from r{r} g{g}")),
            status: match (training, selection.my_policy) {
                (true, _) => "TRAINING",
                (false, false) => "READY",
                (false, true) if models.score(selection).is_none() => "NO DATA",
                (false, true) if next.is_none() => "DONE",
                (false, true) => "READY",
            },
            progress: progress.unwrap_or(0.0),
            training,
            next,
            can_train: self.control.my_model,
        };
        let control = self.control.draw(
            ui,
            card(r.y + PANEL_TITLE_H + card_h + MARGIN),
            &control_view,
        );
        if vision == Action::NewGame {
            Action::NewGame
        } else {
            control
        }
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
