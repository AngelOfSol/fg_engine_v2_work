use hecs::{Entity, World};

use crate::{
    game_match::sounds::{ChannelName, GlobalSound},
    game_object::{
        constructors::{Construct, Constructor},
        properties::{CharacterAttack, PropertyType, TryAsRef},
    },
    roster::{
        character::{
            data::Data,
            player_state::PlayerState,
            typedefs::{Character, HitId},
        },
        hit_info::HitType,
        AllowedCancel,
    },
};

impl<C: Character> PlayerState<C>
where
    PropertyType: TryAsRef<CharacterAttack<C>>,
    Constructor: Construct<C>,
{
    pub fn deal_hit(&mut self, data: &Data<C>, info: &HitType) {
        match info {
            HitType::Hit => {
                self.sound_state
                    .play_sound(ChannelName::Hit, GlobalSound::Hit.into());
            }
            HitType::GuardCrush => {
                self.sound_state
                    .play_sound(ChannelName::Hit, GlobalSound::GuardCrush.into());
            }
            HitType::CounterHit => {
                self.sound_state
                    .play_sound(ChannelName::Hit, GlobalSound::CounterHit.into());
            }
            HitType::Block => {
                self.sound_state
                    .play_sound(ChannelName::Hit, GlobalSound::Block.into());
            }
            HitType::WrongBlock => {
                self.sound_state
                    .play_sound(ChannelName::Hit, GlobalSound::WrongBlock.into());
            }
            HitType::Graze => {}
        }

        let hitbox = data.get(self).hitboxes.hitbox.as_ref().unwrap();
        let attack_info = &data.attacks[&hitbox.data_id];

        self.meter += match info {
            HitType::Hit => attack_info.on_hit.attacker_meter,
            HitType::GuardCrush => attack_info.on_guard_crush.attacker_meter,
            HitType::CounterHit => attack_info.on_counter_hit.attacker_meter,
            HitType::Graze => attack_info.on_graze.attacker_meter,
            HitType::Block => attack_info.on_block.attacker_meter,
            HitType::WrongBlock => attack_info.on_wrongblock.attacker_meter,
        };
        self.hitstop = match info {
            HitType::Hit => attack_info.on_hit.attacker_stop,
            HitType::GuardCrush => attack_info.on_guard_crush.attacker_stop,
            HitType::CounterHit => attack_info.on_counter_hit.attacker_stop,
            HitType::Graze => 0,
            HitType::Block => attack_info.on_block.attacker_stop,
            HitType::WrongBlock => attack_info.on_wrongblock.attacker_stop,
        };

        self.last_hit_using = Some(HitId {
            hitbox_id: hitbox.id,
            id: hitbox.data_id,
        });

        self.smp.push(self.most_recent_command);

        match info {
            HitType::Hit | HitType::GuardCrush | HitType::CounterHit => {
                self.allowed_cancels = AllowedCancel::Hit;
            }
            HitType::Graze => {}
            HitType::Block | HitType::WrongBlock => {
                self.allowed_cancels = AllowedCancel::Block;
            }
        }
    }

    pub fn deal_hit_entity(
        &mut self,
        world: &mut World,
        data: &Data<C>,
        entity: Entity,
        info: &HitType,
    ) {
        match info {
            HitType::Hit => {
                self.sound_state
                    .play_sound(ChannelName::Hit, GlobalSound::Hit.into());
            }
            HitType::GuardCrush => {
                self.sound_state
                    .play_sound(ChannelName::Hit, GlobalSound::GuardCrush.into());
            }
            HitType::CounterHit => {
                self.sound_state
                    .play_sound(ChannelName::Hit, GlobalSound::CounterHit.into());
            }
            HitType::Block => {
                self.sound_state
                    .play_sound(ChannelName::Hit, GlobalSound::Block.into());
            }
            HitType::WrongBlock => {
                self.sound_state
                    .play_sound(ChannelName::Hit, GlobalSound::WrongBlock.into());
            }
            HitType::Graze => {}
        }

        let attack_info = self.get_attack_data_entity(data, world, entity).unwrap().1;

        self.meter += match info {
            HitType::Hit => attack_info.on_hit.attacker_meter,
            HitType::GuardCrush => attack_info.on_guard_crush.attacker_meter,
            HitType::CounterHit => attack_info.on_counter_hit.attacker_meter,
            HitType::Graze => attack_info.on_graze.attacker_meter,
            HitType::Block => attack_info.on_block.attacker_meter,
            HitType::WrongBlock => attack_info.on_wrongblock.attacker_meter,
        };
        self.hitstop = match info {
            HitType::Hit => attack_info.on_hit.attacker_stop,
            HitType::GuardCrush => attack_info.on_guard_crush.attacker_stop,
            HitType::CounterHit => attack_info.on_counter_hit.attacker_stop,
            HitType::Graze => 0,
            HitType::Block => attack_info.on_block.attacker_stop,
            HitType::WrongBlock => attack_info.on_wrongblock.attacker_stop,
        };

        self.on_bullet_deal_hit(world, data, entity, info);
    }
}
