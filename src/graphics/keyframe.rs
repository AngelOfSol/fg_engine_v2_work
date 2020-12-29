use crate::{
    timeline::{Surrounding, Timeline},
    typedefs::graphics::{Matrix4, Vec3},
};
use inspect_design::Inspect;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Display, EnumIter, Inspect)]
pub enum Coordinates {
    Polar,
    Cartesian,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Inspect)]
pub struct Modifiers {
    pub coord_type: Coordinates,
    pub rotation: Keyframes,
    pub scale: [Keyframes; 2],
    pub coords: [Keyframes; 2],
    pub alpha: Keyframes,
    pub value: Keyframes,
}

impl Default for Modifiers {
    fn default() -> Self {
        let percentage_default = Keyframes::with_data(
            vec![(
                0,
                Keyframe {
                    value: 1.0,
                    ..Default::default()
                },
            )],
            1,
        )
        .unwrap();
        Self {
            coord_type: Coordinates::Cartesian,
            coords: Default::default(),
            rotation: Default::default(),
            scale: [percentage_default.clone(), percentage_default.clone()],
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

impl Modifiers {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_duration(&mut self, duration: usize) {
        self.rotation.set_duration(duration);
        self.alpha.set_duration(duration);
        self.value.set_duration(duration);

        for scale in self.scale.iter_mut() {
            scale.set_duration(duration);
        }
        for coords in self.coords.iter_mut() {
            coords.set_duration(duration);
        }
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
            .map(|coord| coord.get_eased(time).unwrap_or(0.0));

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
            .map(|scale| scale.get_eased(time).unwrap_or(0.0));

        let (x, y) = (scale.next().unwrap(), scale.next().unwrap());

        let scale = Matrix4::new_nonuniform_scaling(&Vec3::new(x, y, 0.0));

        translate * scale * rotation
    }
}

pub type Keyframes = Timeline<Keyframe>;

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
