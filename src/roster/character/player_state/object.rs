use std::{collections::HashMap, hash::Hash};

use hecs::{Entity, EntityBuilder, World};

use crate::{
    character::state::components::GlobalGraphicMap,
    game_object::{
        constructors::{Construct, Constructor},
        properties::{CharacterAttack, ObjectHitboxSet, PropertyType, TryAsRef},
        state::{
            BulletHp, BulletTier, ExpiresAfterAnimation, GrazeResistance, HasHitbox, HitDelay,
            Hitstop, MultiHitType, ObjectAttack, Position, Timer, Velocity,
        },
    },
    graphics::animation_group::AnimationGroup,
    hitbox::PositionedHitbox,
    roster::{
        character::{
            data::Data,
            typedefs::{Character, HitId},
        },
        hit_info::HitType,
    },
};

use super::PlayerState;

impl<C: Character> PlayerState<C>
where
    Constructor: Construct<C>,
    PropertyType: TryAsRef<CharacterAttack<C>>,
{
    pub fn spawn_objects(&mut self, world: &mut World, data: &Data<C>) {
        for spawner in data.get(self).current_spawns() {
            let mut builder = EntityBuilder::new();
            for constructor in spawner.data.iter() {
                let _ = constructor
                    .construct_on_to(&mut builder, self, data)
                    .unwrap();
            }
            world.spawn(builder.build());
        }
    }
    pub fn update_objects(
        &mut self,
        world: &mut World,
        data: &Data<C>,
        global_graphics: &GlobalGraphicMap,
    ) {
        for (_, Timer(timer)) in world.query::<&mut Timer>().without::<Hitstop>().iter() {
            *timer += 1;
        }

        update_velocity(world);

        let remove_hitdelay_from = world
            .query::<&mut HitDelay>()
            .without::<Hitstop>()
            .iter()
            .filter_map(|(entity, HitDelay(ref mut hit_delay))| {
                *hit_delay -= 1;
                if *hit_delay <= 0 {
                    Some(entity)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        for entity in remove_hitdelay_from {
            world.remove_one::<HitDelay>(entity).unwrap();
        }

        let remove_hitstop_from = world
            .query::<&mut Hitstop>()
            .iter()
            .filter_map(|(entity, Hitstop(ref mut hitstop))| {
                *hitstop -= 1;
                if *hitstop <= 0 {
                    Some(entity)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        for entity in remove_hitstop_from {
            world.remove_one::<Hitstop>(entity).unwrap();
        }

        self.destroy_objects(world, data, global_graphics);
    }

    pub fn destroy_objects(
        &mut self,
        world: &mut World,
        data: &Data<C>,
        global_graphics: &GlobalGraphicMap,
    ) {
        self.destroy_expire(world, data, &data.graphics);
        self.destroy_expire(world, data, &global_graphics);
        self.destroy_dead(world, data);
    }

    pub fn update_sound(&mut self, data: &Data<C>) {
        for sound in data.get(self).current_sounds() {
            self.sound_state.play_sound(sound.channel, sound.name);
        }
    }

    pub fn get_object_hitboxes(
        &self,
        world: &World,
        data: &Data<C>,
    ) -> Vec<(Entity, Vec<PositionedHitbox>)> {
        world
            .query::<(Option<&Timer>, &Position, &C::ObjectData)>()
            .with::<HasHitbox>()
            .iter()
            .map(|(entity, (timer, position, object_data_id))| {
                let timer = timer.map(|t| t.0).unwrap_or_default();
                let boxes = data
                    .instance
                    .get::<ObjectHitboxSet>(*object_data_id)
                    .unwrap();
                let (_, hitboxes) = boxes.get(timer % boxes.duration());

                (
                    entity,
                    hitboxes
                        .boxes
                        .iter()
                        .map(|hitbox| hitbox.with_collision_position(position.value))
                        .collect(),
                )
            })
            .collect()
    }

    pub fn on_touch_entity(
        &mut self,
        world: &mut World,
        _data: &Data<C>,
        entity: Entity,
        tier: BulletTier,
    ) {
        if let Ok(mut bullet_hp) = world.get_mut::<BulletHp>(entity) {
            if tier >= bullet_hp.tier {
                bullet_hp.health -= 1;
            }
        }
    }

    pub fn on_bullet_deal_hit(
        &mut self,
        world: &mut World,
        data: &Data<C>,
        entity: Entity,
        info: &HitType,
    ) {
        if !matches!(info, HitType::Graze) {
            let mut query = world
                .query_one::<(Option<&Timer>, &mut ObjectAttack<C>, &C::ObjectData)>(entity)
                .unwrap()
                .with::<HasHitbox>();
            if let Some((timer, object_attack, object_data_id)) = query.get() {
                let timer = timer.map(|item| item.0).unwrap_or_default();
                let hitbox_id = data
                    .instance
                    .get::<ObjectHitboxSet>(*object_data_id)
                    .unwrap()[timer]
                    .id;
                let attack_id = data
                    .instance
                    .get::<CharacterAttack<C>>(*object_data_id)
                    .unwrap()[timer];

                self.smp.push(object_attack.command);

                match object_attack.multi_hit {
                    MultiHitType::LastHitUsing(ref mut last_hit_using) => {
                        *last_hit_using = Some(HitId {
                            hitbox_id,
                            id: attack_id,
                        })
                    }
                    MultiHitType::RemainingHits(ref mut hits) => *hits -= 1,
                }

                let id = *object_data_id;

                drop(query);

                if let Some(delay) = data.instance.get::<HitDelay>(id) {
                    world.insert_one(entity, *delay).unwrap();
                }
                if let Some(stop) = data.instance.get::<Hitstop>(id) {
                    world.insert_one(entity, *stop).unwrap();
                }
            }
        } else {
            let mut query = world
                .query_one::<&mut GrazeResistance>(entity)
                .unwrap()
                .with::<HasHitbox>();
            if let Some(GrazeResistance(ref mut graze_resistance)) = query.get() {
                *graze_resistance -= 1;
            }
        }
    }

    pub fn kill(&mut self, world: &mut World, _data: &Data<C>, entity: Entity) {
        world.despawn(entity).unwrap();
    }

    pub fn destroy_expire<K: Hash + Eq + hecs::Component>(
        &mut self,
        world: &mut World,
        data: &Data<C>,
        graphics: &HashMap<K, AnimationGroup>,
    ) {
        let to_destroy: Vec<_> = world
            .query::<(&Timer, &K)>()
            .with::<ExpiresAfterAnimation>()
            .iter()
            .filter(|(_, (Timer(timer), graphic))| *timer >= graphics[graphic].duration())
            .map(|(entity, _)| entity)
            .collect();

        for entity in to_destroy {
            self.kill(world, data, entity);
        }
    }

    pub fn destroy_dead(&mut self, world: &mut World, data: &Data<C>) {
        let to_destroy_hp: Vec<_> = world
            .query::<&BulletHp>()
            .iter()
            .filter(|(_, hp)| hp.health <= 0)
            .map(|(entity, _)| entity)
            .collect();

        for entity in to_destroy_hp {
            self.kill(world, data, entity);
        }

        let to_destroy_hits: Vec<_> = world
            .query::<&ObjectAttack<C>>()
            .iter()
            .filter(|(_, object_attack)| {
                if let MultiHitType::RemainingHits(hits) = object_attack.multi_hit {
                    hits <= 0
                } else {
                    false
                }
            })
            .map(|(entity, _)| entity)
            .collect();

        for entity in to_destroy_hits {
            self.kill(world, data, entity);
        }

        let to_destroy_grazed: Vec<_> = world
            .query::<&GrazeResistance>()
            .iter()
            .filter(|(_, GrazeResistance(resistance))| *resistance <= 0)
            .map(|(entity, _)| entity)
            .collect();

        for entity in to_destroy_grazed {
            self.kill(world, data, entity);
        }
    }
}

pub fn update_velocity(world: &mut World) {
    for (_, (position, velocity)) in world
        .query::<(&mut Position, &Velocity)>()
        .without::<Hitstop>()
        .iter()
    {
        position.value += velocity.value;
    }
}
