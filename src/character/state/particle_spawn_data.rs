use crate::typedefs::collision::Vec2;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use strum_macros::{Display, EnumIter};

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Display, EnumIter, Copy, Hash)]

pub enum GlobalParticle {
    SuperJump,
}

impl<Id> From<GlobalParticle> for ParticlePath<Id> {
    fn from(value: GlobalParticle) -> Self {
        Self::Global(value)
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy)]
#[serde(untagged)]
pub enum ParticlePath<Id> {
    Local(Id),
    Global(GlobalParticle),
}

impl<Id: std::hash::Hash + Eq> ParticlePath<Id> {
    pub fn get<'a, T>(
        &self,
        local: &'a HashMap<Id, T>,
        global: &'a HashMap<GlobalParticle, T>,
    ) -> &'a T {
        match self {
            Self::Local(id) => local.get(id).unwrap(),
            Self::Global(id) => global.get(id).unwrap(),
        }
    }
}

impl<Id: std::fmt::Display> std::fmt::Display for ParticlePath<Id> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Local(id) => write!(f, "{}", &id),
            Self::Global(id) => write!(f, "global::{}", &id),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct ParticleSpawn<Id> {
    pub particle_id: ParticlePath<Id>,
    pub frame: usize,
    pub offset: Vec2,
}

impl From<String> for ParticlePath<String> {
    fn from(value: String) -> Self {
        Self::Local(value)
    }
}

impl ParticleSpawn<String> {
    pub fn new(particle_id: ParticlePath<String>) -> Self {
        Self {
            particle_id,
            frame: 0,
            offset: Vec2::zeros(),
        }
    }
}
