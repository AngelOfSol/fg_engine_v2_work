use gfx::{self, *};
use ggez::graphics;
use ggez::{Context, GameResult};

gfx_defines! { constant UiProgress { rate: f32 = "u_Rate", value: f32 = "u_Value", alpha: f32 = "u_Alpha", } }
gfx_defines! { constant ValueAlpha { value: f32 = "u_Value", alpha: f32 = "u_Alpha", } }
gfx_defines! { constant Shadow { rate: f32 = "u_Rate",  } }

pub type GameShader = graphics::Shader<ValueAlpha>;
pub type ShadowShader = graphics::Shader<Shadow>;
pub type UiShader = graphics::Shader<UiProgress>;

#[derive(Clone, Debug)]
pub struct Assets {
    pub shadow_shader: ShadowShader,
    pub shader: GameShader,
    pub ui_shader: UiShader,
}

impl Assets {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        Ok(Self {
            shader: graphics::Shader::new(
                ctx,
                "/shaders/vertex.glslv",
                "/shaders/fragment.glslf",
                ValueAlpha {
                    alpha: 1.0,
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
            ui_shader: graphics::Shader::new(
                ctx,
                "/shaders/ui/vertex.glslv",
                "/shaders/ui/fragment.glslf",
                UiProgress {
                    rate: 1.0,
                    alpha: 1.0,
                    value: 1.0,
                },
                "UiProgress",
                Some(&[graphics::BlendMode::Alpha]),
            )?,
        })
    }
}
