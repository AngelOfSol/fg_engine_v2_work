use ggez::graphics::Color;
use inspect_design::Inspect;

use serde::{Deserialize, Serialize};
use strum::Display;

use crate::graphics::keyframe::{EaseType, Keyframe, KeyframeExt, Keyframes};

#[derive(Debug, Clone, Copy)]
pub struct FlashOverlay {
    flash_type: FlashType,
    current_time: usize,
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
        let alpha = Keyframes::with_data(
            vec![
                (
                    0,
                    Keyframe {
                        value: 0.0,
                        function: EaseType::EaseOut,
                    },
                ),
                (
                    4,
                    Keyframe {
                        value: 1.0,
                        function: EaseType::EaseIn,
                    },
                ),
                (
                    self.duration() - 1,
                    Keyframe {
                        value: 0.0,
                        function: EaseType::Constant,
                    },
                ),
            ],
            self.duration(),
        )
        .unwrap();
        let alpha = alpha.get_eased(self.current_time).unwrap_or(0.0);

        match self.flash_type {
            FlashType::GuardCrush => Color::new(0.7, 0.0, 0.1, 0.7 * alpha),
            FlashType::Super => Color::new(0.0, 0.0, 0.0, 0.85 * alpha),
            FlashType::PartialSuper => Color::new(0.00, 0.0, 0.0, 0.4 * alpha),
        }
    }

    pub fn duration(&self) -> usize {
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
