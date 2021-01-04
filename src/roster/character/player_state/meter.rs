use crate::{
    character::state::components::StateType,
    roster::character::{data::Data, typedefs::Character},
    typedefs::collision,
};

use super::PlayerState;

impl<C: Character> PlayerState<C> {
    pub fn update_meter(&mut self, data: &Data<C>, opponent_position: collision::Vec2) {
        let state_data = data.get(self);

        self.meter -= state_data.flags.meter_cost;

        if self.meter < 50_00 {
            self.meter += 5;
        } else if self.meter < 100_00 {
            self.meter += 2;
        } else if self.meter > 150_00 {
            self.meter -= 5;
        } else if self.meter > 100_00 {
            self.meter -= 2;
            // clamp to 100 to make sure we don't wobble around 100
            self.meter = self.meter.max(100_00);
        }

        let dir = (opponent_position - self.position).x.signum();
        let facing_opponent = dir == self.facing.collision_multiplier().x;
        if matches!(state_data.state_type, StateType::Movement) && facing_opponent {
            // only apply bonus/penalty if we're facing the opponent
            // fly is the exception to this because it reorients our facing direction
            // TODO stop having fly reorient facing direction

            let speed = self.velocity.x.abs();
            let bonus_meter = 50;
            // apply bonus/penalty based on speed
            if dir == self.velocity.x.signum() {
                self.meter += bonus_meter.min(bonus_meter * speed / 10_00);
            } else if -dir == self.velocity.x.signum() {
                self.meter -= bonus_meter.min(bonus_meter * speed / 10_00);
            }
        }

        self.meter = 0.max(200_00.min(self.meter))
    }

    pub fn update_lockout(&mut self) {
        self.lockout -= 1;
        self.lockout = 0.max(self.lockout);
    }
}
