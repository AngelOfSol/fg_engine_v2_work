#[macro_use]
pub mod generic_character;
mod yuyuko;

use enum_dispatch::enum_dispatch;

pub use generic_character::*;
pub use yuyuko::*;


#[enum_dispatch(GenericCharacterBehavior)]
pub enum CharacterBehavior {
    YuyukoPlayer(YuyukoPlayer),
}

impl From<YuyukoPlayer> for CharacterBehavior {
    fn from(value: YuyukoPlayer) -> Self {
        Self::YuyukoPlayer(value)
    }
}

macro_rules! impl_function {
    (fn $id:ident (&mut self$(,)? $($param:ident : $type:ty),*) $(-> $ret:ty)?$(;)?) => {
        pub fn $id(&mut self $(,$param: $type)*) $(-> $ret)? {
            match self {
                Self::YuyukoPlayer(value) => value.$id( $($param),*),
            }
        }
    };
    (fn $id:ident (&self$(,)? $($param:ident : $type:ty),*) $(-> $ret:ty)?$(;)?) => {
        pub fn $id(&self $(,$param: $type)*) $(-> $ret)? {
            match self {
                Self::YuyukoPlayer(value) => value.$id( $($param),*),
            }
        }
    };

    (fn $id:ident (&self$(,)? $($param:ident : $type:ty),*) $(-> $ret:ty)?; $($rest:tt)+) => {
        impl_function!(fn $id (&self $(,$param : $type)*) $(-> $ret)?);
        impl_function!($($rest)+);
    };
    (fn $id:ident (&mut self$(,)? $($param:ident : $type:ty),*) $(-> $ret:ty)?; $($rest:tt)+) => {
        impl_function!(fn $id (&mut self $(,$param : $type)*) $(-> $ret)?);
        impl_function!($($rest)+);
    };
}

use crate::character::components::AttackInfo;
use crate::character::state::components::Flags;
use crate::game_match::sounds::SoundList;
use crate::game_match::PlayArea;
use crate::hitbox::PositionedHitbox;
use crate::input::{Facing, InputState};
use crate::typedefs::{collision, graphics};
use extra_data::ExtraData;
use ggez::{Context, GameResult};
use hit_info::{HitInfo, HitType};
use rodio::Device;

impl CharacterBehavior {
    impl_function!(
        fn position(&self) -> collision::Vec2;
        fn position_mut(&mut self) -> &mut collision::Vec2;
        fn handle_refacing(&mut self, other_player: collision::Int);
        fn apply_pushback(&mut self, force: collision::Int);
        fn get_pushback(&self, play_area: &PlayArea) -> collision::Int;
        fn update_frame_mut(&mut self, input: &[InputState], play_area: &PlayArea);
        fn collision(&self) -> PositionedHitbox;
        fn velocity(&self) -> collision::Vec2;
        fn facing(&self) -> Facing;
        fn hitboxes(&self) -> Vec<PositionedHitbox>;
        fn hurtboxes(&self) -> Vec<PositionedHitbox>;
        fn get_attack_data(&self) -> Option<HitInfo>;
        fn prune_bullets(&mut self, play_area: &PlayArea);
        fn draw_ui(&self, ctx: &mut Context, bottom_line: graphics::Matrix4) -> GameResult<()>;
        fn draw(&self, ctx: &mut Context, world: graphics::Matrix4) -> GameResult<()>;
        fn draw_particles(&self, ctx: &mut Context, world: graphics::Matrix4) -> GameResult<()>;
        fn draw_bullets(&self, ctx: &mut Context, world: graphics::Matrix4) -> GameResult<()>;
        fn draw_shadow(&self, ctx: &mut Context, world: graphics::Matrix4) -> GameResult<()>;
        fn render_sound(&mut self, audio_device: &Device, sound_list: &SoundList, fps: u32) -> ();
        
        fn take_hit(&mut self, info: &HitType);
        fn deal_hit(&mut self, info: &HitType);
        fn would_be_hit(&self, input: &[InputState], touched: bool, total_info: Option<HitInfo>) -> HitType;
    
        fn save(&self) -> GameResult<Vec<u8>>;
        fn load(&mut self, value: &[u8]) -> GameResult<()>;
    );
    
    pub fn bullets_mut<'a>(&'a mut self) -> OpaqueBulletIterator<'a> {
        match self {
            Self::YuyukoPlayer(value) => value.bullets_mut(),
        }
    }
}
