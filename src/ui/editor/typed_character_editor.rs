use crate::roster::character::data::Data;
use crate::roster::character::typedefs::Character;
use crate::ui::character::components::PropertiesUi;
use crate::{
    app_state::{AppContext, AppState, Transition},
    game_object::properties::{PropertyType, TryAsRef},
};
use crate::{assets::Assets, character::command::Command, input::Input};
use ggez::{graphics, GameError};
use ggez::{Context, GameResult};
use imgui::*;
use inspect_design::traits::{Inspect, InspectMut};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::rc::Rc;
use std::{cell::RefCell, collections::HashMap};
use std::{fs::File, path::PathBuf};
use strum::IntoEnumIterator;

use super::{
    typed_animation_group_editor::TypedAnimationGroupEditor,
    typed_attack_editor::TypedAttackEditor, typed_instance_data_editor::TypedInstanceDataEditor,
    typed_state_editor::TypedStateEditor,
};

pub struct TypedCharacterEditor<C: Character> {
    resource: Rc<RefCell<Data<C>>>,
    assets: Rc<RefCell<Assets>>,
    transition: Transition,
    command_map_state: <HashMap<C::Command, Command<C::State>> as Inspect>::State,
    input_map_state: <HashMap<Input, Vec<C::Command>> as Inspect>::State,
}

pub const EDITOR_BACKGROUND: [f32; 4] = [0.0823, 0.349, 0.3333, 1.0];

impl<C: Character> AppState for TypedCharacterEditor<C>
where
    Data<C>: Serialize + DeserializeOwned,
    PropertyType: TryAsRef<C::Graphic>,
{
    fn update(&mut self, _: &mut Context, _: &mut AppContext) -> GameResult<Transition> {
        Ok(std::mem::replace(&mut self.transition, Transition::None))
    }

    fn on_enter(&mut self, _: &mut Context, _: &mut AppContext) -> GameResult<()> {
        Ok(())
    }
    fn draw(
        &mut self,
        ctx: &mut Context,
        AppContext { ref mut imgui, .. }: &mut AppContext,
    ) -> GameResult<()> {
        graphics::clear(ctx, EDITOR_BACKGROUND.into());
        let mut editor_result = Ok(());

        imgui
            .frame()
            .run(|ui| {
                imgui::Window::new(im_str!("Fields"))
                    .size([300.0, 526.0], Condition::Once)
                    .position([0.0, 20.0], Condition::Once)
                    .build(ui, || {
                        PropertiesUi::draw_ui(ui, &mut self.resource.borrow_mut().properties);
                    });
                imgui::Window::new(im_str!("States"))
                    .size([300.0, 526.0], Condition::Once)
                    .position([300.0, 20.0], Condition::Once)
                    .build(ui, || {
                        for key in C::State::iter() {
                            {
                                let mut pc = self.resource.borrow_mut();
                                let _ = pc.states.entry(key).or_default();
                            }

                            ui.text(im_str!("{}", key));

                            ui.same_line(0.0);
                            if ui.small_button(&im_str!("Edit##State{}", key)) {
                                self.transition =
                                    Transition::Push(Box::new(TypedStateEditor::new(
                                        self.resource.clone(),
                                        key,
                                        self.assets.clone(),
                                    )));
                            }
                        }
                    });

                imgui::Window::new(im_str!("Graphics"))
                    .size([300.0, 526.0], Condition::Once)
                    .position([600.0, 20.0], Condition::Once)
                    .build(ui, || {
                        for key in C::Graphic::iter() {
                            {
                                let mut pc = self.resource.borrow_mut();
                                let _ = pc.graphics.entry(key).or_default();
                            }

                            ui.text(im_str!("{}", key));

                            ui.same_line(0.0);
                            if ui.small_button(&im_str!("Edit##Graphic{}", key)) {
                                self.transition =
                                    Transition::Push(Box::new(TypedAnimationGroupEditor::new(
                                        self.assets.clone(),
                                        self.resource.clone(),
                                        key,
                                    )));
                            }
                        }
                    });

                imgui::Window::new(im_str!("Instance Data"))
                    .size([300.0, 526.0], Condition::Once)
                    .position([900.0, 20.0], Condition::Once)
                    .build(ui, || {
                        for key in C::ObjectData::iter() {
                            ui.text(im_str!("{}", key));

                            ui.same_line(0.0);
                            if ui.small_button(&im_str!("Edit##Instance{}", key)) {
                                self.transition =
                                    Transition::Push(Box::new(TypedInstanceDataEditor::new(
                                        self.assets.clone(),
                                        self.resource.clone(),
                                        key,
                                    )));
                            }
                        }
                    });
                imgui::Window::new(im_str!("Attacks"))
                    .size([600.0, 526.0], Condition::Once)
                    .position([1200.0, 20.0], Condition::Once)
                    .build(ui, || {
                        TabBar::new(im_str!("attack tabs")).build(ui, || {
                            TabItem::new(im_str!("Attacks")).build(ui, || {
                                for key in C::Attack::iter() {
                                    {
                                        let mut pc = self.resource.borrow_mut();
                                        let _ = pc.attacks.entry(key).or_default();
                                    }
                                    ui.text(im_str!("{}", key));

                                    ui.same_line(0.0);
                                    if ui.small_button(&im_str!("Edit##Attack{}", key)) {
                                        self.transition = Transition::Push(Box::new(
                                            TypedAttackEditor::new(key, self.resource.clone()),
                                        ));
                                    }
                                }
                            });

                            TabItem::new(im_str!("Command List")).build(ui, || {
                                let mut pc = self.resource.borrow_mut();
                                for key in C::Command::iter() {
                                    let _ = pc.command_map.entry(key).or_default();
                                }
                                pc.command_map
                                    .inspect_mut("", &mut self.command_map_state, ui)
                            });

                            TabItem::new(im_str!("Input Map")).build(ui, || {
                                self.resource.borrow_mut().input_map.inspect_mut(
                                    "",
                                    &mut self.input_map_state,
                                    ui,
                                )
                            })
                        });
                    });

                ui.main_menu_bar(|| {
                    ui.menu(im_str!("Player Editor"), true, || {
                        if imgui::MenuItem::new(im_str!("Reset")).build(ui) {
                            *self.resource.borrow_mut() = Data::default();
                        }
                        if imgui::MenuItem::new(im_str!("Save Only Data")).build(ui) {
                            if let Ok(nfd::Response::Okay(path)) =
                                nfd::open_save_dialog(Some("json"), None)
                            {
                                let mut path = PathBuf::from(path);
                                path.set_extension("json");
                                let mut json = File::create(&path).unwrap();
                                serde_json::to_writer(
                                    &mut json,
                                    std::ops::Deref::deref(&self.resource.borrow()),
                                )
                                .map_err(|err| GameError::FilesystemError(format!("{}", err)))
                                .unwrap();
                            }
                        }
                        if imgui::MenuItem::new(im_str!("Save to file")).build(ui) {
                            if let Ok(nfd::Response::Okay(path)) =
                                nfd::open_save_dialog(Some("json"), None)
                            {
                                let mut path = PathBuf::from(path);
                                path.set_extension("json");
                                editor_result = Data::save(
                                    ctx,
                                    &mut self.assets.borrow_mut(),
                                    &self.resource.borrow_mut(),
                                    path,
                                );
                            }
                        }
                        if imgui::MenuItem::new(im_str!("Load from file")).build(ui) {
                            if let Ok(nfd::Response::Okay(path)) =
                                nfd::open_file_dialog(Some("json"), None)
                            {
                                match Data::new_with_path(
                                    ctx,
                                    &mut self.assets.borrow_mut(),
                                    PathBuf::from(path),
                                ) {
                                    Ok(character) => {
                                        *self.resource.borrow_mut() = character;
                                    }
                                    Err(err) => editor_result = Err(err),
                                }
                            }
                        }
                        ui.separator();

                        if imgui::MenuItem::new(im_str!("Main Menu")).build(ui) {
                            self.transition = Transition::Pop;
                        }
                    });
                });
            })
            .render(ctx);
        editor_result?;

        graphics::present(ctx)
    }
}

impl<C: Character> TypedCharacterEditor<C> {
    pub fn new(resource: Rc<RefCell<Data<C>>>, assets: Rc<RefCell<Assets>>) -> Self {
        Self {
            resource,
            assets,
            transition: Transition::None,
            command_map_state: Default::default(),
            input_map_state: Default::default(),
        }
    }
}
