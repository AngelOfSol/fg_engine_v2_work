mod command_list;
mod moves;
mod particles;

use serde::Deserialize;

use crate::character_state::CharacterState;

use crate::typedefs::collision;
use crate::typedefs::collision::IntoGraphical;
use crate::typedefs::graphics;

use crate::assets::Assets;

use ggez::{Context, GameResult};

use std::path::PathBuf;

use std::fs::File;
use std::io::BufReader;

use std::collections::HashMap;

use crate::timeline::AtTime;

use crate::input::{read_inputs, InputBuffer};

use crate::command_list::CommandList;

use crate::graphics::Animation;

use moves::YuyukoMove;
use particles::YuyukoParticle;

pub struct Yuyuko {
    assets: Assets,
    states: YuyukoStateList,
    particles: YuyukoParticleList,
    command_list: CommandList<YuyukoMove>,
}

type YuyukoStateList = HashMap<YuyukoMove, CharacterState<YuyukoMove, YuyukoParticle>>;
type YuyukoParticleList = HashMap<YuyukoParticle, Animation>;

impl Yuyuko {
    pub fn new_with_path(ctx: &mut Context, path: PathBuf) -> GameResult<Yuyuko> {
        let mut assets = Assets::new();
        let data = YuyukoData::load_from_json(ctx, &mut assets, path)?;
        Ok(Yuyuko {
            assets,
            states: data.states,
            particles: data.particles,
            command_list: command_list::generate_command_list(),
        })
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct YuyukoData {
    states: YuyukoStateList,
    particles: YuyukoParticleList,
}

impl YuyukoData {
    fn load_from_json(
        ctx: &mut Context,
        assets: &mut Assets,
        mut path: PathBuf,
    ) -> GameResult<YuyukoData> {
        let file = File::open(&path).unwrap();
        let buf_read = BufReader::new(file);
        let character = serde_json::from_reader::<_, YuyukoData>(buf_read).unwrap();
        let name = path.file_stem().unwrap().to_str().unwrap().to_owned();
        path.pop();
        path.push(&name);
        for (name, state) in character.states.iter() {
            CharacterState::load(ctx, assets, state, &name.to_string(), path.clone())?;
        }
        path.push("particles");
        for (_name, particle) in character.particles.iter() {
            Animation::load(ctx, assets, particle, path.clone())?;
        }
        Ok(character)
    }
}

pub struct YuyukoState {
    velocity: collision::Vec2,
    position: collision::Vec2,
    current_state: (usize, YuyukoMove),
    particles: Vec<(usize, collision::Vec2, YuyukoParticle)>,
}

impl YuyukoState {
    pub fn new() -> Self {
        Self {
            velocity: collision::Vec2::zeros(),
            position: collision::Vec2::zeros(),
            current_state: (0, YuyukoMove::Stand),
            particles: Vec::new(),
        }
    }
    pub fn update_frame(&self, data: &Yuyuko, input: &InputBuffer) -> Self {
        let (frame, yuyu_move) = self.current_state;
        // if the next frame would be out of bounds
        let (frame, yuyu_move) = if frame >= data.states[&yuyu_move].duration() - 1 {
            (0, data.states[&yuyu_move].on_expire_state)
        } else {
            (frame + 1, yuyu_move)
        };
        let cancels = data.states[&yuyu_move].cancels.at_time(frame);

        let (frame, mut yuyu_move) = {
            data.command_list
                .get_commands(&read_inputs(&input, true))
                .into_iter()
                .copied()
                .filter(|new_move| {
                    *new_move != yuyu_move
                        && cancels.always.contains(&data.states[new_move].state_type)
                        && !cancels.disallow.contains(new_move)
                })
                .fold(None, |acc, item| acc.or(Some(item)))
                .map(|new_move| (0, new_move))
                .unwrap_or((frame, yuyu_move))
        };

        let hitboxes = data.states[&yuyu_move].hitboxes.at_time(frame);
        let flags = data.states[&yuyu_move].flags.at_time(frame);

        let new_velocity = if flags.reset_velocity {
            collision::Vec2::zeros()
        } else {
            self.velocity
                // we only run gravity if the move doesn't want to reset velocity, because that means the move has a trajectory in mind
                + if flags.airborne {
                    collision::Vec2::new(0_00, -0_25) // TODO: tune gravity
                } else {
                    collision::Vec2::zeros()
                }
        } + flags.accel;
        let new_position = self.position + new_velocity;
        let new_position =
            if !flags.airborne || new_position.y - hitboxes.collision.half_size.y <= -4 {
                if flags.airborne {
                    yuyu_move = YuyukoMove::Stand;
                }
                collision::Vec2::new(new_position.x, hitboxes.collision.half_size.y)
            } else {
                new_position
            };
        let mut particles = self.particles.clone();
        for (ref mut frame, _, _) in particles.iter_mut() {
            *frame += 1;
        }
        for particle in data.states[&yuyu_move]
            .particles
            .iter()
            .filter(|item| item.frame == frame)
        {
            particles.push((0, particle.offset + self.position, particle.particle_id));
        }
        let particles: Vec<_> = particles
            .into_iter()
            .filter(|item| item.0 < data.particles[&item.2].frames.duration())
            .collect();
        Self {
            velocity: new_velocity,
            position: new_position,
            current_state: (frame, yuyu_move),
            particles,
        }
    }

    pub fn draw(
        &self,
        ctx: &mut Context,
        data: &Yuyuko,
        world: graphics::Matrix4,
    ) -> GameResult<()> {
        let (frame, yuyu_move) = self.current_state;

        let collision = &data.states[&yuyu_move].hitboxes.at_time(frame).collision;
        let position = world
            * graphics::Matrix4::new_translation(&graphics::up_dimension(
                self.position.into_graphical(),
            ));

        data.states[&yuyu_move].draw_at_time(
            ctx,
            &data.assets,
            frame,
            position
                * graphics::Matrix4::new_translation(&graphics::up_dimension(
                    -collision.center.into_graphical(),
                )),
        )?;

        for (frame, position, id) in &self.particles {
            data.particles[&id].draw_at_time(
                ctx,
                &data.assets,
                *frame,
                world
                    * graphics::Matrix4::new_translation(&graphics::up_dimension(
                        position.into_graphical(),
                    )),
            )?;
        }
        Ok(())
    }
}
