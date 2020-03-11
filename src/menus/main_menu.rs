use super::gameplay::training_mode::TrainingMode;
use super::gameplay::Character;
use super::gameplay::{CharacterSelect, ControllerSelect, SelectBy};
use super::SettingsMenu;
use crate::app_state::{AppContext, AppState, Transition};
use crate::input::control_scheme::PadControlScheme;
use crate::typedefs::player::PlayerData;
use crate::ui::editor::EditorMenu;
use ggez::graphics;
use ggez::{Context, GameResult};
use gilrs::GamepadId;
use imgui::im_str;

enum NextState {
    Quit,
    Editor,
    Settings,
    TrainingModeControllerSelect,
    VsModeControllerSelect,
}

pub struct MainMenu {
    next: Option<NextState>,
}

impl MainMenu {
    pub fn new() -> Self {
        Self { next: None }
    }
}

impl AppState for MainMenu {
    fn update(
        &mut self,
        _: &mut Context,
        _: &mut AppContext,
    ) -> GameResult<crate::app_state::Transition> {
        match std::mem::replace(&mut self.next, None) {
            Some(state) => match state {
                NextState::Quit => Ok(Transition::Pop),
                NextState::Editor => Ok(Transition::Push(Box::new(EditorMenu::new()))),
                NextState::Settings => Ok(Transition::Push(Box::new(SettingsMenu::new()))),
                NextState::TrainingModeControllerSelect => {
                    let to_training_mode = Box::new(|ctx: &mut Context, player_data, controls| {
                        Transition::Replace(Box::new(TrainingMode::new(ctx, controls).unwrap()))
                    });

                    let to_character_select =
                        Box::new(move |player_data: PlayerData<Option<GamepadId>>| {
                            Transition::Replace(Box::new(CharacterSelect::new(
                                [SelectBy::Local(player_data.p1().unwrap()); 2].into(),
                                to_training_mode,
                            )))
                            //
                        });
                    Ok(Transition::Push(Box::new(ControllerSelect::new(
                        [true, false].into(),
                        to_character_select,
                    ))))
                }
                NextState::VsModeControllerSelect => {
                    let to_character_select =
                        Box::new(|player_data: PlayerData<Option<GamepadId>>| {
                            Transition::Replace(Box::new(CharacterSelect::new(
                                [
                                    SelectBy::Local(player_data.p1().unwrap()),
                                    SelectBy::Local(player_data.p2().unwrap()),
                                ]
                                .into(),
                                Box::new(|_, _, _| Transition::Pop),
                            )))
                        });
                    Ok(Transition::Push(Box::new(ControllerSelect::new(
                        [true, true].into(),
                        to_character_select,
                    ))))
                }
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
                    if ui.small_button(im_str!("VS")) {
                        self.next = Some(NextState::VsModeControllerSelect);
                    }
                    if ui.small_button(im_str!("Training Mode")) {
                        self.next = Some(NextState::TrainingModeControllerSelect);
                    }
                    if ui.small_button(im_str!("Settings")) {
                        self.next = Some(NextState::Settings);
                    }
                    if ui.small_button(im_str!("Editor")) {
                        self.next = Some(NextState::Editor);
                    }
                    if ui.small_button(im_str!("Quit")) {
                        self.next = Some(NextState::Quit);
                    }
                });
            })
            .render(ctx);

        graphics::present(ctx)?;

        Ok(())
    }
}
