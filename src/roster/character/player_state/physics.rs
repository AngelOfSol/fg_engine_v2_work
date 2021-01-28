use crate::{
    character::{components::GroundAction, state::components::StateType},
    game_match::PlayArea,
    roster::character::{
        data::Data,
        typedefs::{state::StateConsts, Character, Timed},
    },
};
use fg_datastructures::math::collision;
use fg_input::Facing;

use super::PlayerState;

const MAX_FALLING_VELOCITY: i32 = -8_00;

impl<C: Character> PlayerState<C> {
    pub fn update_velocity(&mut self, data: &Data<C>, play_area: &PlayArea) {
        let state_data = data.get(self);

        let base_velocity = if state_data.flags.reset_velocity {
            collision::Vec2::zeros()
        } else {
            self.velocity
        };

        // we only run gravity if the move doesn't want to reset velocity, because that [resetting velocity] means the move has a trajectory in mind
        let gravity = if state_data.flags.gravity && state_data.flags.airborne {
            collision::Vec2::new(0_00, -0_20)
        } else {
            collision::Vec2::zeros()
        };
        let friction = if !state_data.flags.airborne || self.in_corner(data, play_area) {
            collision::Vec2::new(
                -i32::min(base_velocity.x.abs(), state_data.flags.friction)
                    * i32::signum(base_velocity.x),
                0_00,
            )
        } else {
            collision::Vec2::zeros()
        };

        let accel = self.facing.fix(state_data.flags.accel);
        self.velocity = base_velocity + accel + friction + gravity;
        self.velocity.y = self.velocity.y.max(MAX_FALLING_VELOCITY);
    }

    pub fn update_position(&mut self, data: &Data<C>, play_area: &PlayArea) {
        let state_data = data.get(self);

        self.position += self.velocity;

        // handle landing
        if state_data.flags.airborne
            && self.position.y - state_data.hitboxes.collision.half_size.y <= -4
        {
            let mut reset_hitstun = true;
            let mut reset_velocity = true;
            self.current_state = if state_data.state_type == StateType::Hitstun {
                let combo = self.current_combo.as_mut().unwrap();
                match combo.ground_action {
                    GroundAction::Knockdown => Timed {
                        time: 0,
                        id: C::State::HIT_GROUND,
                    },
                    GroundAction::GroundSlam => {
                        self.velocity.y *= -90;
                        self.velocity.y /= 100;
                        combo.ground_action = GroundAction::Knockdown;
                        reset_hitstun = false;
                        reset_velocity = false;

                        Timed {
                            time: 0,
                            id: C::State::AIR_HITSTUN,
                        }
                    }
                    GroundAction::OnTheGround => Timed {
                        time: 0,
                        id: C::State::HIT_GROUND,
                    },
                }
            } else {
                Timed {
                    time: 0,
                    id: C::State::STAND,
                }
            };
            if reset_hitstun {
                self.stun = None;
            }
            if reset_velocity {
                self.velocity = collision::Vec2::zeros();
            }
            self.position.y = state_data.hitboxes.collision.half_size.y;
            self.air_actions = data.properties.max_air_actions;
        }

        self.validate_position(data, play_area);
    }

    pub fn validate_position(&mut self, data: &Data<C>, play_area: &PlayArea) {
        let state_data = data.get(self);
        // handle stage sides
        if i32::abs(self.position.x)
            > play_area.width / 2 - state_data.hitboxes.collision.half_size.x
        {
            self.position.x = i32::signum(self.position.x)
                * (play_area.width / 2 - state_data.hitboxes.collision.half_size.x);
        }

        // if not airborne, make sure the character is locked to the ground properly
        if !state_data.flags.airborne {
            self.position.y = state_data.hitboxes.collision.half_size.y;
        }
    }

    pub fn in_corner(&self, data: &Data<C>, play_area: &PlayArea) -> bool {
        let collision = data.get(self).hitboxes.collision;
        i32::abs(self.position.x) >= play_area.width / 2 - collision.half_size.x
    }

    pub fn apply_pushback(&mut self, data: &Data<C>, force: collision::Int) {
        if !data.get(self).flags.airborne {
            self.position.x += force;
        }
    }
    pub fn get_pushback(&self, data: &Data<C>, play_area: &PlayArea) -> collision::Int {
        let state_data = data.get(self);

        if matches!(
            state_data.state_type,
            StateType::Hitstun | StateType::Blockstun
        ) && self.in_corner(data, play_area)
            && self.hitstop == 0
            && self.should_pushback
        {
            -self.velocity.x
        } else {
            0
        }
    }

    pub fn handle_refacing(&mut self, data: &Data<C>, other_player: collision::Int) {
        if data.get(self).flags.allow_reface {
            self.facing = if self.position.x > other_player && self.facing == Facing::Right {
                Facing::Left
            } else if self.position.x < other_player && self.facing == Facing::Left {
                Facing::Right
            } else {
                self.facing
            }
        }
    }
}
