use ggez::graphics::Color;
use inspect_design::Inspect;
use keyframe::{
    functions::{EaseInQuint, EaseOutQuint},
    keyframes, AnimationSequence,
};
use serde::{Deserialize, Serialize};
use strum::Display;

#[derive(Debug, Clone, Copy)]
pub struct FlashOverlay {
    flash_type: FlashType,
    current_time: i32,
}

impl From<FlashType> for FlashOverlay {
    fn from(flash_type: FlashType) -> Self {
        Self {
            flash_type,
            current_time: 0,
        }
    }
}
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash, Display, Inspect)]
pub enum FlashType {
    Super,
    GuardCrush,
    PartialSuper,
}

impl Default for FlashType {
    fn default() -> Self {
        Self::Super
    }
}

impl FlashOverlay {
    pub fn color(&self) -> Color {
        let mut alpha = keyframes! {
            (0.0, 0.0, EaseOutQuint),
            (1.0, 4.0, EaseInQuint),
            (0.0, self.duration() as f64)
        };
        alpha.advance_to(self.current_time as f64);

        match self.flash_type {
            FlashType::GuardCrush => Color::new(0.7, 0.0, 0.1, 0.7 * alpha.now()),
            FlashType::Super => Color::new(0.0, 0.0, 0.0, 0.85 * alpha.now()),
            FlashType::PartialSuper => Color::new(0.00, 0.0, 0.0, 0.4 * alpha.now()),
        }
    }

    pub fn duration(&self) -> i32 {
        match self.flash_type {
            FlashType::GuardCrush => 40,
            FlashType::Super => 20,
            FlashType::PartialSuper => 10,
        }
    }
    pub fn update(mut self) -> Option<Self> {
        self.current_time += 1;
        if self.current_time < self.duration() {
            Some(self)
        } else {
            None
        }
    }
}
