mod bullets;
mod command_list;
mod moves;
mod particles;

use crate::assets::Assets;
use crate::character::state::Flags;
use crate::character::state::MoveType;
use crate::character::state::State;
use crate::command_list::CommandList;
use crate::game_match::PlayArea;
use crate::graphics::Animation;
use crate::hitbox::Hitbox;
use crate::input::{read_inputs, Button, DirectedAxis, Facing, InputBuffer};
use crate::timeline::AtTime;
use crate::typedefs::collision;
use crate::typedefs::collision::IntoGraphical;
use crate::typedefs::graphics;
use bullets::{BulletSpawn, BulletState};
use ggez::{Context, GameResult};
use moves::MoveId;
use particles::Particle;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

#[derive(Clone, Debug, Deserialize)]
pub struct BulletData {
    pub animation: Animation,
    pub hitbox: Hitbox,
}
#[derive(Clone, Debug, Deserialize)]
pub struct BulletList {
    pub butterfly: BulletData,
}
pub struct Yuyuko {
    assets: Assets,
    states: StateList,
    particles: ParticleList,
    bullets: BulletList,
    properties: Properties,
    command_list: CommandList<MoveId>,
}

type StateList = HashMap<MoveId, State<MoveId, Particle, BulletSpawn>>;
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
            bullets: data.bullets,
            command_list: command_list::generate_command_list(),
        })
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct YuyukoData {
    states: StateList,
    particles: ParticleList,
    bullets: BulletList,
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
    max_air_actions: usize,
    max_spirit_gauge: i32,
}

impl YuyukoData {
    fn load_from_json(
        ctx: &mut Context,
        assets: &mut Assets,
        mut path: PathBuf,
    ) -> GameResult<YuyukoData> {
        let file = File::open(&path).unwrap();
        let buf_read = BufReader::new(file);
        let mut character = serde_json::from_reader::<_, YuyukoData>(buf_read).unwrap();
        let name = path.file_stem().unwrap().to_str().unwrap().to_owned();
        path.pop();
        path.push(&name);
        for (name, state) in character.states.iter_mut() {
            State::load(ctx, assets, state, &name.to_string(), path.clone())?;
        }
        path.push("particles");
        for (_name, particle) in character.particles.iter_mut() {
            Animation::load(ctx, assets, particle, path.clone())?;
        }
        path.pop();
        path.push("bullets");
        Animation::load(
            ctx,
            assets,
            &mut character.bullets.butterfly.animation,
            path.clone(),
        )?;

        Ok(character)
    }
}

#[derive(Debug, Clone, Copy)]
enum ExtraData {
    JumpDirection(DirectedAxis),
    FlyDirection(DirectedAxis),
    None,
}

impl ExtraData {
    fn unwrap_jump_direction(self) -> DirectedAxis {
        match self {
            ExtraData::JumpDirection(dir) => dir,
            value => panic!("Expected JumpDirection, found {:?}.", value),
        }
    }
    fn unwrap_fly_direction(self) -> DirectedAxis {
        match self {
            ExtraData::FlyDirection(dir) => dir,
            value => panic!("Expected FlyDirection, found {:?}.", value),
        }
    }
}

#[derive(Debug, Clone)]
pub struct YuyukoState {
    pub velocity: collision::Vec2,
    pub position: collision::Vec2,
    pub current_state: (usize, MoveId),
    extra_data: ExtraData,
    pub particles: Vec<(usize, collision::Vec2, Particle)>,
    pub bullets: Vec<BulletState>,
    pub facing: Facing,
    pub air_actions: usize,
    pub spirit_gauge: i32,
    pub spirit_delay: i32,
}

impl YuyukoState {
    pub fn new(data: &Yuyuko) -> Self {
        Self {
            velocity: collision::Vec2::zeros(),
            position: collision::Vec2::zeros(),
            current_state: (0, MoveId::Stand),
            extra_data: ExtraData::None,
            particles: Vec::new(),
            bullets: Vec::new(),
            air_actions: data.properties.max_air_actions,
            spirit_gauge: data.properties.max_spirit_gauge,
            spirit_delay: 0,
            facing: Facing::Right,
        }
    }

    fn handle_fly(move_id: MoveId, extra_data: &mut ExtraData) -> collision::Vec2 {
        if move_id == MoveId::FlyStart {
            let fly_dir = extra_data.unwrap_fly_direction();
            *extra_data = ExtraData::FlyDirection(fly_dir);
            let speed = match fly_dir {
                DirectedAxis::Forward => collision::Vec2::new(1_00, 0_00),
                DirectedAxis::UpForward => collision::Vec2::new(0_71, 0_71),
                DirectedAxis::DownForward => collision::Vec2::new(0_71, -0_71),
                DirectedAxis::Backward => collision::Vec2::new(-1_00, 0_00),
                DirectedAxis::UpBackward => collision::Vec2::new(-0_71, 0_71),
                DirectedAxis::DownBackward => collision::Vec2::new(-0_71, -0_71),
                DirectedAxis::Up => collision::Vec2::new(0_00, 1_00),
                DirectedAxis::Down => collision::Vec2::new(0_00, -1_00),
                _ => unreachable!(),
            };
            3 * speed / 4
        } else {
            collision::Vec2::zeros()
        }
    }

    fn handle_jump(
        flags: &Flags,
        data: &Properties,
        move_id: MoveId,
        extra_data: &mut ExtraData,
    ) -> collision::Vec2 {
        if flags.jump_start {
            let axis = extra_data.unwrap_jump_direction();
            *extra_data = ExtraData::None;
            match move_id {
                MoveId::Jump => {
                    if axis == DirectedAxis::Up {
                        data.neutral_jump_accel
                    } else {
                        data.directed_jump_accel
                            .component_mul(&collision::Vec2::new(
                                axis.direction_multiplier(true),
                                1,
                            ))
                    }
                }
                MoveId::SuperJump => {
                    if axis == DirectedAxis::Up {
                        data.neutral_super_jump_accel
                    } else {
                        data.directed_super_jump_accel
                            .component_mul(&collision::Vec2::new(
                                axis.direction_multiplier(true),
                                1,
                            ))
                    }
                }
                _ => panic!("jump_start not allowed on non jump moves"),
            }
        } else {
            collision::Vec2::zeros()
        }
    }

    fn handle_expire(&mut self, data: &Yuyuko) {
        let (frame, move_id) = self.current_state;
        // if the next frame would be out of bounds
        self.current_state = if frame >= data.states[&move_id].duration() - 1 {
            (0, data.states[&move_id].on_expire_state)
        } else {
            (frame + 1, move_id)
        };
    }
    fn handle_input(&mut self, data: &Yuyuko, input: &InputBuffer) {
        let (frame, move_id) = self.current_state;
        let cancels = data.states[&move_id].cancels.at_time(frame);

        self.current_state = {
            let inputs = read_inputs(&input, self.facing);
            if move_id == MoveId::Fly {
                if input.top()[Button::A].is_pressed() && input.top()[Button::B].is_pressed() {
                    (frame, move_id)
                } else {
                    (0, MoveId::FlyEnd)
                }
            } else {
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
                            // not ideal way to handle disallowing fly, consider separating out from cancel checking
                            && !(*new_move_id == MoveId::FlyStart && self.air_actions == 0)
                            && self.spirit_gauge >= data.states[&new_move_id].minimum_spirit_required
                    })
                    .fold(None, |acc, item| acc.or(Some(item)))
                    .map(|new_move| (0, new_move))
                    .unwrap_or((frame, move_id))
            }
        };
    }

    fn update_extra_data(&mut self, input: &InputBuffer) {
        let (frame, move_id) = self.current_state;
        if frame == 0 {
            if move_id == MoveId::Jump || move_id == MoveId::SuperJump {
                self.extra_data = ExtraData::JumpDirection(DirectedAxis::from_facing(
                    input.top().axis,
                    self.facing,
                ));
            } else if move_id == MoveId::FlyStart {
                if frame == 0 {
                    self.air_actions -= 1;
                }
                let mut dir = DirectedAxis::from_facing(input.top().axis, self.facing);
                if dir.is_backward() {
                    self.facing = self.facing.invert();
                    dir = dir.invert();
                }
                self.extra_data = ExtraData::FlyDirection(if dir == DirectedAxis::Neutral {
                    DirectedAxis::Forward
                } else {
                    dir
                });
            }
        }
    }

    fn update_velocity(&mut self, data: &Yuyuko) {
        let (frame, move_id) = self.current_state;
        let flags = data.states[&move_id].flags.at_time(frame);

        let base_velocity = if flags.reset_velocity {
            collision::Vec2::zeros()
        } else {
            self.velocity
        };

        // we only run gravity if the move doesn't want to reset velocity, because that [resetting velocity] means the move has a trajectory in mind
        let gravity = if !flags.reset_velocity
            && flags.airborne
            && move_id != MoveId::FlyStart
            && move_id != MoveId::Fly
        {
            collision::Vec2::new(0_00, -0_20)
        } else {
            collision::Vec2::zeros()
        };
        let friction = if !flags.airborne {
            collision::Vec2::new(-0_20 * i32::signum(base_velocity.x), 0_00)
        } else {
            collision::Vec2::zeros()
        };

        let accel = self.facing.fix_collision(flags.accel)
            + self
                .facing
                .fix_collision(YuyukoState::handle_fly(move_id, &mut self.extra_data))
            + self.facing.fix_collision(YuyukoState::handle_jump(
                flags,
                &data.properties,
                move_id,
                &mut self.extra_data,
            ));
        self.velocity = base_velocity + accel + friction + gravity;
    }

    fn update_position(&mut self, data: &Yuyuko, play_area: &PlayArea) {
        let (frame, move_id) = self.current_state;
        let flags = data.states[&move_id].flags.at_time(frame);
        let hitboxes = data.states[&move_id].hitboxes.at_time(frame);
        let collision = &hitboxes.collision;

        self.position += self.velocity;

        // handle landing
        if flags.airborne && self.position.y - collision.half_size.y <= -4 {
            self.velocity = collision::Vec2::zeros();
            self.current_state.0 = 0;
            self.current_state.1 = MoveId::Stand;
            self.position.y = hitboxes.collision.half_size.y;
            self.air_actions = data.properties.max_air_actions;
        }

        // handle stage sides
        if i32::abs(self.position.x) > play_area.width / 2 - collision.half_size.x {
            self.position.x =
                i32::signum(self.position.x) * (play_area.width / 2 - collision.half_size.x);
        }

        // if not airborne, make sure the character is locked to the ground properly
        if !flags.airborne {
            self.position.y = hitboxes.collision.half_size.y;
        }
    }

    fn update_particles(&mut self, data: &Yuyuko) {
        let (frame, move_id) = self.current_state;
        let state_particles = &data.states[&move_id].particles;

        for (ref mut frame, _, _) in self.particles.iter_mut() {
            *frame += 1;
        }
        for particle in state_particles.iter().filter(|item| item.frame == frame) {
            self.particles
                .push((0, particle.offset + self.position, particle.particle_id));
        }
        self.particles
            .retain(|item| item.0 < data.particles[&item.2].frames.duration());
    }

    fn update_bullets(&mut self, data: &Yuyuko, play_area: &PlayArea) {
        // first update all active bullets
        for bullet in self.bullets.iter_mut() {
            match bullet {
                BulletState::Butterfly {
                    ref mut position,
                    velocity,
                    ..
                } => {
                    *position += *velocity;
                }
            }
        }
        // then remove offscreen bullets
        self.bullets = self
            .bullets
            .drain(..)
            .filter(|bullet| match bullet {
                BulletState::Butterfly { position, .. } => {
                    !(i32::abs(position.x)
                        > play_area.width / 2 + data.bullets.butterfly.hitbox.half_size.x
                        || i32::abs(position.y)
                            > play_area.width / 2 + data.bullets.butterfly.hitbox.half_size.y)
                }
            })
            .collect();

        // then spawn bullets
        let (frame, move_id) = self.current_state;
        for spawn in data.states[&move_id]
            .bullets
            .iter()
            .filter(|item| item.get_spawn_frame() == frame)
        {
            self.bullets
                .push(spawn.instantiate(self.position, self.facing));
        }
    }

    fn update_spirit(&mut self, data: &Yuyuko) {
        let (ref mut frame, ref mut move_id) = &mut self.current_state;
        let move_data = &data.states[move_id];
        let flags = move_data.flags.at_time(*frame);

        if move_data.state_type == MoveType::Fly {
            self.spirit_gauge -= 10; // TODO, move this spirit cost to an editor value
            Self::clamp_spirit(&mut self.spirit_gauge, data);
            if self.spirit_gauge == 0 {
                *move_id = MoveId::FlyEnd;
                *frame = 0;
            }
        } else {
            self.spirit_gauge -= flags.spirit_cost;

            if flags.reset_spirit_delay {
                self.spirit_delay = 0;
            }
            self.spirit_delay += flags.spirit_delay;
            self.spirit_delay -= 1;
            self.spirit_delay = std::cmp::max(self.spirit_delay, 0);

            if self.spirit_delay == 0 {
                self.spirit_gauge += 5; // TODO: move this spirit regen to an editor value
            }

            Self::clamp_spirit(&mut self.spirit_gauge, data);
        }
    }
    fn clamp_spirit(spirit_gauge: &mut i32, data: &Yuyuko) {
        *spirit_gauge = std::cmp::max(
            std::cmp::min(*spirit_gauge, data.properties.max_spirit_gauge),
            0,
        );
    }

    fn handle_refacing(&mut self, data: &Yuyuko) {
        let (frame, move_id) = self.current_state;
        let flags = data.states[&move_id].flags.at_time(frame);
        if flags.allow_reface {
            self.facing = if self.position.x > 100 {
                Facing::Left
            } else {
                Facing::Right
            }
        }
    }

    pub fn update_frame_mut(&mut self, data: &Yuyuko, input: &InputBuffer, play_area: &PlayArea) {
        self.handle_expire(data);
        self.handle_input(data, input);
        self.update_extra_data(input);
        self.update_velocity(data);
        self.update_position(data, play_area);
        self.update_spirit(data);
        self.update_particles(data);
        self.update_bullets(data, play_area);
        self.handle_refacing(data);
    }
    pub fn draw_ui(
        &self,
        ctx: &mut Context,
        data: &Yuyuko,
        bottom_line: graphics::Matrix4,
    ) -> GameResult<()> {
        ggez::graphics::set_transform(ctx, bottom_line);
        ggez::graphics::apply_transformations(ctx)?;

        let hp_current = ggez::graphics::Rect::new(
            0.0,
            0.0,
            100.0 * self.spirit_gauge as f32 / data.properties.max_spirit_gauge as f32,
            20.0,
        );
        let hp_backdrop = ggez::graphics::Rect::new(0.0, 0.0, 100.0, 20.0);
        let hp_max = ggez::graphics::Rect::new(-5.0, -5.0, 110.0, 30.0);

        let rect = ggez::graphics::Mesh::new_rectangle(
            ctx,
            ggez::graphics::DrawMode::Fill(ggez::graphics::FillOptions::default()),
            hp_max,
            ggez::graphics::Color::new(0.0, 0.0, 0.0, 1.0),
        )?;

        ggez::graphics::draw(ctx, &rect, ggez::graphics::DrawParam::default())?;

        let rect = ggez::graphics::Mesh::new_rectangle(
            ctx,
            ggez::graphics::DrawMode::Fill(ggez::graphics::FillOptions::default()),
            hp_backdrop,
            ggez::graphics::Color::new(1.0, 1.0, 1.0, 1.0),
        )?;

        ggez::graphics::draw(ctx, &rect, ggez::graphics::DrawParam::default())?;

        let rect = ggez::graphics::Mesh::new_rectangle(
            ctx,
            ggez::graphics::DrawMode::Fill(ggez::graphics::FillOptions::default()),
            hp_current,
            ggez::graphics::Color::new(0.0, 0.0, 1.0, 1.0),
        )?;

        ggez::graphics::draw(ctx, &rect, ggez::graphics::DrawParam::default())?;

        Ok(())
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
                    self.facing.fix_graphics(-collision.center.into_graphical()),
                ))
                * graphics::Matrix4::new_nonuniform_scaling(&graphics::up_dimension(
                    self.facing.graphics_multiplier(),
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
        for bullet in &self.bullets {
            match bullet {
                BulletState::Butterfly {
                    position, rotation, ..
                } => {
                    data.bullets.butterfly.animation.draw_at_time(
                        ctx,
                        &data.assets,
                        0,
                        world
                            * graphics::Matrix4::new_translation(&graphics::up_dimension(
                                position.into_graphical(),
                            ))
                            * graphics::Matrix4::new_rotation(
                                nalgebra::Vector3::new(0.0, 0.0, 1.0) * *rotation,
                            ),
                    )?;
                }
            }
        }

        Ok(())
    }
    pub fn draw_shadow(
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

        data.states[&move_id].draw_shadow_at_time(
            ctx,
            &data.assets,
            frame,
            position
                * graphics::Matrix4::new_translation(&graphics::up_dimension(
                    self.facing.fix_graphics(-collision.center.into_graphical()),
                ))
                * graphics::Matrix4::new_nonuniform_scaling(&graphics::up_dimension(
                    self.facing.graphics_multiplier(),
                )),
        )?;
        Ok(())
    }
}
