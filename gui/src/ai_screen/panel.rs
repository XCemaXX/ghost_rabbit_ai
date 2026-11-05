use game_logic::{SCREEN_HEIGHT, SCREEN_WEIGHT};
use macroquad::prelude::*;

use bot::policy_job::POINTS;
use bot::vision_job::VISION_ROUNDS;

use super::models::{Models, PolicyKey, Selection};
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
const CONTROL_H: f32 = SCREEN_HEIGHT * 0.07;
const RESULT_H: f32 = SCREEN_HEIGHT * 0.05;
const DETAIL_H: f32 = SCREEN_HEIGHT * 0.04;
const CARD_GAP: f32 = SCREEN_HEIGHT * 0.02;
const PROGRESS_H: f32 = SCREEN_HEIGHT * 0.018;
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
}

#[derive(Clone, Copy)]
pub enum Training {
    Vision(f32),
    Policy(f32),
}

struct View {
    points: Vec<usize>,
    pretrained_enabled: bool,
    /// Some job runs: the choices are locked.
    busy: bool,
    /// The progress of this card's job.
    progress: Option<f32>,
    next: Option<usize>,
    can_train: bool,
    train: Action,
}

struct Card {
    title: &'static str,
    purpose: &'static str,
    unit: &'static str,
    color: Color,
    my_model: bool,
    index: usize,
}

impl Card {
    // A large index means the last point: the draw clamps it.
    fn new(title: &'static str, purpose: &'static str, unit: &'static str, color: Color) -> Self {
        Self {
            title,
            purpose,
            unit,
            color,
            my_model: false,
            index: usize::MAX,
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
        ui.text(title, self.title, BIG, TEXT, Align::Left);
        ui.text(title, self.purpose, TINY, self.color, Align::Right);

        let choices = ["PRETRAINED", "MY MODEL"];
        let enabled = [!view.busy && view.pretrained_enabled, !view.busy];
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
        match ui.stepper(row(CONTROL_H), &text, !view.busy, SMALL) {
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

        ui.progress(row(PROGRESS_H), view.progress.unwrap_or(0.0), self.color);

        let buttons = row(CONTROL_H);
        let half = (buttons.w - GAP) / 2.0;
        let left = Rect::new(buttons.x, buttons.y, half, buttons.h);
        let right = Rect::new(buttons.right() - half, buttons.y, half, buttons.h);
        if view.progress.is_some() {
            if ui.button(left, "STOP", SMALL, RESET_COLOR, true) {
                action = Action::Stop;
            }
        } else {
            let train = view
                .next
                .map_or("TRAIN".to_owned(), |next| format!("TRAIN TO {next}"));
            let enabled = view.next.is_some() && view.can_train && !view.busy;
            if ui.button(left, &train, SMALL, TRAIN_COLOR, enabled) {
                action = view.train;
            }
        }
        ui.button(right, "RESET", SMALL, RESET_COLOR, false);
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
            vision: Card::new("VISION", "Learns to see danger", "ROUND", VISION_COLOR),
            control: Card::new("CONTROL", "Learns to choose moves", "GEN", CONTROL_COLOR),
            speed_index: 1,
        }
    }

    /// Game steps per screen frame.
    pub fn speed(&self) -> usize {
        SPEEDS[self.speed_index]
    }

    pub fn selection(&self, models: &Models) -> Selection {
        let rounds = self.vision_points(models);
        let round = rounds[self.vision.index.min(rounds.len() - 1)];
        let key = (self.vision.my_model, round);
        let generations = self.control_points(models, key);
        Selection {
            round,
            generation: generations[self.control.index.min(generations.len() - 1)],
            my_vision: self.vision.my_model,
            my_policy: self.control.my_model,
        }
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

    fn vision_points(&self, models: &Models) -> Vec<usize> {
        if self.vision.my_model {
            // An untrained net is round 0.
            (0..models.my_vision_rounds().max(1)).collect()
        } else {
            (0..VISION_ROUNDS).collect()
        }
    }

    fn control_points(&self, models: &Models, key: PolicyKey) -> Vec<usize> {
        if !self.control.my_model {
            return POINTS.to_vec();
        }
        let generations = models.my_generations(key);
        // An untrained policy is generation 0.
        if generations.is_empty() {
            vec![0]
        } else {
            generations
        }
    }

    fn vision_view(&self, models: &Models, busy: bool, progress: Option<f32>) -> View {
        View {
            points: self.vision_points(models),
            pretrained_enabled: true,
            busy,
            progress,
            next: models.next_vision_round(),
            can_train: self.vision.my_model,
            train: Action::TrainVision,
        }
    }

    fn control_view(&self, models: &Models, busy: bool, progress: Option<f32>) -> View {
        let key = self.selection(models).policy_key();
        let vision_ready = !self.vision.my_model || models.my_vision_rounds() > 0;
        View {
            points: self.control_points(models, key),
            pretrained_enabled: !self.vision.my_model,
            busy,
            progress,
            next: models.next_point(key),
            can_train: self.control.my_model && vision_ready,
            train: Action::TrainPolicy(key),
        }
    }

    // The result belongs to the pair of nets, so it goes below both cards.
    fn draw_result(&self, ui: &Ui, models: &Models, r: Rect, busy: bool) {
        ui.fill(Rect::new(r.x, r.y, r.w, GAP / 4.0), LINE);
        let selection = self.selection(models);
        let row = |i: f32, h: f32| Rect::new(r.x, r.y + GAP + i * (DETAIL_H + GAP), r.w, h);
        // A point finished during a job says little: the job goes on to its target.
        if busy {
            ui.text(row(0.0, RESULT_H), "Training...", BIG, VALUE, Align::Left);
            return;
        }
        let Some(life) = models.life(selection) else {
            ui.text(
                row(0.0, RESULT_H),
                "Not trained yet",
                BIG,
                VALUE,
                Align::Left,
            );
            return;
        };
        let text = format!("Usually lives ~{life} s");
        ui.text(row(0.0, RESULT_H), &text, BIG, VALUE, Align::Left);
        let detail = "Half of 200 test games last longer";
        ui.text(row(1.0, DETAIL_H), detail, TINY, TEXT, Align::Left);
        let inherited = models
            .inherited(selection.policy_key())
            .filter(|_| selection.my_policy);
        if let Some((round, generation)) = inherited {
            let text = format!("Control gen 0 = elite of round {round}, gen {generation}");
            ui.text(row(2.0, DETAIL_H), &text, TINY, TEXT, Align::Left);
        }
    }

    // Goes after the game: the game paints everything outside its field as bars.
    pub fn draw(&mut self, ui: &Ui, models: &Models, training: Option<Training>) -> Action {
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

        let busy = training.is_some();
        let (vision_progress, control_progress) = match training {
            Some(Training::Vision(p)) => (Some(p), None),
            Some(Training::Policy(p)) => (None, Some(p)),
            None => (None, None),
        };
        let view = self.vision_view(models, busy, vision_progress);
        let (vision, y) = self
            .vision
            .draw(ui, inner.x, inner.w, r.y + PANEL_TITLE_H, &view);
        // A pretrained policy never runs on the player's `vision`.
        if self.vision.my_model {
            self.control.my_model = true;
        }
        let view = self.control_view(models, busy, control_progress);
        let (control, y) = self.control.draw(ui, inner.x, inner.w, y + CARD_GAP, &view);
        self.draw_result(ui, models, Rect::new(inner.x, y + GAP, inner.w, 0.0), busy);
        if vision != Action::None {
            vision
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
