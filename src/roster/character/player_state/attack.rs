use std::borrow::Cow;

use crate::{
    character::components::AttackInfo,
    roster::character::{
        data::Data,
        typedefs::{Character, HitId},
    },
};

use super::PlayerState;

pub mod deal_hit;
pub mod take_hit;
pub mod would_be_hit;

impl<C: Character> PlayerState<C> {
    pub fn get_attack_data<'data>(&self, data: &'data Data<C>) -> Option<Cow<'data, AttackInfo>> {
        data.get(self).hitboxes.hitbox.as_ref().and_then(|hitbox| {
            if Some(HitId {
                id: hitbox.data_id,
                hitbox_id: hitbox.id,
            }) != self.last_hit_using
            {
                if self.smp.should_smp(self.most_recent_command) {
                    let mut owned = data.attacks[&hitbox.data_id].clone();

                    owned.on_hit.limit_cost *= 2;

                    Some(Cow::Owned(owned))
                } else {
                    Some(Cow::Borrowed(&data.attacks[&hitbox.data_id]))
                }
            } else {
                None
            }
        })
    }
}
