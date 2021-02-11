pub mod data;
pub mod draw;
pub mod player_state;
pub mod smp;
pub mod typedefs;

mod clone;

use super::hit_info::ComboEffect;
use crate::{
    character::state::components::StateType,
    game_match::{
        sounds::{GlobalSoundList, SoundPath, SoundRenderer},
        FlashType,
    },
    game_object::state::{BulletHp, BulletTier},
    hitbox::PositionedHitbox,
};

use data::Data;
use fg_datastructures::math::collision;
use fg_input::Facing;
use hecs::{Entity, World};
use player_state::PlayerState;
use rodio::Device;
use std::cell::RefCell;
use typedefs::{Character, Timed};

pub struct Player<C: Character> {
    pub data: Data<C>,
    pub world: World,
    pub state: PlayerState<C>,
    pub ui_state: UiState,
    pub sound_renderer: SoundRenderer<SoundPath<C::Sound>>,
}

pub struct UiState {
    pub last_combo_state: Option<Timed<ComboEffect>>,
    pub combo_text: RefCell<Option<ggez::graphics::Text>>,
}

impl<C: Character> Player<C> {
    pub fn new(data: Data<C>) -> Self {
        Self {
            state: PlayerState::new(&data),
            data,
            world: World::with_registry(clone::registry_for::<C>()),
            ui_state: UiState {
                last_combo_state: None,
                combo_text: RefCell::new(None),
            },
            sound_renderer: SoundRenderer::new(),
        }
    }

    pub fn get_last_combo_state(&self) -> &Option<Timed<ComboEffect>> {
        &self.ui_state.last_combo_state
    }

    pub fn collision(&self) -> PositionedHitbox {
        self.data
            .get(&self.state)
            .hitboxes
            .collision
            .with_collision_position(self.state.position)
    }
    pub fn hitboxes(&self) -> impl Iterator<Item = PositionedHitbox> + Clone + '_ {
        self.data
            .get(&self.state)
            .hitboxes
            .hitbox
            .iter()
            .map(move |data| {
                data.boxes
                    .iter()
                    .map(|item| {
                        item.with_position_and_facing(self.state.position, self.state.facing)
                    })
                    .collect::<Vec<_>>()
            })
            .flatten()
    }
    pub fn hurtboxes(&self) -> impl Iterator<Item = PositionedHitbox> + Clone + '_ {
        self.data
            .get(&self.state)
            .hitboxes
            .hurtbox
            .iter()
            .map(move |item| item.with_position_and_facing(self.state.position, self.state.facing))
    }

    pub fn position(&self) -> collision::Vec2 {
        self.state.position
    }
    pub fn position_mut(&mut self) -> &mut collision::Vec2 {
        &mut self.state.position
    }
    pub fn velocity(&self) -> collision::Vec2 {
        self.state.velocity
    }
    pub fn facing(&self) -> Facing {
        self.state.facing
    }
    pub fn is_locked_out(&self) -> bool {
        self.state.lockout > 0
    }
    pub fn in_cutscene(&self) -> bool {
        self.data.get_next(&self.state).flags.cutscene
    }
    pub fn get_flash(&self) -> Option<FlashType> {
        self.data.get(&self.state).flags.flash
    }
    pub fn get_lockout(&self) -> (i32, bool) {
        let flags = self.data.get(&self.state).flags;
        (flags.lockout_timer, flags.reset_lockout_timer)
    }

    pub fn is_dead(&self) -> bool {
        self.state.dead
    }
    pub fn draw_order_priority(&self) -> i32 {
        if matches!(
            self.data.get(&self.state).state_type,
            StateType::Hitstun | StateType::Blockstun
        ) {
            -1
        } else {
            0
        }
    }
    pub fn health(&self) -> i32 {
        self.state.health
    }

    pub fn get_tier(&self, entity: Entity) -> Option<BulletTier> {
        self.world
            .get::<BulletHp>(entity)
            .map(|item| item.tier)
            .ok()
    }

    pub fn modify_lockout(&mut self, timer: i32, reset: bool) {
        self.state.lockout = timer + if reset { 0 } else { self.state.lockout };
    }

    pub fn render_sound(&mut self, audio_device: &Device, sound_list: &GlobalSoundList, fps: u32) {
        self.sound_renderer.render_frame(
            &audio_device,
            &self.data.sounds.data,
            &sound_list.data,
            &self.state.sound_state,
            fps,
        );
    }
}
