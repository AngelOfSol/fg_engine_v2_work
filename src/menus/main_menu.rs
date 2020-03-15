use super::gameplay::local_versus::LocalVersus;
use super::gameplay::netplay_versus::NetplayVersus;
use super::gameplay::training_mode::TrainingMode;
use super::gameplay::watch_replay::WatchReplay;
use super::gameplay::{
    CharacterSelect, ControllerSelect, LocalSelect, NetworkConnect, NetworkSelect,
};
use super::SettingsMenu;

use crate::app_state::{AppContext, AppState, Transition};
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
    NetworkSelect,
    WatchReplay(std::fs::File),
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
        ctx: &mut Context,
        _: &mut AppContext,
    ) -> GameResult<crate::app_state::Transition> {
        match std::mem::replace(&mut self.next, None) {
            Some(state) => match state {
                NextState::Quit => Ok(Transition::Pop),
                NextState::Editor => Ok(Transition::Push(Box::new(EditorMenu::new()))),
                NextState::Settings => Ok(Transition::Push(Box::new(SettingsMenu::new()))),
                NextState::NetworkSelect => {
                    let to_character_select = Box::new(|p1, id, net, target| {
                        if p1 {
                            Transition::Replace(Box::new(
                                CharacterSelect::<_, _, NetplayVersus>::new(
                                    LocalSelect::new(id),
                                    NetworkSelect::new(net, target),
                                ),
                            ))
                        } else {
                            Transition::Replace(Box::new(
                                CharacterSelect::<_, _, NetplayVersus>::new(
                                    NetworkSelect::new(net, target),
                                    LocalSelect::new(id),
                                ),
                            ))
                        }
                    });
                    //
                    Ok(Transition::Push(Box::new(NetworkConnect::new(
                        to_character_select,
                    )?)))
                }
                NextState::TrainingModeControllerSelect => {
                    let to_character_select =
                        Box::new(move |player_data: PlayerData<Option<GamepadId>>| {
                            Transition::Replace(Box::new(
                                CharacterSelect::<_, _, TrainingMode>::new(
                                    LocalSelect::new(player_data.p1().unwrap()),
                                    LocalSelect::new(player_data.p1().unwrap()),
                                ),
                            ))
                        });
                    Ok(Transition::Push(Box::new(ControllerSelect::new(
                        [true, false].into(),
                        to_character_select,
                    ))))
                }
                NextState::VsModeControllerSelect => {
                    let to_character_select =
                        Box::new(|player_data: PlayerData<Option<GamepadId>>| {
                            Transition::Replace(Box::new(
                                CharacterSelect::<_, _, LocalVersus>::new(
                                    LocalSelect::new(player_data.p1().unwrap()),
                                    LocalSelect::new(player_data.p2().unwrap()),
                                ),
                            ))
                        });
                    Ok(Transition::Push(Box::new(ControllerSelect::new(
                        [true, true].into(),
                        to_character_select,
                    ))))
                }
                NextState::WatchReplay(file) => {
                    let file = std::io::BufReader::new(file);
                    Ok(Transition::Push(Box::new(WatchReplay::new(ctx, file)?)))
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
                    if ui.small_button(im_str!("Network")) {
                        self.next = Some(NextState::NetworkSelect);
                    }
                    if ui.small_button(im_str!("Watch Replay")) {
                        let test = nfd::open_file_dialog(Some("rep"), None);
                        if let Ok(nfd::Response::Okay(file)) = test {
                            if let Ok(file) = std::fs::File::open(&file) {
                                self.next = Some(NextState::WatchReplay(file));
                            }
                        }
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
