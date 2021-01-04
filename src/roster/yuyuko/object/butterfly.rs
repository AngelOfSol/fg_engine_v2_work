use hecs::EntityBuilder;
use inspect_design::Inspect;
use serde::{Deserialize, Serialize};

use crate::{
    game_object::{
        constructors::{Construct, ConstructError},
        state::{Rotation, Timer, Velocity},
    },
    roster::{
        character::{data::Data, player_state::PlayerState},
        yuyuko::{Graphic, YuyukoType},
    },
    typedefs::collision,
};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Inspect)]
pub enum ButterflyColor {
    Purple,
    Green,
    Teal,
    Red,
}

impl Default for ButterflyColor {
    fn default() -> Self {
        Self::Purple
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Default, Inspect)]
pub struct SpawnButterfly {
    color: ButterflyColor,
    velocity: collision::Vec2,
}

impl Construct<YuyukoType> for SpawnButterfly {
    fn construct_on_to<'constructor, 'builder>(
        &'constructor self,
        builder: &'builder mut EntityBuilder,
        context: &PlayerState<YuyukoType>,
        _data: &Data<YuyukoType>,
    ) -> Result<&'builder mut EntityBuilder, ConstructError> {
        let velocity = context.facing.fix_collision(self.velocity);
        builder.add(Velocity { value: velocity });
        builder.add(match self.color {
            ButterflyColor::Purple => Graphic::Butterfly1,
            ButterflyColor::Green => Graphic::Butterfly2,
            ButterflyColor::Teal => Graphic::Butterfly3,
            ButterflyColor::Red => Graphic::Butterfly4,
        });
        builder.add(Timer(0));
        builder.add(Rotation(f32::atan2(-velocity.y as f32, velocity.x as f32)));

        Ok(builder)
    }
}
