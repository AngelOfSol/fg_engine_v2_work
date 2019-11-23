use crate::typedefs::StateId;

pub trait GenericParticleId: StateId + Clone + Copy {
    const ON_HIT: Self;
}
