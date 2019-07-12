use serde::{Deserialize, Serialize};

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

use crate::input::{read_inputs, Button, ButtonSet, DirectedAxis, Input, InputBuffer};

use crate::command_list::CommandList;

use crate::{make_command_list, numpad, read_axis};

pub struct Yuyuko {
    assets: Assets,
    states: YuyukoStateList,
    command_list: CommandList<YuyukoMove>,
}

type YuyukoStateList = HashMap<YuyukoMove, CharacterState<YuyukoMove>>;

impl Yuyuko {
    pub fn new_with_path(ctx: &mut Context, path: PathBuf) -> GameResult<Yuyuko> {
        let mut assets = Assets::new();
        let data = YuyukoData::load_from_json(ctx, &mut assets, path)?;
        let command_list = make_command_list! {
            numpad!(5 A), numpad!(4 A), numpad!(6 A) => YuyukoMove::Attack5A,

            numpad!(6) => YuyukoMove::WalkForward,
            numpad!(4) => YuyukoMove::WalkBackward,

            numpad!(1) => YuyukoMove::ToCrouch,
            numpad!(2) => YuyukoMove::ToCrouch,
            numpad!(3) => YuyukoMove::ToCrouch,
            numpad!(1) => YuyukoMove::Crouch,
            numpad!(2) => YuyukoMove::Crouch,
            numpad!(3) => YuyukoMove::Crouch,

            numpad!(5) => YuyukoMove::ToStand,
            numpad!(5) => YuyukoMove::Stand
        };
        Ok(Yuyuko {
            assets,
            states: data.states,
            command_list,
        })
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Hash, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum YuyukoMove {
    Stand,
    WalkBackward,
    WalkForward,
    #[serde(rename = "attack5a")]
    Attack5A,
    Crouch,
    ToCrouch,
    ToStand,
}

impl Default for YuyukoMove {
    fn default() -> Self {
        YuyukoMove::Stand
    }
}

impl YuyukoMove {
    pub fn to_string(self) -> String {
        serde_json::to_string(&self)
            .unwrap()
            .trim_matches('\"')
            .to_owned()
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct YuyukoData {
    states: YuyukoStateList,
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
        Ok(character)
    }
}

pub struct YuyukoState {
    velocity: collision::Vec2,
    position: collision::Vec2,
    current_state: (usize, YuyukoMove),
}

impl YuyukoState {
    pub fn new() -> Self {
        Self {
            velocity: collision::Vec2::zeros(),
            position: collision::Vec2::zeros(),
            current_state: (0, YuyukoMove::Stand),
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

        let (frame, yuyu_move) = {
            data.command_list
                .get_commands(&read_inputs(&input, true))
                .into_iter()
                .copied()
                .filter(|new_move| {
                    *new_move != yuyu_move
                        && cancels.always.contains(&data.states[new_move].state_type)
                        && !cancels.disallow.contains(new_move)
                    /*&& data
                    .disallow
                    .get(&yuyu_move)
                    .map(|disallowed| disallowed != new_move)
                    .unwrap_or(true)*/
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
                    collision::Vec2::new(0_00, -0_20) // TODO: tune gravity
                } else {
                    collision::Vec2::zeros()
                }
        } + flags.accel;
        let new_position = self.position + new_velocity;
        let new_position = if !flags.airborne {
            collision::Vec2::new(new_position.x, hitboxes.collision.center.y)
        } else {
            new_position
        };

        Self {
            velocity: new_velocity,
            position: new_position,
            current_state: (frame, yuyu_move),
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
        data.states[&yuyu_move].draw_at_time(
            ctx,
            &data.assets,
            frame,
            world
                * graphics::Matrix4::new_translation(&graphics::up_dimension(
                    (-collision.center + self.position).into_graphical(),
                )),
        )?;
        Ok(())
    }
}
