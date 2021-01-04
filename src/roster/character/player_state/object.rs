use hecs::{EntityBuilder, World};

use crate::{
    character::state::components::{GlobalGraphic, GlobalGraphicMap},
    game_object::{
        constructors::{Construct, Constructor},
        state::{ExpiresAfterAnimation, Timer},
    },
    roster::character::{data::Data, typedefs::Character},
};

use super::PlayerState;

impl<C: Character> PlayerState<C>
where
    Constructor: Construct<C>,
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
        for (_, Timer(timer)) in world.query::<&mut Timer>().iter() {
            *timer += 1;
        }
        let to_destroy: Vec<_> = world
            .query::<(&Timer, &C::Graphic)>()
            .with::<ExpiresAfterAnimation>()
            .iter()
            .filter(|(_, (Timer(timer), graphic))| *timer >= data.graphics[graphic].duration())
            .map(|(entity, _)| entity)
            .chain(
                world
                    .query::<(&Timer, &GlobalGraphic)>()
                    .with::<ExpiresAfterAnimation>()
                    .iter()
                    .filter(|(_, (Timer(timer), graphic))| {
                        *timer >= global_graphics[graphic].duration()
                    })
                    .map(|(entity, _)| entity),
            )
            .collect();
        for entity in to_destroy {
            world.despawn(entity).unwrap();
        }
    }

    pub fn update_sound(&mut self, data: &Data<C>) {
        for sound in data.get(self).current_sounds() {
            self.sound_state.play_sound(sound.channel, sound.name);
        }
    }
}
