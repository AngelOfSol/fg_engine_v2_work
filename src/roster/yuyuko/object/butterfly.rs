use hecs::EntityBuilder;
use inspect_design::Inspect;
use serde::{Deserialize, Serialize};
use yuyuko::ObjectData;

use crate::{
    game_object::{
        constructors::{Construct, ConstructError},
        properties::typedefs::Speed,
        state::{BulletHp, Hitbox, Rotation, Timer, Velocity},
    },
    roster::{
        character::{data::Data, player_state::PlayerState},
        yuyuko::{self, Graphic, YuyukoType},
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
        data: &Data<YuyukoType>,
    ) -> Result<&'builder mut EntityBuilder, ConstructError> {
        let angle = f32::atan2(-self.velocity.y as f32, self.velocity.x as f32);
        builder.add(Rotation(angle));
        builder.add(context.facing);

        let velocity = context.facing.fix_collision(self.velocity);
        let angle = f32::atan2(-velocity.y as f32, velocity.x as f32);
        let speed = data
            .instance
            .get::<Speed>(ObjectData::Butterfly)
            .ok_or(ConstructError::MissingRequiredData)?
            .0;

        builder.add(Velocity {
            value: collision::Vec2::new(
                (angle.cos() * speed as f32) as i32,
                (-angle.sin() * speed as f32) as i32,
            ),
        });
        builder.add(match self.color {
            ButterflyColor::Purple => Graphic::Butterfly1,
            ButterflyColor::Green => Graphic::Butterfly2,
            ButterflyColor::Teal => Graphic::Butterfly3,
            ButterflyColor::Red => Graphic::Butterfly4,
        });
        builder.add(Timer(0));
        builder.add(Hitbox(ObjectData::Butterfly));

        builder.add(
            *data
                .instance
                .get::<BulletHp>(ObjectData::Butterfly)
                .ok_or(ConstructError::MissingRequiredData)?,
        );

        Ok(builder)
    }
}
