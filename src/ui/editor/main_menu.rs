use crate::app_state::{AppContext, AppState, Transition};
use crate::assets::Assets;
use crate::character::PlayerCharacter;
use crate::graphics::animation_group::AnimationGroup;
use crate::ui::editor::character_editor::StandaloneAnimationGroupResource;
use crate::ui::editor::{AnimationGroupEditor, CharacterEditor};
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

                imgui::Window::new(im_str!("Editor Menu")).build(ui, || {
                    if ui.small_button(im_str!("New Character")) {
                        let character = Rc::new(RefCell::new(PlayerCharacter::new()));
                        let assets = Rc::new(RefCell::new(Assets::new(ctx).unwrap()));
                        self.next =
                            Transition::Push(Box::new(CharacterEditor::new(character, assets)));
                    }
                    if ui.small_button(im_str!("Load Character")) {
                        if let Ok(nfd::Response::Okay(path)) =
                            nfd::open_file_dialog(Some("json"), None)
                        {
                            let assets = Rc::new(RefCell::new(Assets::new(ctx).unwrap()));
                            let character = PlayerCharacter::load_from_json(
                                ctx,
                                &mut assets.borrow_mut(),
                                PathBuf::from(path),
                            );
                            let character = character.map(|result| Rc::new(RefCell::new(result)));
                            if let Ok(character) = character {
                                self.next = Transition::Push(Box::new(CharacterEditor::new(
                                    character, assets,
                                )));
                            }
                        }
                    }
                    if ui.small_button(im_str!("New Animation Group")) {
                        let animation_group = AnimationGroup::new();
                        let assets = Rc::new(RefCell::new(Assets::new(ctx).unwrap()));
                        self.next = Transition::Push(Box::new(
                            AnimationGroupEditor::new(
                                assets,
                                Box::new(StandaloneAnimationGroupResource::from(animation_group)),
                            )
                            .unwrap(),
                        ));
                    }
                    if ui.small_button(im_str!("Load Animation Group")) {
                        if let Ok(nfd::Response::Okay(path)) =
                            nfd::open_file_dialog(Some("json"), None)
                        {
                            let assets = Rc::new(RefCell::new(Assets::new(ctx).unwrap()));
                            let animation_group = AnimationGroup::load_from_json(
                                ctx,
                                &mut assets.borrow_mut(),
                                PathBuf::from(path),
                            );

                            if let Ok(animation_group) = animation_group {
                                self.next = Transition::Push(Box::new(
                                    AnimationGroupEditor::new(
                                        assets,
                                        Box::new(StandaloneAnimationGroupResource::from(
                                            animation_group,
                                        )),
                                    )
                                    .unwrap(),
                                ));
                            } else if let Err(err) = animation_group {
                                dbg!(err);
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
