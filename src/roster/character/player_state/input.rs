use super::PlayerState;
use crate::{
    character::{
        command::{Effect, Requirement},
        state::components::StateType,
    },
    roster::{
        character::{
            data::Data,
            typedefs::{state::StateConsts, Character, Timed},
        },
        AllowedCancel,
    },
};
use fg_input::{button::button_set, read_inputs, InputState};

impl<C: Character> PlayerState<C> {
    pub fn handle_input(&mut self, data: &Data<C>, input: &[InputState]) {
        let Timed { time, id } = self.current_state;
        let state_data = data.get(self);

        self.current_state = {
            let inputs = read_inputs(
                input.iter().rev(),
                self.facing,
                state_data.state_type.buffer_window(),
            );
            // TODO move to better handling for this
            // inputs should be able to detect releases
            if id == C::State::FLY {
                if input
                    .last()
                    .unwrap()
                    .button_set()
                    .is_superset(button_set::E)
                {
                    Timed { time, id }
                } else {
                    Timed {
                        time: 0,
                        id: C::State::FLY_END,
                    }
                }
            } else {
                let possible_new_move = inputs
                    .iter()
                    .flat_map(|input| data.input_map.get(input))
                    .flat_map(|command_ids| command_ids.iter())
                    .map(|command_id| (*command_id, &data.command_map[command_id]))
                    .find(|(command_id, command)| {
                        command.reqs.iter().all(|requirement| match requirement {
                            Requirement::HasAirActions => self.air_actions > 0,
                            Requirement::InBlockstun => {
                                state_data.state_type == StateType::Blockstun
                            }
                            Requirement::NotLockedOut => self.lockout == 0,
                            Requirement::CanCancel(new_state_type) => {
                                let is_self = command.state_id == id;
                                let is_allowed_cancel =
                                    match self.allowed_cancels {
                                        AllowedCancel::Hit => {
                                            state_data.cancels.hit.contains(new_state_type)
                                        }
                                        AllowedCancel::Block => {
                                            state_data.cancels.block.contains(new_state_type)
                                        }
                                        AllowedCancel::Always => false,
                                    } || state_data.cancels.always.contains(new_state_type);

                                let can_rebeat = !self.rebeat_chain.contains(&command_id);

                                ((!is_self && can_rebeat)
                                    || (is_self && state_data.cancels.self_gatling))
                                    && is_allowed_cancel
                            }
                            Requirement::Meter(value) => self.meter >= *value,
                            Requirement::Spirit(value) => self.spirit_gauge >= *value,
                            Requirement::Airborne => state_data.flags.airborne,
                            Requirement::Grounded => !state_data.flags.airborne,
                            Requirement::CancelFrom(previous_state) => previous_state == &id,
                            Requirement::NoCancelFrom(previous_state) => previous_state != &id,
                        })
                    });

                let ret = possible_new_move
                    .as_ref()
                    .map(|(_, command)| Timed {
                        time: command.frame,
                        id: command.state_id,
                    })
                    .unwrap_or(Timed { time, id });

                if let Some((command_id, command)) = possible_new_move {
                    for effect in command.effects.iter() {
                        match effect {
                            Effect::UseAirAction => {
                                self.air_actions = self.air_actions.saturating_sub(1)
                            }
                            Effect::UseMeter(meter) => self.meter -= meter,
                            Effect::RefillSpirit => {
                                self.spirit_gauge = data.properties.max_spirit_gauge
                            }
                            Effect::FlipFacing => {
                                self.facing = self.facing.invert();
                            }
                        }
                    }
                    self.allowed_cancels = AllowedCancel::Always;
                    self.stun = None;
                    self.last_hit_using = None;
                    self.rebeat_chain.insert(command_id);
                    self.most_recent_command = Timed {
                        id: command_id,
                        time: self.most_recent_command.time.checked_add(1).unwrap(),
                    };
                }

                ret
            }
        };
    }
}
