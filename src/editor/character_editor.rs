use crate::editor::{AnimationEditor, EditorState, MessageData, Mode, StateEditor, Transition};

use ggez::{Context, GameResult};

use crate::assets::Assets;
use crate::imgui_wrapper::ImGuiWrapper;

use crate::character_state::CharacterState;

use imgui::*;

use crate::character::{ParticlesUi, PlayerCharacter, PropertiesUi, StatesUi};

use std::path::PathBuf;

use crate::graphics::Animation;

pub struct CharacterEditor {
    resource: PlayerCharacter,
    transition: Transition,
    particle_ui_data: ParticlesUi,
    states_ui_data: StatesUi,
}
impl CharacterEditor {
    pub fn new(resource: PlayerCharacter) -> Self {
        let particle_ui_data = ParticlesUi::new(&resource.particles);
        let states_ui_data = StatesUi::new(&resource.states);
        Self {
            resource,
            particle_ui_data,
            states_ui_data,
            transition: Transition::None,
        }
    }

    pub fn handle_message(&mut self, data: MessageData, mode: Mode) {
        match data {
            MessageData::State(state) => match mode {
                Mode::Standalone => (),
                Mode::New => {
                    self.resource.states.rest.insert(
                        self.resource.states.guarentee_unique_key("new state"),
                        state,
                    );
                }
                Mode::Edit(name) => {
                    self.resource.states.replace_state(name, state);
                }
            },
            MessageData::Animation(animation) => match mode {
                Mode::Standalone => (),
                Mode::New => {
                    let name = self
                        .resource
                        .particles
                        .guarentee_unique_key(&animation.name);
                    self.particle_ui_data.particle_keys.push(name.clone());
                    self.resource.particles.particles.insert(name, animation);
                }
                Mode::Edit(name) => {
                    self.resource.particles.particles.remove(&name);
                    let name = self
                        .resource
                        .particles
                        .guarentee_unique_key(&animation.name);
                    self.resource.particles.particles.insert(name, animation);
                }
            },
        }
    }

    pub fn update(&mut self) -> GameResult<Transition> {
        let ret = std::mem::replace(&mut self.transition, Transition::None);
        Ok(ret)
    }

    pub fn draw(
        &mut self,
        ctx: &mut Context,
        assets: &mut Assets,
        imgui: &mut ImGuiWrapper,
    ) -> GameResult<()> {
        let mut editor_result = Ok(());
        imgui
            .frame()
            .run(|ui| {
                ui.window(im_str!("Fields"))
                    .size([300.0, 526.0], Condition::Always)
                    .position([0.0, 20.0], Condition::Always)
                    .resizable(false)
                    .movable(false)
                    .collapsible(false)
                    .build(|| {
                        PropertiesUi::draw_ui(ui, &mut self.resource.properties);
                    });
                ui.window(im_str!("States"))
                    .size([300.0, 526.0], Condition::Always)
                    .position([300.0, 20.0], Condition::Always)
                    .resizable(false)
                    .movable(false)
                    .collapsible(false)
                    .build(|| {
                        let edit_result =
                            self.states_ui_data
                                .draw_ui(ctx, assets, ui, &mut self.resource.states);
                        if let Ok(Some(mode)) = &edit_result {
                            let state = match mode {
                                Mode::Edit(key) => self.resource.states.get_state(key).clone(),
                                _ => CharacterState::new(),
                            };
                            self.transition = Transition::Push(
                                Box::new(
                                    StateEditor::with_state(
                                        state,
                                        self.resource.particles.particles.keys().cloned().collect(),
                                    )
                                    .into(),
                                ),
                                mode.clone(),
                            );
                        }
                        editor_result = edit_result.map(|_| ());
                    });
                ui.window(im_str!("Particles"))
                    .size([300.0, 526.0], Condition::Always)
                    .position([600.0, 20.0], Condition::Always)
                    .resizable(false)
                    .movable(false)
                    .collapsible(false)
                    .build(|| {
                        let edit_change = self.particle_ui_data.draw_ui(
                            ctx,
                            assets,
                            ui,
                            &mut self.resource.particles,
                        );
                        if let Some(mode) = &edit_change {
                            let animation = match &mode {
                                Mode::Edit(name) => self
                                    .resource
                                    .particles
                                    .particles
                                    .values()
                                    .find(|item| &item.name == name)
                                    .cloned()
                                    .unwrap(),
                                _ => Animation::new("new"),
                            };
                            self.transition = Transition::Push(
                                Box::new(AnimationEditor::with_animation(animation).into()),
                                mode.clone(),
                            );
                        }
                    });
                ui.main_menu_bar(|| {
                    ui.menu(im_str!("Player Editor")).build(|| {
                        if ui.menu_item(im_str!("Reset")).build() {
                            self.resource = PlayerCharacter::new();
                            self.particle_ui_data = ParticlesUi::new(&self.resource.particles);
                            self.states_ui_data = StatesUi::new(&self.resource.states)
                        }
                        if ui.menu_item(im_str!("Save to file")).build() {
                            if let Ok(nfd::Response::Okay(path)) =
                                nfd::open_save_dialog(Some("json"), None)
                            {
                                let mut path = PathBuf::from(path);
                                path.set_extension("json");
                                editor_result =
                                    PlayerCharacter::save(ctx, assets, &self.resource, path);
                            }
                        }
                        if ui.menu_item(im_str!("Load from file")).build() {
                            if let Ok(nfd::Response::Okay(path)) =
                                nfd::open_file_dialog(Some("json"), None)
                            {
                                match PlayerCharacter::load_from_json(
                                    ctx,
                                    assets,
                                    PathBuf::from(path),
                                ) {
                                    Ok(character) => {
                                        self.resource = character;
                                        self.particle_ui_data =
                                            ParticlesUi::new(&self.resource.particles);
                                        self.states_ui_data = StatesUi::new(&self.resource.states)
                                    }
                                    Err(err) => editor_result = Err(err),
                                }
                            }
                        }
                        ui.separator();

                        if ui.menu_item(im_str!("Main Menu")).build() {
                            self.transition = Transition::Pop(None);
                        }
                    });
                });
            })
            .render(ctx);
        editor_result?;

        Ok(())
    }
}

impl Into<EditorState> for CharacterEditor {
    fn into(self) -> EditorState {
        EditorState::CharacterEditor(self)
    }
}
