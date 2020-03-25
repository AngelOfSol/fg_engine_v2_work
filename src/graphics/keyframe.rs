use crate::typedefs::graphics::{Matrix4, Vec2, Vec3};
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Display, EnumIter)]
pub enum Coordinates {
    Polar,
    Cartesian,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Modifiers {
    pub rotation: Keyframes,
    pub scale: [Keyframes; 2],
    pub coords: [Keyframes; 2],
    pub coord_type: Coordinates,
    // TODO use this
    pub alpha: Keyframes,
    // TODO use this
    pub value: Keyframes,
}

impl Default for Modifiers {
    fn default() -> Self {
        let default = Keyframes::new(Default::default());
        let percentage_default = Keyframes::new(Keyframe {
            value: 1.0,
            ..Default::default()
        });
        Self {
            coord_type: Coordinates::Cartesian,
            coords: [default.clone(), default.clone()],
            scale: [percentage_default.clone(), percentage_default.clone()],
            rotation: default,
            alpha: percentage_default.clone(),
            value: percentage_default.clone(),
        }
    }
}

impl Modifiers {
    pub fn with_basic(rotation: f32, scale: Vec2, coords: Vec2) -> Self {
        Modifiers {
            rotation: Keyframes::new(Keyframe {
                frame: 0,
                value: rotation,
                function: EaseType::Constant,
            }),
            coords: [
                Keyframes::new(Keyframe {
                    frame: 0,
                    value: coords.x,
                    function: EaseType::Constant,
                }),
                Keyframes::new(Keyframe {
                    frame: 0,
                    value: coords.y,
                    function: EaseType::Constant,
                }),
            ],
            scale: [
                Keyframes::new(Keyframe {
                    frame: 0,
                    value: scale.x,
                    function: EaseType::Constant,
                }),
                Keyframes::new(Keyframe {
                    frame: 0,
                    value: scale.y,
                    function: EaseType::Constant,
                }),
            ],
            ..Default::default()
        }
    }

    pub fn new() -> Self {
        Self::default()
    }

    pub fn matrix_at_time(&self, time: usize) -> Matrix4 {
        let rotation = Matrix4::new_rotation(Vec3::new(
            0.0,
            0.0,
            self.rotation.at_time(time).unwrap_or(0.0),
        ));

        let mut coords = self
            .coords
            .iter()
            .map(|coord| coord.at_time(time).unwrap_or(0.0));

        let (x, y) = match self.coord_type {
            Coordinates::Cartesian => (coords.next().unwrap(), coords.next().unwrap()),
            Coordinates::Polar => {
                let radius = coords.next().unwrap();
                let angle = coords.next().unwrap();

                (radius * angle.cos(), radius * angle.sin())
            }
        };

        let translate = Matrix4::new_translation(&Vec3::new(x, y, 0.0));

        let mut scale = self
            .scale
            .iter()
            .map(|scale| scale.at_time(time).unwrap_or(0.0));

        let (x, y) = (scale.next().unwrap(), scale.next().unwrap());

        let scale = Matrix4::new_nonuniform_scaling(&Vec3::new(x, y, 0.0));

        translate * scale * rotation
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Keyframes {
    pub frames: Vec<Keyframe>,
}

impl Keyframes {
    pub fn new(frame: Keyframe) -> Self {
        Self {
            frames: vec![frame],
        }
    }

    pub fn at_time(&self, time: usize) -> Option<f32> {
        self.frames
            .windows(2)
            .find(|items| items[0].frame <= time && items[1].frame > time)
            .map(|items| items[0].ease(time, &items[1]))
            .or_else(|| self.frames.last().map(|item| item.value))
    }
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq, Display, EnumIter)]
pub enum EaseType {
    Constant,
    Linear,
    EaseOut,
    EaseIn,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq)]
pub struct Keyframe {
    pub frame: usize,
    pub value: f32,
    pub function: EaseType,
}

impl Keyframe {
    fn ease(&self, time: usize, end: &Keyframe) -> f32 {
        let duration = end.frame - self.frame;
        let initial = time - self.frame;
        let value = if duration == 0 {
            1.0
        } else {
            initial as f32 / duration as f32
        };

        let value = match self.function {
            EaseType::Constant => 0.0,
            EaseType::Linear => value,
            EaseType::EaseIn => value.powf(2.0),
            EaseType::EaseOut => (2.0 - value) * value,
        };

        self.value * (1.0 - value) + end.value * value
    }
}

impl Default for Keyframe {
    fn default() -> Self {
        Self {
            frame: 0,
            value: 0.0,
            function: EaseType::Constant,
        }
    }
}
