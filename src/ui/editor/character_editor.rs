use crate::assets::Assets;
use crate::character::components::bullets::BulletsUi;
use crate::character::components::particles::ParticlesUi;
use crate::character::components::properties::PropertiesUi;
use crate::character::components::states::StatesUi;
use crate::character::state::State;
use crate::character::PlayerCharacter;
use crate::graphics::Animation;
use crate::imgui_wrapper::ImGuiWrapper;
use crate::ui::editor::{
    AnimationEditor, BulletInfoEditor, EditorState, MessageData, Mode, StateEditor, Transition,
};
use ggez::{Context, GameResult};
use imgui::*;
use std::path::PathBuf;

pub struct CharacterEditor {
    resource: PlayerCharacter,
    transition: Transition,
    particle_ui_data: ParticlesUi,
    states_ui_data: StatesUi,
    bullet_ui_data: BulletsUi,
}
impl CharacterEditor {
    pub fn new(resource: PlayerCharacter) -> Self {
        let particle_ui_data = ParticlesUi::new(&resource.particles);
        let states_ui_data = StatesUi::new(&resource.states);
        let bullet_ui_data = BulletsUi::new(&resource.bullets);
        Self {
            resource,
            particle_ui_data,
            states_ui_data,
            bullet_ui_data,
            transition: Transition::None,
        }
    }

    pub fn handle_message(&mut self, data: MessageData, mode: Mode) {
        match data {
            MessageData::BulletInfo(bullet) => match mode {
                Mode::Standalone => (),
                Mode::New => {
                    self.resource.bullets.bullets.insert(
                        self.resource.bullets.guarentee_unique_key("new bullet"),
                        bullet,
                    );
                }
                Mode::Edit(name) => {
                    for (_, state) in self.resource.states.rest.iter_mut() {
                        for spawn in state.bullets.iter_mut() {
                            if spawn.bullet_id == name {
                                spawn.fix_properties(&bullet.properties);
                            }
                        }
                    }
                    self.resource.bullets.bullets.insert(name, bullet);
                }
            },
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
                imgui::Window::new(im_str!("Fields"))
                    .size([300.0, 526.0], Condition::Once)
                    .position([0.0, 20.0], Condition::Once)
                    .build(ui, || {
                        PropertiesUi::draw_ui(ui, &mut self.resource.properties);
                    });
                imgui::Window::new(im_str!("States"))
                    .size([300.0, 526.0], Condition::Once)
                    .position([300.0, 20.0], Condition::Once)
                    .build(ui, || {
                        let edit_result =
                            self.states_ui_data
                                .draw_ui(ctx, assets, ui, &mut self.resource.states);
                        if let Ok(Some(mode)) = &edit_result {
                            let state = match mode {
                                Mode::Edit(key) => self.resource.states.get_state(key).clone(),
                                _ => State::new(),
                            };
                            self.transition = Transition::Push(
                                Box::new(
                                    StateEditor::with_state(
                                        state,
                                        self.resource.particles.particles.keys().cloned().collect(),
                                        self.resource.states.rest.keys().cloned().collect(),
                                        self.resource
                                            .bullets
                                            .bullets
                                            .iter()
                                            .map(|(key, value)| {
                                                (key.clone(), value.properties.clone())
                                            })
                                            .collect(),
                                    )
                                    .into(),
                                ),
                                mode.clone(),
                            );
                        }
                        editor_result = edit_result.map(|_| ());
                    });
                imgui::Window::new(im_str!("Particles"))
                    .size([300.0, 526.0], Condition::Once)
                    .position([600.0, 20.0], Condition::Once)
                    .build(ui, || {
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
                imgui::Window::new(im_str!("Bullets"))
                    .size([300.0, 526.0], Condition::Once)
                    .position([900.0, 20.0], Condition::Once)
                    .build(ui, || {
                        let edit_change =
                            self.bullet_ui_data.draw_ui(ui, &mut self.resource.bullets);
                        if let Some(mode) = &edit_change {
                            let bullet = match &mode {
                                Mode::Edit(name) => self.resource.bullets.bullets[name].clone(),
                                _ => panic!("Attempting to edit bullet with no name."),
                            };
                            self.transition = Transition::Push(
                                Box::new(BulletInfoEditor::with_bullet(bullet).into()),
                                mode.clone(),
                            );
                        }
                    });
                ui.main_menu_bar(|| {
                    ui.menu(im_str!("Player Editor"), true, || {
                        if imgui::MenuItem::new(im_str!("Reset")).build(ui) {
                            self.resource = PlayerCharacter::new();
                            self.particle_ui_data = ParticlesUi::new(&self.resource.particles);
                            self.states_ui_data = StatesUi::new(&self.resource.states)
                        }
                        if imgui::MenuItem::new(im_str!("Save to file")).build(ui) {
                            if let Ok(nfd::Response::Okay(path)) =
                                nfd::open_save_dialog(Some("json"), None)
                            {
                                let mut path = PathBuf::from(path);
                                path.set_extension("json");
                                editor_result =
                                    PlayerCharacter::save(ctx, assets, &self.resource, path);
                            }
                        }
                        if imgui::MenuItem::new(im_str!("Load from file")).build(ui) {
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

                        if imgui::MenuItem::new(im_str!("Main Menu")).build(ui) {
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
