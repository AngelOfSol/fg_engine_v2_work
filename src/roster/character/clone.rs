use hecs::CloneRegistry;

use crate::{
    character::state::components::GlobalGraphic,
    game_object::{
        properties::typedefs::TotalHits,
        state::{
            BulletHp, ExpiresAfterAnimation, GrazeResistance, HasHitbox, HitDelay, Hitstop,
            ObjectAttack, Position, Rotation, Timer, Velocity,
        },
    },
    input::Facing,
};

use super::typedefs::Character;

pub fn registry_for<C: Character>() -> CloneRegistry {
    CloneRegistry::default()
        .register::<C::Graphic>()
        .register::<Timer>()
        .register::<ExpiresAfterAnimation>()
        .register::<HasHitbox>()
        .register::<Facing>()
        .register::<BulletHp>()
        .register::<ObjectAttack<C>>()
        .register::<TotalHits>()
        .register::<GrazeResistance>()
        .register::<Rotation>()
        .register::<Velocity>()
        .register::<Position>()
        .register::<GlobalGraphic>()
        .register::<Hitstop>()
        .register::<HitDelay>()
        .register::<C::ObjectData>()
}
