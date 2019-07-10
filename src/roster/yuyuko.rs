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

use std::hash::Hash;

use crate::timeline::AtTime;

pub struct Yuyuko {
    assets: Assets,
    pub states: YuyukoStateList,
}

type YuyukoStateList = HashMap<YuyukoMove, CharacterState>;

impl Yuyuko {
    pub fn new_with_path(ctx: &mut Context, path: PathBuf) -> GameResult<Yuyuko> {
        let mut assets = Assets::new();
        let data = YuyukoData::load_from_json(ctx, &mut assets, path)?;
        Ok(Yuyuko {
            assets,
            states: data.states,
        })
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Hash, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum YuyukoMove {
    Idle,
    WalkBackward,
    WalkForward,
    #[serde(rename = "attack_5a")]
    Attack5A,
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
            current_state: (0, YuyukoMove::Idle),
        }
    }
    pub fn update_frame(&self, data: &Yuyuko) -> Self {
        let (frame, yuyu_move) = self.current_state;
        // if the next frame would be out of bounds
        let (frame, yuyu_move) = if frame >= data.states[&yuyu_move].duration() - 1 {
            (0, YuyukoMove::Idle)
        } else {
            (frame + 1, yuyu_move)
        };
        let _cancels = data.states[&yuyu_move].cancels.at_time(frame);
        let _hitboxes = data.states[&yuyu_move].hitboxes.at_time(frame);
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

        Self {
            velocity: new_velocity,
            position: self.position + new_velocity,
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
        data.states[&yuyu_move].draw_at_time(
            ctx,
            &data.assets,
            frame,
            world
                * graphics::Matrix4::new_translation(&graphics::up_dimension(
                    self.position.into_graphical(),
                )),
        )?;
        Ok(())
    }
}
