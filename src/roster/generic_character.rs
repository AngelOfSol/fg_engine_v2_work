pub mod hit_info;
pub mod move_id;

use crate::game_match::sounds::{GlobalSound, SoundList};
use crate::game_match::UiElements;
use crate::game_match::{FlashType, PlayArea};
use crate::graphics::animation_group::AnimationGroup;
use crate::hitbox::PositionedHitbox;
use crate::input::{Facing, InputState};
use crate::typedefs::{collision, graphics};
use crate::{assets::Assets, character::components::AttackInfo};
use crate::{character::state::components::GlobalGraphic, game_object::state::BulletTier};
use enum_dispatch::enum_dispatch;
use ggez::{Context, GameResult};
use hecs::{Entity, World};
use hit_info::{ComboEffect, HitEffect, HitResult, HitType, Source};
use rodio::Device;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum AllowedCancel {
    Always,
    Hit,
    Block,
}
pub struct OpponentState {
    pub position: collision::Vec2,
    pub in_hitstun: bool,
}

#[enum_dispatch(CharacterBehavior)]
pub trait GenericCharacterBehaviour {
    fn apply_pushback(&mut self, force: collision::Int);
    fn get_pushback(&self, play_area: &PlayArea) -> collision::Int;

    fn collision(&self) -> PositionedHitbox;
    fn hitboxes(&self) -> Vec<PositionedHitbox>;
    fn hurtboxes(&self) -> Vec<PositionedHitbox>;

    fn handle_refacing(&mut self, other_player: collision::Int);

    fn update_frame_mut(
        &mut self,
        input: &[InputState],
        opponent: OpponentState,
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

    fn would_be_hit(
        &self,
        input: &[InputState],
        attack_info: &AttackInfo,
        source: &Source,
        old_effect: Option<HitEffect>,
    ) -> HitResult;

    fn take_hit(&mut self, info: &HitEffect, play_area: &PlayArea);
    fn deal_hit(&mut self, info: &HitType);
    fn get_attack_data(&self) -> Option<Cow<'_, AttackInfo>>;
    fn get_last_combo_state(&self) -> Option<(ComboEffect, usize)>;
    fn in_hitstun(&self) -> bool;

    fn get_object_hitboxes(&self) -> Vec<(Entity, Vec<PositionedHitbox>)>;

    fn get_tier(&self, entity: Entity) -> Option<BulletTier>;

    fn on_touch_entity(&mut self, entity: Entity, tier: BulletTier);

    fn get_attack_data_entity(&self, entity: Entity) -> Option<(Facing, Cow<'_, AttackInfo>)>;
    fn deal_hit_entity(&mut self, entity: Entity, info: &HitType);
}

use super::yuyuko::YuyukoType;
use std::borrow::Cow;

#[derive(Clone)]
#[non_exhaustive]
pub enum OpaqueStateData {
    Yuyuko(
        super::character::player_state::PlayerState<YuyukoType>,
        World,
    ),
    #[allow(dead_code)]
    Broken,
}
