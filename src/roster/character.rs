pub mod data;
pub mod player_state;
pub mod smp;
pub mod typedefs;

use super::hit_info::ComboEffect;
use crate::game_match::sounds::{SoundPath, SoundRenderer};
use data::Data;
use hecs::World;
use player_state::PlayerState;
use std::cell::RefCell;
use typedefs::{Character, Timed};

pub struct Player<C: Character> {
    pub data: Data<C>,
    pub world: World,
    pub state: PlayerState<C>,
    pub ui_state: UiState,
    pub sound_renderer: SoundRenderer<SoundPath<C::Sound>>,
}

pub struct UiState {
    pub last_combo_state: Option<Timed<ComboEffect>>,
    pub combo_text: RefCell<Option<ggez::graphics::Text>>,
}
