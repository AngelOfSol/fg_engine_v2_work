use hecs::EntityBuilder;
use inspect_design::Inspect;
use serde::{Deserialize, Serialize};
use yuyuko::ObjectData;

use crate::{
    game_object::{
        constructors::{Construct, ConstructError},
        properties::typedefs::TotalHits,
        state::{
            BulletHp, ExpiresAfterAnimation, GrazeResistance, Hitbox, MultiHitType, ObjectAttack,
            Timer,
        },
    },
    roster::{
        character::{data::Data, player_state::PlayerState},
        yuyuko::{self, Graphic, YuyukoType},
    },
};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Default, Inspect)]
pub struct SpawnGhost {}

impl Construct<YuyukoType> for SpawnGhost {
    fn construct_on_to<'constructor, 'builder>(
        &'constructor self,
        builder: &'builder mut EntityBuilder,
        context: &PlayerState<YuyukoType>,
        data: &Data<YuyukoType>,
    ) -> Result<&'builder mut EntityBuilder, ConstructError> {
        const OBJECT_KEY: ObjectData = ObjectData::Ghost;

        builder.add(Graphic::Ghost);

        builder.add(Timer(0));
        builder.add(ExpiresAfterAnimation);
        builder.add(OBJECT_KEY);
        builder.add(Hitbox);
        builder.add(context.facing);

        builder.add(
            *data
                .instance
                .get::<BulletHp>(OBJECT_KEY)
                .ok_or(ConstructError::MissingRequiredData)?,
        );

        builder.add(ObjectAttack::<YuyukoType> {
            command: context.most_recent_command,
            multi_hit: data
                .instance
                .get::<TotalHits>(OBJECT_KEY)
                .map(|TotalHits(hits)| MultiHitType::RemainingHits(*hits))
                .unwrap_or(MultiHitType::LastHitUsing(None)),
        });

        if let Some(graze_resistance) = data.instance.get::<GrazeResistance>(OBJECT_KEY) {
            builder.add(*graze_resistance);
        }

        Ok(builder)
    }
}
