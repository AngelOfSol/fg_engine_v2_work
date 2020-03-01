use crate::app_state::{AppContext, AppState, Transition};
use crate::assets::Assets;
use crate::character::PlayerCharacter;
use crate::ui::editor::CharacterEditor;
use ggez::graphics;
use ggez::{Context, GameResult};
use imgui::*;
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

pub struct EditorMenu {
    next: Transition,
}

impl AppState for EditorMenu {
    fn update(&mut self, _: &mut Context, _: &mut AppContext) -> GameResult<Transition> {
        let ret = std::mem::replace(&mut self.next, Transition::None);
        Ok(ret)
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
        imgui
            .frame()
            .run(|ui| {
                let id = ui.push_id("Editor Main Menu");
                // Window
                imgui::Window::new(im_str!("Editor Menu")).build(ui, || {
                    if ui.small_button(im_str!("New Character")) {
                        let character = Rc::new(RefCell::new(PlayerCharacter::new()));
                        let assets = Rc::new(RefCell::new(Assets::new()));
                        self.next =
                            Transition::Push(Box::new(CharacterEditor::new(character, assets)));
                    }
                    if ui.small_button(im_str!("Load Character")) {
                        if let Ok(nfd::Response::Okay(path)) =
                            nfd::open_file_dialog(Some("json"), None)
                        {
                            let assets = Rc::new(RefCell::new(Assets::new()));
                            let character = PlayerCharacter::load_from_json(
                                ctx,
                                &mut assets.borrow_mut(),
                                PathBuf::from(path),
                            );
                            let character = character.map(|result| Rc::new(RefCell::new(result)));
                            if character.is_err() {
                                dbg!(character.as_ref().unwrap_err());
                            }
                            if let Ok(character) = character {
                                self.next = Transition::Push(Box::new(CharacterEditor::new(
                                    character, assets,
                                )));
                            }
                        }
                    }
                    if ui.small_button(im_str!("Quit")) {
                        self.next = Transition::Pop;
                    }
                });
                id.pop(ui);
            })
            .render(ctx);
        graphics::present(ctx)
    }
}

impl EditorMenu {
    pub fn new() -> Self {
        EditorMenu {
            next: Transition::None,
        }
    }
}
