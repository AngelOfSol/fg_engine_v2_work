pub mod hit_info;
pub mod move_id;

pub mod impls;

use crate::game_match::sounds::{GlobalSound, SoundList};
use crate::game_match::UiElements;
use crate::game_match::{FlashType, PlayArea};
use crate::graphics::animation_group::AnimationGroup;
use crate::hitbox::PositionedHitbox;
use crate::input::{Facing, InputState};
use crate::typedefs::{collision, graphics};
use crate::{assets::Assets, character::components::AttackInfo};
use crate::{
    character::state::components::GlobalGraphic,
    game_match::sounds::{PlayerSoundState, SoundPath},
};
use enum_dispatch::enum_dispatch;
use ggez::{Context, GameResult};
use hit_info::new::{ComboEffect, HitResultNew, OnHitEffect, OnHitType, Source};
use rodio::Device;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub enum AllowedCancel {
    Always,
    Hit,
    Block,
}
#[derive(Debug, Clone)]
pub struct PlayerState<MoveId, SoundId, CommandId, AttackId> {
    pub velocity: collision::Vec2,
    pub position: collision::Vec2,
    pub current_state: (usize, MoveId),
    pub stun: Option<i32>,
    pub facing: Facing,
    pub air_actions: usize,
    pub spirit_gauge: i32,
    pub spirit_delay: i32,
    pub hitstop: i32,
    pub last_hit_using: Option<u64>,
    pub last_hit_using_new: Option<(AttackId, usize)>,
    pub current_combo_new: Option<ComboEffect>,
    pub health: i32,
    pub allowed_cancels: AllowedCancel,
    pub rebeat_chain: HashSet<CommandId>,
    pub smp_list: HashMap<CommandId, usize>,
    pub first_command: Option<(CommandId, usize)>,
    pub most_recent_command: (CommandId, usize),
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

    fn get_attack_data_new(&self) -> Option<&AttackInfo>;

    fn prune_bullets(&mut self, play_area: &PlayArea);

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
        last_combo_state: &Option<(ComboEffect, usize)>,
    ) -> GameResult<()>;

    fn get_last_combo_state(&self) -> &Option<(ComboEffect, usize)>;
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

    fn would_be_hit_new(
        &self,
        input: &[InputState],
        attack_info: &AttackInfo,
        source: &Source,
        combo_effect: Option<&ComboEffect>,
        old_effect: Option<OnHitEffect>,
    ) -> HitResultNew;
    fn take_hit_new(&mut self, info: &OnHitEffect, play_area: &PlayArea);
    fn deal_hit_new(&mut self, info: &OnHitType);

    fn get_current_combo(&self) -> Option<&ComboEffect>;
}

use super::yuyuko::YuyukoState;

#[derive(Clone)]
#[non_exhaustive]
pub enum OpaqueStateData {
    Yuyuko(YuyukoState),
    #[allow(dead_code)]
    Broken,
}
