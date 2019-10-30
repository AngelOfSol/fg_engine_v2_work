use crate::assets::Assets;
use crate::game_match::PlayArea;
use crate::input::Facing;
use crate::typedefs::{collision, graphics};
use ggez::{Context, GameResult};

pub trait GenericBulletState {
    type Resource;

    fn alive(&self, data: &Self::Resource, play_area: &PlayArea) -> bool;
    fn update(&mut self, data: &Self::Resource);
    fn draw(
        &self,
        ctx: &mut Context,
        data: &Self::Resource,
        assets: &Assets,
        world: graphics::Matrix4,
    ) -> GameResult<()>;
}
pub trait GenericBulletSpawn {
    type Output;
    fn get_spawn_frame(&self) -> usize;
    fn instantiate(&self, position: collision::Vec2, facing: Facing) -> Self::Output;
}
