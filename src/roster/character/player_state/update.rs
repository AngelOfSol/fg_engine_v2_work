use fg_input::InputState;
use hecs::World;

use super::PlayerState;
use crate::{
    character::state::components::GlobalGraphicMap,
    game_match::PlayArea,
    game_object::{
        constructors::{Construct, Constructor},
        properties::{CharacterAttack, PropertyType, TryAsRef},
    },
    roster::{
        character::{
            data::Data,
            typedefs::{state::StateConsts, Character, Timed},
        },
        hit_info::ComboEffect,
        AllowedCancel, OpponentState,
    },
};

impl<C: Character> PlayerState<C>
where
    Constructor: Construct<C>,
    PropertyType: TryAsRef<CharacterAttack<C>>,
{
    pub fn handle_expire(&mut self, data: &Data<C>) {
        let Timed { time, id } = self.current_state;

        let state_data = data.get(self);

        self.current_state = if time >= state_data.duration - 1 {
            self.allowed_cancels = AllowedCancel::Always;
            self.last_hit_using = None;
            self.rebeat_chain.clear();

            if id == C::State::HIT_GROUND && self.dead {
                Timed {
                    time: 0,
                    id: C::State::DEAD,
                }
            } else {
                Timed {
                    time: state_data.on_expire.frame,
                    id: state_data.on_expire.state_id,
                }
            }
        } else {
            Timed { time: time + 1, id }
        };
    }

    #[allow(clippy::too_many_arguments)]
    pub fn update_frame_mut(
        &mut self,
        world: &mut World,
        last_combo_state: &mut Option<Timed<ComboEffect>>,
        data: &Data<C>,
        input: &[InputState],
        opponent: OpponentState,
        play_area: &PlayArea,
        global_graphics: &GlobalGraphicMap,
    ) {
        if self.hitstop > 0 {
            self.hitstop -= 1;
        } else {
            self.handle_expire(data);
            self.handle_rebeat_data(data);
            self.handle_hitstun(data);
            self.handle_input(data, input);
            self.update_velocity(data, play_area);
            self.update_position(data, play_area);
            self.update_sound(data);
        }
        self.handle_combo_state(last_combo_state, data);
        self.update_spirit(data);
        self.handle_smp(&opponent);
        self.update_lockout();
        self.update_meter(data);
        self.update_objects(world, data, global_graphics);
        self.spawn_objects(world, data);
        self.sound_state.update();
        self.hitstop = i32::max(0, self.hitstop);
    }

    pub fn update_no_input(
        &mut self,
        world: &mut World,
        last_combo_state: &mut Option<Timed<ComboEffect>>,
        data: &Data<C>,
        play_area: &PlayArea,
        global_graphics: &GlobalGraphicMap,
    ) {
        if self.hitstop > 0 {
            self.hitstop -= 1;
        } else {
            self.handle_expire(data);
            self.handle_hitstun(data);
            self.update_velocity(data, play_area);
            self.update_position(data, play_area);
            self.update_sound(data);
        }
        self.handle_combo_state(last_combo_state, data);
        self.update_spirit(data);
        self.update_lockout();
        self.update_objects(world, data, global_graphics);
        self.spawn_objects(world, data);
        self.sound_state.update();
        self.hitstop = i32::max(0, self.hitstop);
    }

    pub fn update_cutscene(&mut self, data: &Data<C>, play_area: &PlayArea) {
        if data.get_next(self).flags.cutscene {
            self.handle_expire(data);
        }
        self.validate_position(data, play_area);
        self.sound_state.update();
    }
}
