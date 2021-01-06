use crate::typedefs::graphics::{Matrix4, Vec3};
use crate::{
    app_state::{AppContext, AppState, Transition},
    character::state::components::GlobalGraphic,
    graphics::animation_group::AnimationGroup,
    hitbox::Hitbox,
    timeline::Timeline,
};
use crate::{assets::Assets, character::PlayerCharacter};
use crate::{game_match::load_global_graphics, game_object::properties::PropertyType};
use ggez::graphics;
use ggez::graphics::{Color, DrawParam, Mesh};
use ggez::{Context, GameResult};
use imgui::*;
use inspect_design::traits::Inspect;
use inspect_design::traits::InspectMut;
use std::rc::Rc;
use std::{cell::RefCell, collections::HashMap};
use strum::IntoEnumIterator;

enum Status {
    DoneAndQuit,
    NotDone,
}

pub struct InstanceDataEditor {
    frame: usize,
    path: String,
    resource: Rc<RefCell<PlayerCharacter>>,
    done: Status,
    selected_property: PropertyType,
    transition: Transition,
    assets: Rc<RefCell<Assets>>,
    globals: HashMap<GlobalGraphic, AnimationGroup>,
    inspect_state: <PropertyType as Inspect>::State,
}

impl AppState for InstanceDataEditor {
    fn update(&mut self, ctx: &mut Context, _: &mut AppContext) -> GameResult<Transition> {
        while ggez::timer::check_update_time(ctx, 60) {
            self.frame = self.frame.wrapping_add(1);
        }

        match self.done {
            Status::NotDone => Ok(std::mem::replace(&mut self.transition, Transition::None)),
            Status::DoneAndQuit => Ok(Transition::Pop),
        }
    }

    fn on_enter(&mut self, ctx: &mut Context, _: &mut AppContext) -> GameResult<()> {
        self.globals = load_global_graphics(ctx, &mut *self.assets.borrow_mut())?;
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
                imgui::Window::new(im_str!("Editor"))
                    .size([600.0, 700.0], Condition::Once)
                    .position([300.0, 20.0], Condition::Once)
                    .build(ui, || {
                        let path = self.path.clone();

                        let available_properties: Vec<_> = {
                            let pc = self.resource.borrow();
                            PropertyType::iter()
                                .filter(|item| {
                                    pc.instance
                                        .iter_key(path.clone())
                                        .find(|(_, value)| value.same_type_as(item))
                                        .is_none()
                                })
                                .collect()
                        };

                        if !available_properties.is_empty() {
                            if !available_properties
                                .iter()
                                .any(|item| item.same_type_as(&self.selected_property))
                            {
                                self.selected_property =
                                    available_properties.first().unwrap().clone();
                            }

                            ComboBox::new(im_str!("Type"))
                                .preview_value(&im_str!("{}", self.selected_property))
                                .build(ui, || {
                                    for item in available_properties.iter() {
                                        if Selectable::new(&im_str!("{}", item))
                                            .selected(item.same_type_as(&self.selected_property))
                                            .build(ui)
                                        {
                                            self.selected_property = item.clone();
                                        }
                                    }
                                });
                            let mut pc = self.resource.borrow_mut();

                            ui.same_line(0.0);
                            if ui.small_button(im_str!("Add")) {
                                pc.instance
                                    .insert_any(self.path.clone(), self.selected_property.clone());
                            }
                            ui.separator();
                        }
                        let mut to_remove = None;
                        TabBar::new(im_str!("Tabs")).build(ui, || {
                            let mut pc = self.resource.borrow_mut();
                            for (type_name, value) in pc.instance.iter_key_mut(self.path.clone()) {
                                let token = TabItem::new(&im_str!("{}", type_name)).begin(ui);
                                if let Some(token) = token {
                                    value.inspect_mut("data", &mut self.inspect_state, ui);

                                    ui.separator();

                                    if ui.small_button(im_str!("Delete")) {
                                        to_remove = Some(value.inner_type_id());
                                    }

                                    token.end(ui);
                                }
                            }
                        });

                        let mut pc = self.resource.borrow_mut();

                        if let Some(to_remove) = to_remove {
                            pc.instance.remove_any(self.path.clone(), to_remove);
                        }
                    });

                ui.main_menu_bar(|| {
                    ui.menu(im_str!("Instance Data Editor"), true, || {
                        if imgui::MenuItem::new(im_str!("Back")).build(ui) {
                            self.done = Status::DoneAndQuit;
                        }
                    });
                });
            })
            .render(ctx);

        let draw_cross = |ctx: &mut Context, origin: (f32, f32)| {
            let vertical = Mesh::new_line(
                ctx,
                &[[0.0, -10.0], [0.0, 10.0]],
                1.0,
                Color::new(0.0, 1.0, 0.0, 1.0),
            )?;

            let horizontal = Mesh::new_line(
                ctx,
                &[[-10.0, 0.0], [10.0, 0.0]],
                1.0,
                Color::new(0.0, 1.0, 0.0, 1.0),
            )?;
            graphics::draw(
                ctx,
                &vertical,
                DrawParam::default().dest([origin.0, origin.1]),
            )?;
            graphics::draw(
                ctx,
                &horizontal,
                DrawParam::default().dest([origin.0, origin.1]),
            )
        };

        let dim = (256.0, 256.0);
        let (width, height) = dim;
        // normal bullet
        let pos = (0.0, 20.0);
        let (x, y) = pos;
        let origin = (x + width / 2.0, y + height / 2.0);
        let offset = Matrix4::new_translation(&Vec3::new(origin.0, origin.1, 0.0));

        let resource = self.resource.borrow();

        graphics::set_transform(ctx, Matrix4::identity());
        graphics::apply_transformations(ctx)?;

        if let Some(animation) = resource
            .instance
            .get::<crate::roster::yuyuko::Graphic>(self.path.clone())
        {
            let key = animation.file_name();
            let resource = resource.graphics.get(&key).unwrap();
            if resource.duration() > 0 {
                let _lock = graphics::use_shader(ctx, &self.assets.borrow().shader);

                resource.draw_at_time(
                    ctx,
                    &self.assets.borrow(),
                    self.frame % resource.duration(),
                    offset,
                )?;
            }
        }
        if let Some(animation) = resource.instance.get::<GlobalGraphic>(self.path.clone()) {
            let resource = self.globals.get(animation).unwrap();
            if resource.duration() > 0 {
                let _lock = graphics::use_shader(ctx, &self.assets.borrow().shader);

                resource.draw_at_time(
                    ctx,
                    &self.assets.borrow(),
                    self.frame % resource.duration(),
                    offset,
                )?;
            }
        }

        if let Some(boxes) = resource
            .instance
            .get::<Timeline<Vec<Hitbox>>>(self.path.clone())
        {
            let (_, boxes) = boxes.get(self.frame % boxes.duration());
            for hitbox in boxes.iter() {
                hitbox.draw(ctx, offset, Color::new(1.0, 0.0, 0.0, 0.5))?;
            }
        }
        graphics::set_transform(ctx, Matrix4::identity());
        graphics::apply_transformations(ctx)?;
        draw_cross(ctx, origin)?;
        graphics::present(ctx)
    }
}

impl InstanceDataEditor {
    pub fn new(
        assets: Rc<RefCell<Assets>>,
        resource: Rc<RefCell<PlayerCharacter>>,

        path: String,
    ) -> Option<Self> {
        Some(Self {
            inspect_state: Default::default(),
            assets,
            path,
            frame: 0,
            resource,
            done: Status::NotDone,
            transition: Transition::None,
            selected_property: Default::default(),
            globals: HashMap::new(),
        })
    }
}
