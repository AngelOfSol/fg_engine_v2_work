use crate::app_state::{AppContext, AppState, Transition};
use crate::imgui_extra::UiExtensions;
use ggez::{conf, graphics};
use ggez::{Context, GameResult};
use imgui::im_str;
use std::cmp::Ordering;

enum NextState {
    Back,
}

enum SaveStatus {
    FixFullscreen,
    FixResolution,
    None,
}

pub struct DisplaySettings {
    next: Option<NextState>,
    save: SaveStatus,
    display_sizes: Vec<(f32, f32)>,
    window_mode: ggez::conf::WindowMode,
}

impl DisplaySettings {
    pub fn new(ctx: &mut Context) -> Self {
        let mut display_sizes = vec![
            (1280.0, 720.0),
            (1366.0, 768.0),
            (1600.0, 900.0),
            (1920.0, 1080.0),
            (2560.0, 1440.0),
            {
                let window = ggez::graphics::window(&ctx);
                let monitor_size = window.get_current_monitor().get_dimensions();
                (monitor_size.width as f32, monitor_size.height as f32)
            },
        ];
        display_sizes.sort_by(|lhs, rhs| match lhs.0.partial_cmp(&rhs.0).unwrap() {
            Ordering::Equal => lhs.1.partial_cmp(&rhs.0).unwrap(),
            x => x,
        });
        display_sizes.dedup();
        Self {
            save: SaveStatus::None,
            next: None,
            display_sizes,
            window_mode: ggez::graphics::conf(ctx).window_mode.clone(),
        }
    }
}

impl AppState for DisplaySettings {
    fn update(
        &mut self,
        ctx: &mut Context,
        _: &mut AppContext,
    ) -> GameResult<crate::app_state::Transition> {
        self.save = match self.save {
            SaveStatus::FixResolution => {
                ggez::graphics::set_drawable_size(
                    ctx,
                    self.window_mode.width,
                    self.window_mode.height,
                )?;
                crate::graphics::prepare_screen_for_editor(ctx)?;
                let conf = ggez::graphics::conf(ctx).clone();
                ggez::filesystem::write_config(ctx, &conf)?;
                SaveStatus::None
            }
            SaveStatus::FixFullscreen => {
                ggez::graphics::set_fullscreen(ctx, self.window_mode.fullscreen_type)?;
                crate::graphics::prepare_screen_for_editor(ctx)?;
                let conf = ggez::graphics::conf(ctx).clone();
                ggez::filesystem::write_config(ctx, &conf)?;
                SaveStatus::None
            }
            SaveStatus::None => SaveStatus::None,
        };

        match std::mem::replace(&mut self.next, None) {
            Some(state) => match state {
                NextState::Back => Ok(Transition::Pop),
            },
            None => Ok(Transition::None),
        }
    }
    fn on_enter(&mut self, _: &mut Context, _: &mut AppContext) -> GameResult<()> {
        Ok(())
    }
    fn draw(
        &mut self,
        ctx: &mut Context,
        AppContext { ref mut imgui, .. }: &mut AppContext,
    ) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);

        let frame = imgui.frame();

        frame
            .run(|ui| {
                imgui::Window::new(im_str!("Main Menu")).build(ui, || {
                    if ui.combo_items(
                        im_str!("Display Mode"),
                        &mut self.window_mode.fullscreen_type,
                        &[conf::FullscreenType::True, conf::FullscreenType::Windowed],
                        &|item| {
                            im_str!(
                                "{}",
                                match item {
                                    conf::FullscreenType::Desktop => "Windowed Fullscreen",
                                    conf::FullscreenType::True => "Full Screen",
                                    conf::FullscreenType::Windowed => "Windowed",
                                }
                            )
                            .into()
                        },
                    ) {
                        self.save = SaveStatus::FixFullscreen;
                    }

                    if self.window_mode.fullscreen_type == conf::FullscreenType::Windowed {
                        let mut size = (self.window_mode.width, self.window_mode.height);

                        if ui.combo_items(
                            im_str!("Display Resolution"),
                            &mut size,
                            &self.display_sizes,
                            &|(width, height)| im_str!("{} x {}", width, height).into(),
                        ) {
                            self.save = SaveStatus::FixResolution;
                        }
                        self.window_mode.width = size.0;
                        self.window_mode.height = size.1;
                    }

                    if ui.small_button(im_str!("Back")) {
                        self.next = Some(NextState::Back);
                    }
                });
            })
            .render(ctx);

        graphics::present(ctx)?;

        Ok(())
    }
}
