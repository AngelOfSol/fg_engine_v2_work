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

use crate::input::{read_inputs, DirectedAxis, InputBuffer};

use crate::command_list::CommandList;

use crate::graphics::Animation;

use moves::MoveId;
use particles::Particle;

pub struct Yuyuko {
    assets: Assets,
    states: StateList,
    particles: ParticleList,
    properties: Properties,
    command_list: CommandList<MoveId>,
}

type StateList = HashMap<MoveId, CharacterState<MoveId, Particle>>;
type ParticleList = HashMap<Particle, Animation>;

impl Yuyuko {
    pub fn new_with_path(ctx: &mut Context, path: PathBuf) -> GameResult<Yuyuko> {
        let mut assets = Assets::new();
        let data = YuyukoData::load_from_json(ctx, &mut assets, path)?;
        Ok(Yuyuko {
            assets,
            states: data.states,
            particles: data.particles,
            properties: data.properties,
            command_list: command_list::generate_command_list(),
        })
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct YuyukoData {
    states: StateList,
    particles: ParticleList,
    properties: Properties,
}

#[derive(Clone, Debug, Deserialize)]
struct Properties {
    health: u32,
    name: String,
    neutral_jump_accel: collision::Vec2,
    neutral_super_jump_accel: collision::Vec2,
    directed_jump_accel: collision::Vec2,
    directed_super_jump_accel: collision::Vec2,
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

#[derive(Debug, Clone, Copy)]
enum ExtraData {
    JumpDirection(DirectedAxis),
    None,
}

impl ExtraData {
    fn unwrap_jump_direction(self) -> DirectedAxis {
        match self {
            ExtraData::JumpDirection(dir) => dir,
            value => panic!("Expected JumpDirection, found {:?}.", value),
        }
    }
}

#[derive(Debug, Clone)]
pub struct YuyukoState {
    velocity: collision::Vec2,
    position: collision::Vec2,
    current_state: (usize, MoveId),
    extra_data: ExtraData,
    particles: Vec<(usize, collision::Vec2, Particle)>,
}

impl YuyukoState {
    pub fn new() -> Self {
        Self {
            velocity: collision::Vec2::zeros(),
            position: collision::Vec2::zeros(),
            current_state: (0, MoveId::Stand),
            extra_data: ExtraData::None,
            particles: Vec::new(),
        }
    }
    pub fn update_frame(&self, data: &Yuyuko, input: &InputBuffer) -> Self {
        let (frame, move_id) = self.current_state;
        // if the next frame would be out of bounds
        let (frame, move_id) = if frame >= data.states[&move_id].duration() - 1 {
            (0, data.states[&move_id].on_expire_state)
        } else {
            (frame + 1, move_id)
        };
        let cancels = data.states[&move_id].cancels.at_time(frame);

        let (frame, mut move_id) = {
            let inputs = read_inputs(&input, true);
            if inputs.len() > 1 {
                dbg!(&inputs);
            }
            data.command_list
                .get_commands(&inputs)
                .into_iter()
                .copied()
                .filter(|new_move_id| {
                    *new_move_id != move_id
                        && cancels
                            .always
                            .contains(&data.states[new_move_id].state_type)
                        && !cancels.disallow.contains(new_move_id)
                })
                .fold(None, |acc, item| acc.or(Some(item)))
                .map(|new_move| (0, new_move))
                .unwrap_or((frame, move_id))
        };

        let mut new_extra_data =
            if frame == 0 && (move_id == MoveId::Jump || move_id == MoveId::SuperJump) {
                ExtraData::JumpDirection(input.top().axis.into())
            } else {
                self.extra_data
            };

        let hitboxes = data.states[&move_id].hitboxes.at_time(frame);
        let flags = data.states[&move_id].flags.at_time(frame);

        let mut new_velocity = if flags.reset_velocity {
            collision::Vec2::zeros()
        } else {
            let vel = if flags.airborne {
                self.velocity
                    + if flags.jump_start {
                        let axis = new_extra_data.unwrap_jump_direction();
                        new_extra_data = ExtraData::None;
                        match move_id {
                            MoveId::Jump => {
                                if axis == DirectedAxis::Up {
                                    data.properties.neutral_jump_accel
                                } else {
                                    data.properties.directed_jump_accel.component_mul(
                                        &collision::Vec2::new(axis.direction_multiplier(true), 1),
                                    )
                                }
                            }
                            MoveId::SuperJump => {
                                if axis == DirectedAxis::Up {
                                    data.properties.neutral_super_jump_accel
                                } else {
                                    data.properties.directed_super_jump_accel.component_mul(
                                        &collision::Vec2::new(axis.direction_multiplier(true), 1),
                                    )
                                }
                            }
                            _ => panic!("jump_start not allowed on non jump moves"),
                        }
                    } else {
                        collision::Vec2::zeros()
                    }
            } else {
                self.velocity.component_div(&collision::Vec2::new(2, 1))
            };
            // we only run gravity if the move doesn't want to reset velocity, because that means the move has a trajectory in mind
            vel + if flags.airborne {
                collision::Vec2::new(0_00, -0_25)
            } else {
                collision::Vec2::zeros()
            }
        } + flags.accel;

        let new_position = self.position + new_velocity;
        let new_position =
            if !flags.airborne || new_position.y - hitboxes.collision.half_size.y <= -4 {
                if flags.airborne {
                    new_velocity = collision::Vec2::zeros();
                    move_id = MoveId::Stand;
                }
                collision::Vec2::new(new_position.x, hitboxes.collision.half_size.y)
            } else {
                new_position
            };
        let mut particles = self.particles.clone();
        for (ref mut frame, _, _) in particles.iter_mut() {
            *frame += 1;
        }
        for particle in data.states[&move_id]
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
            current_state: (frame, move_id),
            extra_data: new_extra_data,
            particles,
        }
    }

    pub fn draw(
        &self,
        ctx: &mut Context,
        data: &Yuyuko,
        world: graphics::Matrix4,
    ) -> GameResult<()> {
        let (frame, move_id) = self.current_state;

        let collision = &data.states[&move_id].hitboxes.at_time(frame).collision;
        let position = world
            * graphics::Matrix4::new_translation(&graphics::up_dimension(
                self.position.into_graphical(),
            ));

        data.states[&move_id].draw_at_time(
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
