mod position;
mod templates;
mod traits;
mod tuples;
pub mod typedefs;

use super::state::Position;
use crate::roster::yuyuko;
use crate::{
    character::state::components::GlobalGraphic,
    roster::{
        character::{data::Data, player_state::PlayerState},
        yuyuko::YuyukoType,
    },
};
use hecs::EntityBuilder;
use inspect_design::Inspect;
pub use position::*;
use serde::{Deserialize, Serialize};
pub use traits::Construct;
use typedefs::ParticleData;
use yuyuko::object::SpawnButterfly;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum ConstructError {
    MissingRequiredComponent,
    MissingRequiredData,
}

// this should be implemented for every character for every constructor
// it should
#[derive(Debug, Clone, Serialize, Deserialize, Inspect, Eq, PartialEq)]
#[no_label]
pub enum Constructor {
    Position(Position),
    GlobalParticle(ParticleData<GlobalGraphic>),
    YuyukoParticle(ParticleData<yuyuko::Graphic>),
    Butterfly(SpawnButterfly),
}

impl Default for Constructor {
    fn default() -> Self {
        Self::Position(Default::default())
    }
}

impl Construct<YuyukoType> for Constructor {
    fn construct_on_to<'constructor, 'builder>(
        &'constructor self,
        builder: &'builder mut EntityBuilder,
        context: &PlayerState<YuyukoType>,
        data: &Data<YuyukoType>,
    ) -> Result<&'builder mut EntityBuilder, ConstructError> {
        match self {
            Self::Position(v) => v.construct_on_to(builder, context, data),
            Self::GlobalParticle(v) => v.construct_on_to(builder, context, data),
            Self::YuyukoParticle(v) => v.construct_on_to(builder, context, data),
            Self::Butterfly(v) => v.construct_on_to(builder, context, data),
        }
    }
}
