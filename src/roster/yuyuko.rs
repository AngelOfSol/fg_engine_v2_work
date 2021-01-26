mod attacks;
mod commands;
mod graphic;
pub mod object;
mod sounds;
mod state;

use super::{
    character::{
        draw::UiContext,
        typedefs::{state::StateConsts, Character, Timed},
        Player,
    },
    hit_info::{ComboEffect, HitEffect, HitResult, HitType, Source},
    OpponentState,
};
use crate::character::components::AttackInfo;
use crate::character::state::components::GlobalGraphic;
use crate::game_match::sounds::GlobalSound;
use crate::game_match::{FlashType, PlayArea, UiElements};
use crate::graphics::animation_group::AnimationGroup;
use crate::hitbox::PositionedHitbox;
use crate::input::{Facing, InputState};
use crate::roster::generic_character::GenericCharacterBehaviour;
use crate::roster::generic_character::OpaqueStateData;
use crate::typedefs::collision;
use crate::typedefs::graphics;
use crate::{assets::Assets, game_object::state::BulletTier};

use ggez::{Context, GameResult};
use hecs::Entity;
use rodio::Device;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

pub use attacks::Attack;
pub use commands::Command;
pub use graphic::Graphic;
pub use object::ObjectData;
pub use sounds::Sound;
pub use state::State;

#[derive(Default, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct YuyukoType;

impl Character for YuyukoType {
    type Sound = Sound;
    type State = State;
    type Attack = Attack;
    type Graphic = Graphic;
    type ObjectData = ObjectData;
    type Command = Command;
    type StaticData = ();

    fn round_start_reset(&mut self, _data: &super::character::data::Data<Self>) {}
}
impl StateConsts for State {
    const GAME_START: Self = Self::RoundStart;
    const ROUND_START: Self = Self::Stand;
    const DEAD: Self = Self::Dead;
    const HIT_GROUND: Self = Self::HitGround;
    const AIR_IDLE: Self = Self::AirIdle;
    const STAND: Self = Self::Stand;
    const CROUCH: Self = Self::Crouch;
    const UNTECH: Self = Self::Untech;
    const FLY: Self = Self::Fly;
    const FLY_END: Self = Self::FlyEnd;
    const AIR_HITSTUN: Self = Self::HitstunAirStart;
    const GROUND_HITSTUN: Self = Self::HitstunStandStart;
    const GUARD_CRUSH: Self = Self::GuardCrush;
    const AIR_BLOCKSTUN: Self = Self::BlockstunAirStart;
    const STAND_BLOCKSTUN: Self = Self::BlockstunStandStart;
    const CROUCH_BLOCKSTUN: Self = Self::BlockstunCrouchStart;
    const STAND_WRONG_BLOCKSTUN: Self = Self::WrongblockStandStart;
    const CROUCH_WRONG_BLOCKSTUN: Self = Self::WrongblockCrouchStart;
}

impl GenericCharacterBehaviour for Player<YuyukoType> {
    fn apply_pushback(&mut self, force: collision::Int) {
        self.state.apply_pushback(&self.data, force);
    }

    fn get_pushback(&self, play_area: &PlayArea) -> collision::Int {
        self.state.get_pushback(&self.data, play_area)
    }

    fn collision(&self) -> PositionedHitbox {
        self.collision()
    }

    fn hitboxes(&self) -> Vec<PositionedHitbox> {
        self.hitboxes().collect()
    }

    fn hurtboxes(&self) -> Vec<PositionedHitbox> {
        self.hurtboxes().collect()
    }

    fn handle_refacing(&mut self, other_player: collision::Int) {
        self.state.handle_refacing(&self.data, other_player)
    }

    fn update_frame_mut(
        &mut self,
        input: &[InputState],
        opponent: OpponentState,
        play_area: &PlayArea,
        global_graphics: &std::collections::HashMap<GlobalGraphic, AnimationGroup>,
    ) {
        self.state.update_frame_mut(
            &mut self.world,
            &mut self.ui_state.last_combo_state,
            &self.data,
            input,
            opponent,
            play_area,
            global_graphics,
        )
    }

    fn update_cutscene(&mut self, play_area: &PlayArea) {
        self.state.update_cutscene(&self.data, play_area)
    }

    fn update_no_input(
        &mut self,
        play_area: &PlayArea,
        global_graphics: &std::collections::HashMap<GlobalGraphic, AnimationGroup>,
    ) {
        self.state.update_no_input(
            &mut self.world,
            &mut self.ui_state.last_combo_state,
            &self.data,
            play_area,
            global_graphics,
        )
    }

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
    ) -> GameResult<()> {
        self.draw_ui(
            ctx,
            assets,
            UiContext {
                ui,
                bottom_line,
                flipped,
                wins,
                first_to,
                last_combo_state: &last_combo_state.as_ref().map(|item| Timed {
                    time: item.1,
                    id: item.0.clone(),
                }),
            },
        )
    }

    fn draw(&self, ctx: &mut Context, assets: &Assets, world: graphics::Matrix4) -> GameResult<()> {
        let _ = self.save();
        self.draw(ctx, assets, world)
    }

    fn draw_objects(
        &self,
        ctx: &mut Context,
        assets: &Assets,
        world: graphics::Matrix4,
        global_graphics: &std::collections::HashMap<GlobalGraphic, AnimationGroup>,
    ) -> GameResult<()> {
        self.draw_objects(ctx, assets, world, global_graphics)
    }

    fn draw_shadow(
        &self,
        ctx: &mut Context,
        assets: &Assets,
        world: graphics::Matrix4,
    ) -> GameResult<()> {
        self.draw_shadow(ctx, assets, world)
    }

    fn render_sound(
        &mut self,
        audio_device: &Device,
        sound_list: &crate::game_match::sounds::SoundList<GlobalSound>,
        fps: u32,
    ) {
        self.render_sound(audio_device, sound_list, fps)
    }

    fn position(&self) -> collision::Vec2 {
        self.position()
    }

    fn position_mut(&mut self) -> &mut collision::Vec2 {
        self.position_mut()
    }

    fn velocity(&self) -> collision::Vec2 {
        self.velocity()
    }

    fn facing(&self) -> Facing {
        self.facing()
    }

    fn in_cutscene(&self) -> bool {
        self.in_cutscene()
    }

    fn draw_order_priority(&self) -> i32 {
        self.draw_order_priority()
    }

    fn save(&self) -> GameResult<OpaqueStateData> {
        let w = self.world.clone();
        Ok(OpaqueStateData::Yuyuko(self.state.clone(), w))
    }

    fn load(&mut self, value: OpaqueStateData) -> GameResult<()> {
        match value {
            OpaqueStateData::Yuyuko(state, world) => {
                self.state = state;
                self.world = world;
            }
            _ => unreachable!(),
        }

        Ok(())
    }

    fn get_flash(&self) -> Option<FlashType> {
        self.get_flash()
    }

    fn get_lockout(&self) -> (i32, bool) {
        self.get_lockout()
    }

    fn modify_lockout(&mut self, timer: i32, reset: bool) {
        self.modify_lockout(timer, reset)
    }

    fn is_locked_out(&self) -> bool {
        self.is_locked_out()
    }

    fn validate_position(&mut self, play_area: &PlayArea) {
        self.state.validate_position(&self.data, play_area)
    }

    fn is_dead(&self) -> bool {
        self.is_dead()
    }

    fn health(&self) -> i32 {
        self.health()
    }

    fn reset_to_position_roundstart(
        &mut self,
        play_area: &PlayArea,
        position: collision::Int,
        facing: Facing,
    ) {
        self.state
            .reset_to_position_roundstart(&self.data, play_area, position, facing)
    }

    fn reset_to_position_gamestart(
        &mut self,
        play_area: &PlayArea,
        position: collision::Int,
        facing: Facing,
    ) {
        self.state
            .reset_to_position_gamestart(&self.data, play_area, position, facing)
    }

    fn would_be_hit(
        &self,
        input: &[InputState],
        attack_info: &AttackInfo,
        source: &Source,
        old_effect: Option<HitEffect>,
    ) -> HitResult {
        self.state
            .would_be_hit(&self.data, input, attack_info, source, old_effect)
    }

    fn take_hit(&mut self, info: &HitEffect, play_area: &PlayArea) {
        self.state.take_hit(&self.data, info, play_area)
    }

    fn deal_hit(&mut self, info: &HitType) {
        self.state.deal_hit(&self.data, info)
    }

    fn get_attack_data(&self) -> Option<Cow<'_, AttackInfo>> {
        self.state.get_attack_data(&self.data)
    }
    fn get_attack_data_entity(&self, entity: Entity) -> Option<(Facing, Cow<'_, AttackInfo>)> {
        self.state
            .get_attack_data_entity(&self.data, &self.world, entity)
    }

    fn get_last_combo_state(&self) -> Option<(ComboEffect, usize)> {
        self.get_last_combo_state()
            .as_ref()
            .map(|item| (item.id.clone(), item.time))
    }

    fn in_hitstun(&self) -> bool {
        self.state.in_hitstun(&self.data)
    }

    fn get_object_hitboxes(&self) -> Vec<(Entity, Vec<PositionedHitbox>)> {
        self.state.get_object_hitboxes(&self.world, &self.data)
    }

    fn get_tier(&self, entity: Entity) -> Option<BulletTier> {
        self.get_tier(entity)
    }
    fn on_touch_entity(&mut self, entity: Entity, tier: BulletTier) {
        self.state
            .on_touch_entity(&mut self.world, &self.data, entity, tier)
    }

    fn deal_hit_entity(&mut self, entity: Entity, info: &HitType) {
        self.state
            .deal_hit_entity(&mut self.world, &self.data, entity, info);
    }
}
