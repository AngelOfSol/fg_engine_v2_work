use crate::{
    character::state::components::StateType,
    roster::{
        character::{
            data::Data,
            typedefs::{Character, Timed},
        },
        hit_info::ComboEffect,
        OpponentState,
    },
};

use super::PlayerState;

impl<C: Character> PlayerState<C> {
    pub fn handle_rebeat_data(&mut self, data: &Data<C>) {
        if !matches!(data.get(self).state_type, StateType::Attack) {
            self.rebeat_chain.clear();
        }
    }

    pub fn handle_smp(&mut self, opponent: &OpponentState) {
        if !opponent.in_hitstun {
            self.smp.reset();
        }
    }
    pub fn handle_combo_state(
        &mut self,
        last_combo_state: &mut Option<Timed<ComboEffect>>,
        data: &Data<C>,
    ) {
        let state_data = data.get(self);
        if !matches!(
            state_data.state_type,
            StateType::Hitstun | StateType::Blockstun
        ) {
            self.current_combo = None;
        }

        if self.current_combo.is_some() {
            *last_combo_state = Some(Timed {
                time: 30,
                id: self.current_combo.clone().unwrap(),
            });
        }
        if last_combo_state.is_some() && self.current_combo.is_none() {
            let Timed { time, .. } = last_combo_state.as_mut().unwrap();
            *time -= 1;
            if *time == 0 {
                *last_combo_state = None;
            }
        }
    }
}
