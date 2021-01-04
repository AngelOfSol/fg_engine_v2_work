use super::{
    player_state::PlayerState,
    typedefs::{Character, CharacterState, CharacterStateInstant},
};
use crate::{
    character::{
        command::Command,
        components::{AttackInfo, Properties},
    },
    game_match::sounds::SoundList,
    graphics::animation_group::AnimationGroup,
    input::Input,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Serialize, Deserialize)]
pub struct Data<C: Character> {
    pub states: HashMap<C::State, CharacterState<C>>,
    pub attacks: HashMap<C::Attack, AttackInfo>,
    pub properties: Properties,
    #[serde(skip)]
    pub sounds: SoundList<C::Sound>,
    pub graphics: HashMap<C::Graphic, AnimationGroup>,
    pub input_map: HashMap<Input, Vec<C::Command>>,
    pub command_map: HashMap<C::Command, Command<C::State>>,
    pub state_graphics_map: HashMap<C::State, C::Graphic>,
    #[serde(default)]
    pub other: C::StaticData,
}

impl<C: Character> Data<C> {
    pub fn get<'this, 'state>(
        &'this self,
        state: &'state PlayerState<C>,
    ) -> CharacterStateInstant<'this, C> {
        self.states[&state.current_state.id].get(state.current_state.time)
    }
    pub fn get_next<'this, 'state>(
        &'this self,
        state: &'state PlayerState<C>,
    ) -> CharacterStateInstant<'this, C> {
        self.states[&state.current_state.id].get(state.current_state.time + 1)
    }
}
