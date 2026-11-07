use game_logic::{SCREEN_HEIGHT, SCREEN_WEIGHT};
use macroquad::prelude::*;

use bot::policy_job::TEST_GAMES;

use super::models::{Models, Selection};
use super::storage::PolicyKey;
use super::widgets::{ACTION, Align, DANGER, LINE, MAIN, NEUTRAL, RESET, SUBTLE, SURE, TEXT, Ui};
use crate::resources::{Backgrounds, Labels};

// The splash is too bright for text at full brightness.
const BACKGROUND_TINT: Color = Color::new(0.6, 0.6, 0.6, 1.0);

// Parts of the options background, as shares of its height: above CLEAN_FROM there is baked text,
// and the part up to CLEAN_TO has no splash, so it can repeat.
const CLEAN_FROM: f32 = 420.0 / 640.0;
const CLEAN_TO: f32 = 490.0 / 640.0;

const SPEEDS: [usize; 3] = [1, 2, 4];

const RESET_CONFIRM_SEC: f64 = 3.0;

const MARGIN: f32 = SCREEN_WEIGHT * 0.05;
const GAP: f32 = SCREEN_HEIGHT * 0.012;
const PANEL_TITLE_H: f32 = SCREEN_HEIGHT * 0.1;
const PANEL_LABEL_H: f32 = SCREEN_HEIGHT * 0.09;
const SPEED_H: f32 = SCREEN_HEIGHT * 0.05;
const TITLE_H: f32 = SCREEN_HEIGHT * 0.07;
const CONTROL_H: f32 = SCREEN_HEIGHT * 0.0625;
const RESULT_H: f32 = SCREEN_HEIGHT * 0.05;
const DETAIL_H: f32 = SCREEN_HEIGHT * 0.04;
const CARD_GAP: f32 = SCREEN_HEIGHT * 0.02;
const BIG: f32 = SCREEN_WEIGHT / 12.0;
const SMALL: f32 = SCREEN_WEIGHT / 17.0;
const TINY: f32 = SCREEN_WEIGHT / 21.0;

/// What the panel asks the screen to do.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Action {
    None,
    NewGame,
    TrainVision,
    TrainPolicy(PolicyKey),
    Stop,
    ResetVision,
    ResetPolicy(PolicyKey),
}

#[derive(Clone, Copy)]
pub enum Training {
    Vision { progress: f32, rebuilding: bool },
    Policy(f32),
}

struct View {
    my_model: bool,
    points: Vec<usize>,
    pretrained_enabled: bool,
    /// Some job runs: the choices are locked.
    busy: bool,
    /// The progress of this card's job.
    progress: Option<f32>,
    next: Option<usize>,
    can_train: bool,
    train: Action,
    can_reset: bool,
    reset: Action,
}

struct Card {
    title: Labels,
    purpose: &'static str,
    unit: &'static str,
    my_model: bool,
    index: usize,
    reset_asked_at: f64,
}

impl Card {
    // A large index means the last point: each use clamps it to the list of that moment.
    fn new(title: Labels, purpose: &'static str, unit: &'static str) -> Self {
        Self {
            title,
            purpose,
            unit,
            my_model: false,
            index: usize::MAX,
            reset_asked_at: f64::NEG_INFINITY,
        }
    }

    fn select_my_model(&mut self) {
        self.my_model = true;
        self.index = usize::MAX;
    }

    // Starts at height `y` and returns the height below the card.
    fn draw(&mut self, ui: &Ui, x: f32, w: f32, y: f32, view: &View) -> (Action, f32) {
        let mut action = Action::None;
        ui.fill(Rect::new(x, y, w, GAP / 4.0), LINE);
        let mut next_y = y + GAP;
        let mut row = |h: f32| {
            let row = Rect::new(x, next_y, w, h);
            next_y += h + GAP;
            row
        };

        let title = row(TITLE_H);
        ui.label(&self.title, title.x, title.y, title.h);
        ui.text(title, self.purpose, TINY, SUBTLE, Align::Right);

        let choices = ["PRETRAINED", "MY MODEL"];
        let enabled = [!view.busy && view.pretrained_enabled, !view.busy];
        let selected = view.my_model as usize;
        if let Some(i) = ui.choice(row(CONTROL_H), choices, selected, enabled, SMALL) {
            self.my_model = i == 1;
            self.index = usize::MAX;
            action = Action::NewGame;
        }

        let last = view.points.len() - 1;
        let index = self.index.min(last);
        let text = format!(
            "{} {} / {}",
            self.unit, view.points[index], view.points[last]
        );
        if let Some(step) = ui.stepper(row(CONTROL_H), &text, !view.busy, SMALL) {
            let count = view.points.len() as i32;
            self.index = (index as i32 + step).rem_euclid(count) as usize;
            action = Action::NewGame;
        }

        let buttons = row(CONTROL_H);
        let half = (buttons.w - GAP) / 2.0;
        let left = Rect::new(buttons.x, buttons.y, half, buttons.h);
        let right = Rect::new(buttons.right() - half, buttons.y, half, buttons.h);
        if let Some(progress) = view.progress {
            if ui.progress_button(left, "STOP", SMALL, progress) {
                action = Action::Stop;
            }
        } else {
            let train = view
                .next
                .map_or("TRAIN".to_owned(), |next| format!("TRAIN TO {next}"));
            let enabled = view.next.is_some() && view.can_train && !view.busy;
            if ui.button(left, &train, SMALL, &MAIN, enabled) {
                action = view.train;
            }
        }
        let asked = get_time() - self.reset_asked_at < RESET_CONFIRM_SEC;
        let (reset, style) = if asked {
            ("SURE?", &SURE)
        } else {
            ("RESET", &RESET)
        };
        if ui.button(right, reset, SMALL, style, view.can_reset && !view.busy) {
            if asked {
                self.reset_asked_at = f64::NEG_INFINITY;
                action = view.reset;
            } else {
                self.reset_asked_at = get_time();
            }
        }
        (action, next_y)
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
            vision: Card::new(Labels::AiVision, "Learns to see danger", "ROUND"),
            control: Card::new(Labels::AiControl, "Learns to choose moves", "GEN"),
            speed_index: 1,
        }
    }

    /// How many times faster than real time the game runs.
    pub fn speed(&self) -> usize {
        SPEEDS[self.speed_index]
    }

    pub fn selection(&self, models: &Models) -> Selection {
        let rounds = models.rounds(self.vision.my_model);
        let round = rounds[self.vision.index.min(rounds.len() - 1)];
        let key = PolicyKey {
            my_vision: self.vision.my_model,
            round,
        };
        let generations = models.generations(key, self.my_policy());
        Selection {
            round,
            generation: generations[self.control.index.min(generations.len() - 1)],
            my_vision: self.vision.my_model,
            my_policy: self.my_policy(),
        }
    }

    // A pretrained policy never runs on the player's `vision`.
    fn my_policy(&self) -> bool {
        self.control.my_model || self.vision.my_model
    }

    /// Shows the last point of the player's policy.
    pub fn select_my_policy(&mut self) {
        self.control.select_my_model();
    }

    /// Shows the last round of the player's `vision`.
    pub fn select_my_vision(&mut self) {
        self.vision.select_my_model();
        self.control.select_my_model();
    }

    fn vision_view(&self, models: &Models, busy: bool, progress: Option<f32>) -> View {
        View {
            my_model: self.vision.my_model,
            points: models.rounds(self.vision.my_model),
            pretrained_enabled: true,
            busy,
            progress,
            next: models.next_vision_round(),
            can_train: self.vision.my_model,
            train: Action::TrainVision,
            can_reset: self.vision.my_model && models.my_vision_rounds() > 0,
            reset: Action::ResetVision,
        }
    }

    fn control_view(&self, models: &Models, busy: bool, progress: Option<f32>) -> View {
        let key = self.selection(models).policy_key();
        View {
            my_model: self.my_policy(),
            points: models.generations(key, self.my_policy()),
            pretrained_enabled: !self.vision.my_model,
            busy,
            progress,
            next: models.next_point(key),
            can_train: self.my_policy() && models.can_train_policy(key),
            train: Action::TrainPolicy(key),
            can_reset: self.my_policy() && models.has_points(key),
            reset: Action::ResetPolicy(key),
        }
    }

    // Why no training can start yet.
    fn train_hint(&self, models: &Models) -> Option<&'static str> {
        if !self.my_policy() {
            Some("To train, choose MY MODEL")
        } else if self.vision.my_model && models.my_vision_rounds() == 0 {
            Some("Train VISION first, then CONTROL")
        } else {
            None
        }
    }

    // The result belongs to the pair of nets, so it goes below both cards.
    fn draw_result(&self, ui: &Ui, models: &Models, r: Rect, job_text: Option<&str>) {
        ui.fill(Rect::new(r.x, r.y, r.w, GAP / 4.0), LINE);
        let row = |i: f32, h: f32| Rect::new(r.x, r.y + GAP + i * (DETAIL_H + GAP), r.w, h);
        let headline = |text: &str| ui.text(row(0.0, RESULT_H), text, BIG, ACTION, Align::Left);
        // A point finished during a job says little: the job goes on to its target.
        if let Some(text) = job_text {
            headline(text);
            return;
        }
        if let Some(hint) = self.train_hint(models) {
            ui.text(row(2.0, DETAIL_H), hint, TINY, TEXT, Align::Left);
        }
        if models.save_failed() {
            let text = "Not saved: browser storage is full";
            ui.text(row(3.0, DETAIL_H), text, TINY, DANGER, Align::Left);
        }
        let Some(result) = models.result(self.selection(models)) else {
            headline("Not trained yet");
            return;
        };
        let (life, score) = (result.life, result.score);
        headline(&format!("Lives ~{life:.0} s, ~{score:.0} points"));
        let detail = format!("Half of {TEST_GAMES} test games did better");
        ui.text(row(1.0, DETAIL_H), &detail, TINY, SUBTLE, Align::Left);
    }

    // Goes after the game: the game paints everything outside its field as bars.
    pub fn draw(&mut self, ui: &Ui, models: &Models, training: Option<Training>) -> Action {
        let r = Rect::new(SCREEN_WEIGHT, 0.0, SCREEN_WEIGHT, SCREEN_HEIGHT);
        draw_background(ui, r);
        let inner = Rect::new(r.x + MARGIN, r.y, r.w - 2.0 * MARGIN, r.h);
        let half = inner.w / 2.0;
        ui.label(&Labels::AiTraining, inner.x, inner.y, PANEL_LABEL_H);
        let speed_rect = Rect::new(
            inner.x + half + GAP,
            inner.y + (PANEL_TITLE_H - SPEED_H) / 2.0,
            half - GAP,
            SPEED_H,
        );
        let speed = format!("SPEED x{}", self.speed());
        if ui.button(speed_rect, &speed, SMALL, &NEUTRAL, true) {
            self.speed_index = (self.speed_index + 1) % SPEEDS.len();
        }

        let busy = training.is_some();
        let (vision_progress, control_progress) = match training {
            Some(Training::Vision { progress, .. }) => (Some(progress), None),
            Some(Training::Policy(p)) => (None, Some(p)),
            None => (None, None),
        };
        let my_policy_before = self.my_policy();
        let view = self.vision_view(models, busy, vision_progress);
        let (vision, y) = self
            .vision
            .draw(ui, inner.x, inner.w, r.y + PANEL_TITLE_H, &view);
        // The list of generations has just changed: show its last point.
        if self.my_policy() != my_policy_before {
            self.control.index = usize::MAX;
        }
        let view = self.control_view(models, busy, control_progress);
        let (control, y) = self.control.draw(ui, inner.x, inner.w, y + CARD_GAP, &view);
        let job_text = training.map(|t| match t {
            Training::Vision {
                rebuilding: true, ..
            } => "Restoring frames...",
            _ => "Training...",
        });
        self.draw_result(
            ui,
            models,
            Rect::new(inner.x, y + GAP, inner.w, 0.0),
            job_text,
        );
        let action = if vision != Action::None {
            vision
        } else {
            control
        };
        // The confirmation belongs to the model that was selected when it was asked.
        if action != Action::None {
            self.vision.reset_asked_at = f64::NEG_INFINITY;
            self.control.reset_asked_at = f64::NEG_INFINITY;
        }
        action
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
        ui.image(dest, texture, strip, BACKGROUND_TINT);
    }
    let bottom = Rect::new(0.0, h * CLEAN_FROM, w, h * (1.0 - CLEAN_FROM));
    let dest_h = r.h * (1.0 - CLEAN_FROM);
    ui.image(
        Rect::new(r.x, r.bottom() - dest_h, r.w, dest_h),
        texture,
        bottom,
        BACKGROUND_TINT,
    );
}
