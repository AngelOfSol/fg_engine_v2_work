use std::borrow::Cow;

use hecs::{Entity, World};

use crate::{
    character::components::AttackInfo,
    game_object::{
        properties::{CharacterAttack, ObjectHitboxSet, PropertyType, TryAsRef},
        state::{Hitbox, ObjectAttack, Timer},
    },
    input::Facing,
    roster::character::{
        data::Data,
        typedefs::{Character, HitId},
    },
};

use super::PlayerState;

pub mod deal_hit;
pub mod take_hit;
pub mod would_be_hit;

impl<C: Character> PlayerState<C>
where
    PropertyType: TryAsRef<CharacterAttack<C>>,
{
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

    pub fn get_attack_data_entity<'data>(
        &self,
        data: &'data Data<C>,
        world: &World,
        entity: Entity,
    ) -> Option<(Facing, Cow<'data, AttackInfo>)> {
        let mut query = world
            .query_one::<(
                Option<&Timer>,
                &ObjectAttack<C>,
                &Hitbox<C::ObjectData>,
                &Facing,
            )>(entity)
            .unwrap();

        let (timer, attack, hitboxes, facing) = query.get()?;

        let timer = timer.map(|t| t.0).unwrap_or_default();

        let hitboxes = &data.instance.get::<ObjectHitboxSet>(hitboxes.0).unwrap()[timer];

        let attack_id = data
            .instance
            .get::<CharacterAttack<C>>(attack.id)?
            .get(timer)
            .1;

        if attack.last_hit_using
            != Some(HitId {
                hitbox_id: hitboxes.id,
                id: *attack_id,
            })
        {
            if self.smp.should_smp(attack.command) {
                let mut owned = data.attacks[&attack_id].clone();

                owned.on_hit.limit_cost *= 2;

                Some((*facing, Cow::Owned(owned)))
            } else {
                Some((*facing, Cow::Borrowed(&data.attacks[&attack_id])))
            }
        } else {
            None
        }
    }
}
