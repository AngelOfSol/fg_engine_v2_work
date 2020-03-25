pub mod animation;
#[allow(clippy::module_inception)]
mod graphics;
pub mod keyframe;
pub mod particle;
mod sprite;

pub use animation::Animation;
pub use graphics::BlendMode;
pub use sprite::Sprite;

use ggez::{Context, GameResult};

pub fn prepare_screen_for_editor(ctx: &mut Context) -> GameResult {
    let conf = ggez::graphics::conf(ctx);
    let window_size = match conf.window_mode.fullscreen_type {
        ggez::conf::FullscreenType::True | ggez::conf::FullscreenType::Desktop => {
            let window = ggez::graphics::window(&ctx);
            let monitor_size = window.get_current_monitor().get_dimensions();
            (monitor_size.width as f32, monitor_size.height as f32)
        }
        ggez::conf::FullscreenType::Windowed => ggez::graphics::drawable_size(ctx),
    };
    ggez::graphics::set_screen_coordinates(
        ctx,
        ggez::graphics::Rect::new(0.0, 0.0, window_size.0, window_size.1),
    )
}
pub fn prepare_screen_for_game(ctx: &mut Context) -> GameResult {
    ggez::graphics::set_screen_coordinates(ctx, ggez::graphics::Rect::new(0.0, 0.0, 1280.0, 720.0))
}
