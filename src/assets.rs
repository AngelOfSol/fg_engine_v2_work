use crate::game_match::{GameShader, Shadow, ShadowShader, ValueAlpha};
use ggez::graphics;
use ggez::{Context, GameResult};
#[derive(Clone, Debug)]
pub struct Assets {
    pub shadow_shader: ShadowShader,
    pub shader: GameShader,
}

impl Assets {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        Ok(Self {
            shader: graphics::Shader::new(
                ctx,
                "/shaders/vertex.glslv",
                "/shaders/fragment.glslf",
                ValueAlpha {
                    alpha: 0.5,
                    value: 1.0,
                },
                "ValueAlpha",
                Some(&[graphics::BlendMode::Alpha, graphics::BlendMode::Add]),
            )?,
            shadow_shader: graphics::Shader::new(
                ctx,
                "/shaders/shadow/vertex.glslv",
                "/shaders/shadow/fragment.glslf",
                Shadow { rate: 0.7 },
                "Shadow",
                Some(&[graphics::BlendMode::Alpha]),
            )?,
        })
    }
}
