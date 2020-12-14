pub mod combo_state;
pub mod extra_data;
pub mod hit_info;
pub mod move_id;

pub mod impls;

use crate::assets::Assets;
use crate::game_match::sounds::{GlobalSound, SoundList};
use crate::game_match::UiElements;
use crate::game_match::{FlashType, PlayArea};
use crate::graphics::animation_group::AnimationGroup;
use crate::hitbox::PositionedHitbox;
use crate::input::{Facing, InputState};
use crate::typedefs::{collision, graphics};
use crate::{
    character::state::components::GlobalGraphic,
    game_match::sounds::{PlayerSoundState, SoundPath},
};
use enum_dispatch::enum_dispatch;
use ggez::{Context, GameResult};
use hit_info::{HitAction, HitEffect, HitResult};
use rodio::Device;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct PlayerState<MoveId, SoundId> {
    pub velocity: collision::Vec2,
    pub position: collision::Vec2,
    pub current_state: (usize, MoveId),
    pub extra_data: ExtraData,
    pub facing: Facing,
    pub air_actions: usize,
    pub spirit_gauge: i32,
    pub spirit_delay: i32,
    pub hitstop: i32,
    pub last_hit_using: Option<u64>,
    pub current_combo: Option<ComboState>,
    pub health: i32,
    pub allowed_cancels: AllowedCancel,
    pub rebeat_chain: HashSet<MoveId>,
    pub should_pushback: bool,
    pub sound_state: PlayerSoundState<SoundPath<SoundId>>,
    pub meter: i32,
    pub lockout: i32,
    pub dead: bool,
}

#[enum_dispatch(CharacterBehavior)]
pub trait GenericCharacterBehaviour {
    fn apply_pushback(&mut self, force: collision::Int);
    fn get_pushback(&self, play_area: &PlayArea) -> collision::Int;

    fn collision(&self) -> PositionedHitbox;
    fn hitboxes(&self) -> Vec<PositionedHitbox>;
    fn hurtboxes(&self) -> Vec<PositionedHitbox>;

    fn get_attack_data(&self) -> Option<HitAction>;

    fn prune_bullets(&mut self, play_area: &PlayArea);

    fn would_be_hit(
        &self,
        input: &[InputState],
        total_info: HitAction,
        effect: Option<HitEffect>,
    ) -> (Option<HitEffect>, Option<HitResult>);

    fn take_hit(&mut self, info: HitEffect, play_area: &PlayArea);
    fn deal_hit(&mut self, info: &HitResult);
    fn handle_refacing(&mut self, other_player: collision::Int);

    fn update_frame_mut(
        &mut self,
        input: &[InputState],
        opponent_position: collision::Vec2,
        play_area: &PlayArea,
        global_graphics: &HashMap<GlobalGraphic, AnimationGroup>,
    );
    fn update_cutscene(&mut self, play_area: &PlayArea);
    fn update_no_input(
        &mut self,
        play_area: &PlayArea,
        global_graphics: &HashMap<GlobalGraphic, AnimationGroup>,
    );

    #[allow(clippy::clippy::too_many_arguments)]
    fn draw_ui(
        &self,
        ctx: &mut Context,
        assets: &Assets,
        ui: &UiElements,
        bottom_line: graphics::Matrix4,
        flipped: bool,
        wins: usize,
        first_to: usize,
    ) -> GameResult<()>;

    fn draw(&self, ctx: &mut Context, assets: &Assets, world: graphics::Matrix4) -> GameResult<()>;

    fn draw_objects(
        &self,
        ctx: &mut Context,
        assets: &Assets,
        world: graphics::Matrix4,
        global_graphics: &HashMap<GlobalGraphic, AnimationGroup>,
    ) -> GameResult<()>;

    fn draw_shadow(
        &self,
        ctx: &mut Context,
        assets: &Assets,
        world: graphics::Matrix4,
    ) -> GameResult<()>;

    fn render_sound(
        &mut self,
        audio_device: &Device,
        sound_list: &SoundList<GlobalSound>,
        fps: u32,
    );

    fn position(&self) -> collision::Vec2;
    fn position_mut(&mut self) -> &mut collision::Vec2;

    fn velocity(&self) -> collision::Vec2;
    fn facing(&self) -> Facing;
    fn in_cutscene(&self) -> bool;
    fn draw_order_priority(&self) -> i32;

    fn save(&self) -> GameResult<OpaqueStateData>;
    fn load(&mut self, value: OpaqueStateData) -> GameResult<()>;

    fn get_flash(&self) -> Option<FlashType>;
    fn get_lockout(&self) -> (i32, bool);
    fn modify_lockout(&mut self, timer: i32, reset: bool);
    fn is_locked_out(&self) -> bool;
    fn validate_position(&mut self, play_area: &PlayArea);
    fn is_dead(&self) -> bool;
    fn health(&self) -> i32;

    fn reset_to_position_roundstart(
        &mut self,
        play_area: &PlayArea,
        position: collision::Int,
        facing: Facing,
    );
    fn reset_to_position_gamestart(
        &mut self,
        play_area: &PlayArea,
        position: collision::Int,
        facing: Facing,
    );
}

use self::{
    combo_state::{AllowedCancel, ComboState},
    extra_data::ExtraData,
};

use super::yuyuko::YuyukoState;

#[derive(Clone)]
#[non_exhaustive]
pub enum OpaqueStateData {
    Yuyuko(YuyukoState),
    #[allow(dead_code)]
    Broken,
}
