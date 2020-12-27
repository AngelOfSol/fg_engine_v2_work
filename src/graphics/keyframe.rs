use crate::{
    timeline::{Surrounding, Timeline},
    typedefs::graphics::{Matrix4, Vec3},
};
use inspect_design::Inspect;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter};

pub type Modifiers = ModifiersOld;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Display, EnumIter, Inspect)]
pub enum Coordinates {
    Polar,
    Cartesian,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Inspect)]
pub struct ModifiersOld {
    #[serde(deserialize_with = "new_serde::deserialize")]
    pub rotation: Keyframes,
    pub scale: [OldKeyframes; 2],
    pub coords: [OldKeyframes; 2],
    pub coord_type: Coordinates,
    pub alpha: OldKeyframes,
    pub value: OldKeyframes,
}

impl Default for ModifiersOld {
    fn default() -> Self {
        let default = OldKeyframes::new(Default::default());
        let percentage_default = OldKeyframes::new(OldKeyframe {
            value: 1.0,
            ..Default::default()
        });
        Self {
            coord_type: Coordinates::Cartesian,
            coords: [default.clone(), default],
            scale: [percentage_default.clone(), percentage_default.clone()],
            rotation: Default::default(),
            alpha: percentage_default.clone(),
            value: percentage_default,
        }
    }
}

fn get_value(value: Surrounding<(usize, &Keyframe)>, time: usize) -> Option<f32> {
    match value {
        Surrounding::None => None,
        Surrounding::Pair { start, end } => {
            let duration = end.0 - start.0;
            let initial = time - start.0;
            let value = if duration == 0 {
                1.0
            } else {
                initial as f32 / duration as f32
            };

            let value = match start.1.function {
                EaseType::Constant => 0.0,
                EaseType::Linear => value,
                EaseType::EaseIn => value.powf(2.0),
                EaseType::EaseOut => (2.0 - value) * value,
            };

            Some(start.1.value * (1.0 - value) + end.1.value * value)
        }
        Surrounding::End { start, .. } => Some(start.1.value),
    }
}

pub trait KeyframeExt {
    fn get_eased(&self, time: usize) -> Option<f32>;
}

impl KeyframeExt for Keyframes {
    fn get_eased(&self, time: usize) -> Option<f32> {
        get_value(self.surrounding(time), time)
    }
}

impl ModifiersOld {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_matrix(&self, time: usize) -> Matrix4 {
        let rotation = Matrix4::new_rotation(Vec3::new(
            0.0,
            0.0,
            self.rotation.get_eased(time).unwrap_or(0.0),
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

pub type Keyframes = Timeline<Keyframe>;

mod new_serde {
    use serde::{Deserialize, Deserializer};

    use super::{Keyframe, Keyframes, OldKeyframes};

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Keyframes, D::Error> {
        let value = OldKeyframes::deserialize(deserializer)?;
        let mut duration = 0;
        Ok(Keyframes::with_data(
            value
                .frames
                .into_iter()
                .map(|item| {
                    let value = Keyframe {
                        value: item.value,
                        function: item.function,
                    };
                    duration = item.frame;

                    (item.frame, value)
                })
                .collect(),
            duration,
        )
        .unwrap())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Inspect)]
pub struct OldKeyframes {
    pub frames: Vec<OldKeyframe>,
}

impl OldKeyframes {
    pub fn new(frame: OldKeyframe) -> Self {
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

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq, Display, EnumIter, Inspect)]
pub enum EaseType {
    Constant,
    Linear,
    EaseOut,
    EaseIn,
}

impl Default for EaseType {
    fn default() -> Self {
        Self::Constant
    }
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Inspect, Default)]
pub struct Keyframe {
    pub value: f32,
    pub function: EaseType,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Inspect)]
pub struct OldKeyframe {
    pub frame: usize,
    pub value: f32,
    pub function: EaseType,
}

impl OldKeyframe {
    fn ease(&self, time: usize, end: &OldKeyframe) -> f32 {
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

impl Default for OldKeyframe {
    fn default() -> Self {
        Self {
            frame: 0,
            value: 0.0,
            function: EaseType::Constant,
        }
    }
}
