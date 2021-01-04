use super::templates::{ConstructDefault, ConstructId};
use crate::game_object::state::{ExpiresAfterAnimation, Timer};

pub type ParticleData<T> = (
    ConstructId<T>,
    ConstructDefault<ExpiresAfterAnimation>,
    ConstructDefault<Timer>,
);
