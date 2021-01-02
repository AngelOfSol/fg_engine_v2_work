use super::{
    player_state::PlayerState,
    typedefs::{Character, CharacterState, CharacterStateInstant},
};
use crate::{
    character::{
        command::Command,
        components::{AttackInfo, Properties},
    },
    game_match::sounds::{SoundList, SoundPath},
    graphics::animation_group::AnimationGroup,
    input::Input,
};
use std::collections::HashMap;

pub struct Data<C: Character> {
    pub states: HashMap<C::State, CharacterState<C>>,
    pub attacks: HashMap<C::Attack, AttackInfo>,
    pub properties: Properties,
    pub sounds: SoundList<SoundPath<C::Sound>>,
    pub graphics: HashMap<C::Command, AnimationGroup>,
    pub input_map: HashMap<Input, Vec<C::Command>>,
    pub command_map: HashMap<C::Command, Command<C::State>>,
    pub state_graphics_map: HashMap<C::State, C::Graphic>,
    pub marker: C::StaticData,
}

impl<C: Character> Data<C> {
    pub fn get<'this, 'state>(
        &'this self,
        state: &'state PlayerState<C>,
    ) -> CharacterStateInstant<'this, C> {
        self.states[&state.current_state.id].get(state.current_state.time)
    }
}
