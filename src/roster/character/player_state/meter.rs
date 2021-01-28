use crate::roster::character::{data::Data, typedefs::Character};

use super::PlayerState;

impl<C: Character> PlayerState<C> {
    pub fn update_meter(&mut self, data: &Data<C>) {
        let state_data = data.get(self);

        self.meter -= state_data.flags.meter_cost;

        self.meter = 0.max(200_00.min(self.meter))
    }

    pub fn update_lockout(&mut self) {
        self.lockout -= 1;
        self.lockout = 0.max(self.lockout);
    }
}
