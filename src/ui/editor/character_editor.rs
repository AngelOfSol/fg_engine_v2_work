use crate::assets::Assets;
use crate::character::components::AttackInfo;
use crate::character::state::EditorCharacterState;
use crate::character::PlayerCharacter;
use crate::ui::character::components::{AttacksUi, ParticlesUi, PropertiesUi, StatesUi};
use crate::ui::editor::{AnimationGroupEditor, AttackInfoEditor, StateEditor};
use crate::{
    app_state::{AppContext, AppState, Transition},
    graphics::animation_group::AnimationGroup,
};
use ggez::graphics;
use ggez::{Context, GameResult};
use imgui::*;
use std::path::PathBuf;
use std::rc::Rc;
use std::{
    cell::{Ref, RefCell, RefMut},
    ops::DerefMut,
};

pub trait ItemResource {
    type Output;
    fn get_from(&self) -> Option<Ref<Self::Output>>;
    fn get_from_mut(&self) -> Option<RefMut<Self::Output>>;
}

pub struct GraphicResource {
    pub graphic_id: String,
    pub data: Rc<RefCell<PlayerCharacter>>,
}
impl ItemResource for GraphicResource {
    type Output = AnimationGroup;
    fn get_from(&self) -> Option<Ref<Self::Output>> {
        let character = self.data.borrow();
        if character.graphics.contains_key(&self.graphic_id) {
            Some(Ref::map(character, |character| {
                character.graphics.get(&self.graphic_id).unwrap()
            }))
        } else {
            None
        }
    }
    fn get_from_mut(&self) -> Option<RefMut<Self::Output>> {
        let character = self.data.borrow_mut();
        if character.graphics.contains_key(&self.graphic_id) {
            Some(RefMut::map(character, |character| {
                character.graphics.get_mut(&self.graphic_id).unwrap()
            }))
        } else {
            None
        }
    }
}

pub struct GraphicAnimationResource {
    pub animation: String,
    pub data: Rc<RefCell<AnimationGroup>>,
}

impl ItemResource for GraphicAnimationResource {
    type Output = crate::graphics::Animation;
    fn get_from(&self) -> Option<Ref<Self::Output>> {
        let particle = self.data.borrow();
        Some(Ref::map(particle, |particle| {
            particle
                .animations
                .iter()
                .find(|item| item.name == self.animation)
                .unwrap()
        }))
    }
    fn get_from_mut(&self) -> Option<RefMut<Self::Output>> {
        let particle = self.data.borrow_mut();
        Some(RefMut::map(particle, |particle| {
            particle
                .animations
                .iter_mut()
                .find(|item| item.name == self.animation)
                .unwrap()
        }))
    }
}
pub struct AttackResource {
    pub attack: String,
    pub data: Rc<RefCell<PlayerCharacter>>,
}
impl ItemResource for AttackResource {
    type Output = AttackInfo;
    fn get_from(&self) -> Option<Ref<Self::Output>> {
        let character = self.data.borrow();
        if character.attacks.attacks.contains_key(&self.attack) {
            Some(Ref::map(character, |character| {
                character.attacks.attacks.get(&self.attack).unwrap()
            }))
        } else {
            None
        }
    }
    fn get_from_mut(&self) -> Option<RefMut<Self::Output>> {
        let character = self.data.borrow_mut();
        if character.attacks.attacks.contains_key(&self.attack) {
            Some(RefMut::map(character, |character| {
                character.attacks.attacks.get_mut(&self.attack).unwrap()
            }))
        } else {
            None
        }
    }
}

pub struct StateResource {
    pub state: String,
    pub data: Rc<RefCell<PlayerCharacter>>,
}

impl ItemResource for StateResource {
    type Output = EditorCharacterState;
    fn get_from(&self) -> Option<Ref<Self::Output>> {
        let character = self.data.borrow();
        if character.states.rest.contains_key(&self.state) {
            Some(Ref::map(character, |character| {
                character.states.rest.get(&self.state).unwrap()
            }))
        } else {
            None
        }
    }
    fn get_from_mut(&self) -> Option<RefMut<Self::Output>> {
        let character = self.data.borrow_mut();
        if character.states.rest.contains_key(&self.state) {
            Some(RefMut::map(character, |character| {
                character.states.rest.get_mut(&self.state).unwrap()
            }))
        } else {
            None
        }
    }
}

pub struct ParticleAnimationResource {
    pub animation: String,
    pub data: Rc<RefCell<crate::graphics::animation_group::AnimationGroup>>,
}

impl ItemResource for ParticleAnimationResource {
    type Output = crate::graphics::Animation;
    fn get_from(&self) -> Option<Ref<Self::Output>> {
        let particle = self.data.borrow();
        Some(Ref::map(particle, |particle| {
            particle
                .animations
                .iter()
                .find(|item| item.name == self.animation)
                .unwrap()
        }))
    }
    fn get_from_mut(&self) -> Option<RefMut<Self::Output>> {
        let particle = self.data.borrow_mut();
        Some(RefMut::map(particle, |particle| {
            particle
                .animations
                .iter_mut()
                .find(|item| item.name == self.animation)
                .unwrap()
        }))
    }
}

pub struct ParticleResource {
    pub particle: String,
    pub data: Rc<RefCell<PlayerCharacter>>,
}

impl ItemResource for ParticleResource {
    type Output = crate::graphics::animation_group::AnimationGroup;
    fn get_from(&self) -> Option<Ref<Self::Output>> {
        let character = self.data.borrow();
        if character.particles.particles.contains_key(&self.particle) {
            Some(Ref::map(character, |character| {
                character.particles.particles.get(&self.particle).unwrap()
            }))
        } else {
            None
        }
    }
    fn get_from_mut(&self) -> Option<RefMut<Self::Output>> {
        let character = self.data.borrow_mut();
        if character.particles.particles.contains_key(&self.particle) {
            Some(RefMut::map(character, |character| {
                character
                    .particles
                    .particles
                    .get_mut(&self.particle)
                    .unwrap()
            }))
        } else {
            None
        }
    }
}

pub struct StandaloneParticleResource {
    pub particle: Rc<RefCell<crate::graphics::animation_group::AnimationGroup>>,
}

impl From<crate::graphics::animation_group::AnimationGroup> for StandaloneParticleResource {
    fn from(value: crate::graphics::animation_group::AnimationGroup) -> Self {
        Self {
            particle: Rc::new(RefCell::new(value)),
        }
    }
}

impl ItemResource for StandaloneParticleResource {
    type Output = crate::graphics::animation_group::AnimationGroup;
    fn get_from(&self) -> Option<Ref<Self::Output>> {
        Some(self.particle.borrow())
    }
    fn get_from_mut(&self) -> Option<RefMut<Self::Output>> {
        Some(self.particle.borrow_mut())
    }
}

pub struct StateAnimationResource {
    pub data: Rc<RefCell<EditorCharacterState>>,
    pub name: String,
}

impl ItemResource for StateAnimationResource {
    type Output = crate::graphics::Animation;
    fn get_from(&self) -> Option<Ref<Self::Output>> {
        let state = self.data.borrow();
        if state.animations.iter().any(|item| item.name == self.name) {
            Some(Ref::map(state, |state| {
                state
                    .animations
                    .iter()
                    .find(|item| item.name == self.name)
                    .unwrap()
            }))
        } else {
            None
        }
    }
    fn get_from_mut(&self) -> Option<RefMut<Self::Output>> {
        let state = self.data.borrow_mut();
        if state.animations.iter().any(|item| item.name == self.name) {
            Some(RefMut::map(state, |state| {
                state
                    .animations
                    .iter_mut()
                    .find(|item| item.name == self.name)
                    .unwrap()
            }))
        } else {
            None
        }
    }
}

pub struct CharacterEditor {
    resource: Rc<RefCell<PlayerCharacter>>,
    assets: Rc<RefCell<Assets>>,
    transition: Transition,
    particle_ui_data: ParticlesUi,
    states_ui_data: StatesUi,
    attacks_ui_data: AttacksUi,
}

impl AppState for CharacterEditor {
    fn update(&mut self, _: &mut Context, _: &mut AppContext) -> GameResult<Transition> {
        let mut pc = self.resource.borrow_mut();
        let pc = pc.deref_mut();
        let (graphics, particles) = (&mut pc.graphics, &mut pc.particles);
        for key in pc.properties.character.graphic_name_iterator() {
            graphics
                .entry(key.clone())
                .or_insert_with(|| particles.particles[&key].clone());
        }

        Ok(std::mem::replace(&mut self.transition, Transition::None))
    }

    fn on_enter(&mut self, _: &mut Context, _: &mut AppContext) -> GameResult<()> {
        let mut pc = self.resource.borrow_mut();
        let pc = pc.deref_mut();
        let (graphics, particles) = (&mut pc.graphics, &mut pc.particles);
        dbg!(particles.particles.keys().collect::<Vec<_>>());
        Ok(())
    }
    fn draw(
        &mut self,
        ctx: &mut Context,
        AppContext { ref mut imgui, .. }: &mut AppContext,
    ) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);
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
                        let edit_result = self.states_ui_data.draw_ui(
                            ctx,
                            &mut self.assets.borrow_mut(),
                            ui,
                            &mut self.resource.borrow_mut().states,
                        );
                        if let Ok(Some(state)) = &edit_result {
                            let state = state.clone();
                            self.transition = Transition::Push(Box::new(
                                StateEditor::new(
                                    self.resource.clone(),
                                    self.assets.clone(),
                                    StateResource {
                                        state,
                                        data: self.resource.clone(),
                                    },
                                )
                                .unwrap(),
                            ));
                        }
                        editor_result = edit_result.map(|_| ());
                    });

                imgui::Window::new(im_str!("Graphics"))
                    .size([300.0, 526.0], Condition::Once)
                    .position([600.0, 20.0], Condition::Once)
                    .build(ui, || {
                        for key in self.resource.borrow().graphics.keys() {
                            ui.text(key);
                            ui.same_line(0.0);
                            if ui.small_button(&im_str!("Edit##{}", key)) {
                                self.transition = Transition::Push(Box::new(
                                    AnimationGroupEditor::new(
                                        self.assets.clone(),
                                        Box::new(GraphicResource {
                                            data: self.resource.clone(),
                                            graphic_id: key.clone(),
                                        }),
                                    )
                                    .unwrap(),
                                ));
                            }
                        }
                    });

                imgui::Window::new(im_str!("Attacks"))
                    .size([300.0, 526.0], Condition::Once)
                    .position([1200.0, 20.0], Condition::Once)
                    .build(ui, || {
                        let edit_change = self
                            .attacks_ui_data
                            .draw_ui(ui, &mut self.resource.borrow_mut().attacks);
                        if let Some(attack) = edit_change {
                            self.transition = Transition::Push(Box::new(
                                AttackInfoEditor::new(AttackResource {
                                    data: self.resource.clone(),
                                    attack,
                                })
                                .unwrap(),
                            ));
                        }
                    });
                ui.main_menu_bar(|| {
                    ui.menu(im_str!("Player Editor"), true, || {
                        if imgui::MenuItem::new(im_str!("Reset")).build(ui) {
                            *self.resource.borrow_mut() = PlayerCharacter::new();
                            self.particle_ui_data =
                                ParticlesUi::new(&self.resource.borrow_mut().particles);
                            self.states_ui_data = StatesUi::new(&self.resource.borrow_mut().states)
                        }
                        if imgui::MenuItem::new(im_str!("Save to file")).build(ui) {
                            if let Ok(nfd::Response::Okay(path)) =
                                nfd::open_save_dialog(Some("json"), None)
                            {
                                let mut path = PathBuf::from(path);
                                path.set_extension("json");
                                editor_result = PlayerCharacter::save(
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
                                match PlayerCharacter::load_from_json(
                                    ctx,
                                    &mut self.assets.borrow_mut(),
                                    PathBuf::from(path),
                                ) {
                                    Ok(character) => {
                                        *self.resource.borrow_mut() = character;
                                        self.particle_ui_data =
                                            ParticlesUi::new(&self.resource.borrow_mut().particles);
                                        self.states_ui_data =
                                            StatesUi::new(&self.resource.borrow_mut().states)
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

impl CharacterEditor {
    pub fn new(resource: Rc<RefCell<PlayerCharacter>>, assets: Rc<RefCell<Assets>>) -> Self {
        let particle_ui_data = ParticlesUi::new(&resource.borrow().particles);
        let states_ui_data = StatesUi::new(&resource.borrow().states);
        let attacks_ui_data = AttacksUi::new(&resource.borrow().attacks);
        Self {
            resource,
            assets,
            particle_ui_data,
            states_ui_data,
            attacks_ui_data,
            transition: Transition::None,
        }
    }
}
