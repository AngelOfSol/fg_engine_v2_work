use crate::assets::Assets;
use crate::character::PlayerCharacter;
use crate::graphics::animation_group::AnimationGroup;
use crate::ui::editor::character_editor::StandaloneAnimationGroupResource;
use crate::ui::editor::{AnimationGroupEditor, CharacterEditor};
use crate::{
    app_state::{AppContext, AppState, Transition},
    timeline::new::Timeline,
};
use ggez::graphics;
use ggez::{Context, GameResult};
use imgui::*;
use inspect_design::traits::{Inspect, InspectMut};
use inspect_design::Inspect;
use std::path::PathBuf;
use std::rc::Rc;
use std::{cell::RefCell, collections::HashMap};

#[derive(Inspect, Clone, Default)]
pub struct Simple {
    x: i32,
    y: i32,
    z: i32,
    x1: i32,
    y1: i32,
    z1: i32,
    x2: i32,
    y2: i32,
    z2: i32,
    #[tab = "Inner"]
    inner: Inner,
}

#[derive(Inspect, Default, Clone)]
struct Inner(String, String, Vec<f32>);

#[derive(Inspect, Default, Clone)]
pub struct Complex {
    test_enum: Option<(i32, f32)>,
    name: String,
    data: Vec<Simple>,
    other_data: Vec<(i32, i32)>,
    other_data2: Vec<i32>,
    number: f32,
    test: Simple,
    hash: HashMap<String, Simple>,
    hash2: HashMap<i32, i32>,
    edit: bool,
    frames: Vec<usize>,
    frame_idx: u32,
    duration: u32,
}

#[derive(Inspect)]
pub enum Test {
    Test,
    Variant(i32),
}

impl Default for Test {
    fn default() -> Self {
        Self::Test
    }
}

pub struct EditorMenu {
    next: Transition,
    timeline: crate::timeline::new::Timeline<Complex>,
    edit: bool,
    test_state: <Timeline<Complex> as Inspect>::State,
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

                imgui::Window::new(im_str!("test_window")).build(ui, || {
                    ui.checkbox(im_str!("Edit"), &mut self.edit);
                    ui.separator();
                    if self.edit {
                        self.timeline
                            .inspect_mut("timeline", &mut self.test_state, ui);
                    } else {
                        self.timeline.inspect("timeline", &mut self.test_state, ui);
                    }
                });

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
            edit: false,
            timeline: Timeline::with_data(
                vec![(
                    0,
                    Complex {
                        frame_idx: 0,
                        duration: 15,
                        frames: vec![0, 5, 10],
                        test_enum: Default::default(),
                        edit: false,
                        hash: vec![
                            (
                                "left".to_owned(),
                                Simple {
                                    x: 0,
                                    y: 0,
                                    z: 0,
                                    x1: 0,
                                    y1: 0,
                                    z1: 0,
                                    x2: 0,
                                    y2: 0,
                                    z2: 0,
                                    inner: Default::default(),
                                },
                            ),
                            (
                                "right".to_owned(),
                                Simple {
                                    x: 0,
                                    y: 0,
                                    z: 0,
                                    x1: 0,
                                    y1: 0,
                                    z1: 0,
                                    x2: 0,
                                    y2: 0,
                                    z2: 0,
                                    inner: Default::default(),
                                },
                            ),
                            (
                                "center".to_owned(),
                                Simple {
                                    x: 0,
                                    y: 0,
                                    z: 0,
                                    x1: 0,
                                    y1: 0,
                                    z1: 0,
                                    x2: 0,
                                    y2: 0,
                                    z2: 0,
                                    inner: Default::default(),
                                },
                            ),
                            (
                                "center2".to_owned(),
                                Simple {
                                    x: 0,
                                    y: 0,
                                    z: 0,
                                    x1: 0,
                                    y1: 0,
                                    z1: 0,
                                    x2: 0,
                                    y2: 0,
                                    z2: 0,
                                    inner: Default::default(),
                                },
                            ),
                        ]
                        .into_iter()
                        .collect(),
                        data: vec![
                            Simple {
                                x: 0,
                                y: 0,
                                z: 0,
                                x1: 0,
                                y1: 0,
                                z1: 0,
                                x2: 0,
                                y2: 0,
                                z2: 0,
                                inner: Default::default(),
                            };
                            20
                        ],
                        name: "ask".to_string(),
                        number: 0.0,
                        test: Simple {
                            x: 0,
                            y: 0,
                            z: 0,
                            x1: 0,
                            y1: 0,
                            z1: 0,
                            x2: 0,
                            y2: 0,
                            z2: 0,
                            inner: Default::default(),
                        },
                        other_data: vec![(3, 3), (6, 6), (9, 9), (12, 12)],
                        other_data2: vec![1],
                        hash2: vec![(3, 3), (6, 6), (9, 9), (12, 12)].into_iter().collect(),
                    },
                )],
                30,
            )
            .unwrap(),
            test_state: Default::default(),
            next: Transition::None,
        }
    }
}
