use crate::{
    character::state::components::GlobalGraphic,
    game_object::{
        properties::typedefs::TotalHits,
        state::{
            BulletHp, ExpiresAfterAnimation, GrazeResistance, HasHitbox, HitDelay, Hitstop,
            ObjectAttack, Position, Rotation, Timer, Velocity,
        },
    },
};
use fg_input::Facing;
use hecs::clone::CloneRegistry;

use super::typedefs::Character;

pub fn default_registry() -> CloneRegistry {
    CloneRegistry::default()
        .register::<Timer>()
        .register::<ExpiresAfterAnimation>()
        .register::<HasHitbox>()
        .register::<Facing>()
        .register::<BulletHp>()
        .register::<TotalHits>()
        .register::<GrazeResistance>()
        .register::<Rotation>()
        .register::<Velocity>()
        .register::<Position>()
        .register::<GlobalGraphic>()
        .register::<Hitstop>()
        .register::<HitDelay>()
}

pub fn registry_for<C: Character>() -> CloneRegistry {
    default_registry()
        .register::<ObjectAttack<C>>()
        .register::<C::ObjectData>()
        .register::<C::Graphic>()
}
