use super::typed_character_editor::EDITOR_BACKGROUND;
use crate::character::state::SpawnerInfo;
use crate::{
    app_state::{AppContext, AppState, Transition},
    character::state::components::{CancelSet, HitboxSet},
    roster::character::data::Data,
};
use crate::{assets::Assets, roster::character::typedefs::Character};
use crate::{character::state::components::Flags, timeline::Timeline};
use crate::{character::state::components::StateType, imgui_extra::UiExtensions};
use crate::{character::state::State, game_object::constructors::Constructor};
use crate::{timeline, ui::character::state::CancelSetUi};
use fg_datastructures::math::graphics::{Matrix4, Vec3};
use fg_datastructures::math::{collision::IntoGraphical, graphics::Vec2};
use ggez::graphics;
use ggez::graphics::{Color, DrawParam, Mesh};
use ggez::{Context, GameResult};
use imgui::*;
use inspect_design::traits::{Inspect, InspectMut};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;
use strum::IntoEnumIterator;

pub struct TypedStateEditor<C: Character> {
    character_data: Rc<RefCell<Data<C>>>,
    assets: Rc<RefCell<Assets>>,
    id: C::State,
    new_state: State<C>,
    frame: usize,
    is_playing: bool,
    transition: Transition,
    draw_mode: DrawMode,
    graphic_inspect_state: <C::Graphic as Inspect>::State,
    spawner_info_inspect_state: <Vec<SpawnerInfo> as Inspect>::State,
    flags_state: <Timeline<Flags> as Inspect>::State,
    cancels_state: <Timeline<CancelSet> as Inspect>::State,
    hitbox_state: <Timeline<HitboxSet<C>> as Inspect>::State,
    current_cancel_set_ui: CancelSetUi,
}
struct DrawMode {
    collision_alpha: f32,
    hurtbox_alpha: f32,
    hitbox_alpha: f32,
    debug_animation: bool,
    show_axes: bool,
}

impl<C: Character> AppState for TypedStateEditor<C>
where
    Data<C>: Serialize + for<'de> Deserialize<'de>,
{
    fn update(&mut self, ctx: &mut Context, _: &mut AppContext) -> GameResult<Transition> {
        while ggez::timer::check_update_time(ctx, 60) {
            if self.is_playing {
                self.frame = self.frame.wrapping_add(1);

                self.frame %= self.new_state.duration();
            }
        }

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

        imgui
            .frame()
            .run(|ui| {
                imgui::Window::new(im_str!("Main"))
                    .size([1620.0, 1060.0], Condition::Always)
                    .position([300.0, 20.0], Condition::Always)
                    .draw_background(true)
                    .movable(false)
                    .resizable(false)
                    .collapsible(false)
                    .title_bar(false)
                    .build(ui, || {
                        imgui::TabBar::new(im_str!("Main")).build(ui, || {
                            imgui::TabItem::new(&im_str!("Properties")).build(ui, || {
                                {
                                    let mut cd = self.character_data.borrow_mut();
                                    let entry = cd.state_graphics_map.entry(self.id).or_default();

                                    entry.inspect_mut(
                                        "Graphic",
                                        &mut self.graphic_inspect_state,
                                        ui,
                                    );
                                }

                                let cd = self.character_data.borrow();
                                let entry = cd.state_graphics_map.get(&self.id).unwrap();

                                self.new_state.set_duration(cd.graphics[entry].duration());

                                ui.combo_items(
                                    im_str!("State Type"),
                                    &mut self.new_state.state_type,
                                    StateType::all(),
                                    &|item| im_str!("{}", item).into(),
                                );

                                ui.combo_items(
                                    im_str!("On Expire"),
                                    &mut self.new_state.on_expire.state_id,
                                    &C::State::iter().collect::<Vec<_>>(),
                                    &|item| im_str!("{}", item).into(),
                                );

                                let _ = ui.input_whole(
                                    im_str!("Frame"),
                                    &mut self.new_state.on_expire.frame,
                                );
                            });

                            imgui::TabItem::new(im_str!("Spawners##State")).build(ui, || {
                                self.new_state.spawns.inspect_mut(
                                    "spawners",
                                    &mut self.spawner_info_inspect_state,
                                    ui,
                                );
                            });

                            imgui::TabItem::new(im_str!("Flags")).build(ui, || {
                                self.new_state.flags.inspect_mut(
                                    "flags",
                                    &mut self.flags_state,
                                    ui,
                                );
                            });
                            imgui::TabItem::new(im_str!("Cancels")).build(ui, || {
                                let id = ui.push_id("Cancels");
                                let ui_state = &mut self.current_cancel_set_ui;

                                timeline::inspect::inspect_mut_custom(
                                    &mut self.new_state.cancels,
                                    "cancels",
                                    &mut self.cancels_state,
                                    ui,
                                    |_, data| {
                                        ui.separator();
                                        imgui::ChildWindow::new(im_str!("child frame"))
                                            .size([0.0, 0.0])
                                            .build(ui, || {
                                                ui_state.draw_ui(ui, data);
                                            });
                                    },
                                );

                                id.pop(ui);
                            });
                            imgui::TabItem::new(im_str!("Hitboxes")).build(ui, || {
                                self.new_state.hitboxes.inspect_mut(
                                    "hitboxes",
                                    &mut self.hitbox_state,
                                    ui,
                                );
                            });
                        });
                    });

                imgui::Window::new(im_str!("Playback"))
                    .size([300.0, 215.0], Condition::Once)
                    .movable(false)
                    .resizable(false)
                    .collapsible(false)
                    .position([0.0, 505.0], Condition::Once)
                    .build(ui, || {
                        if self.new_state.duration() > 0 {
                            if ui
                                .slider_whole(
                                    im_str!("Frame"),
                                    &mut self.frame,
                                    0,
                                    self.new_state.duration() - 1,
                                )
                                .unwrap_or(false)
                            {
                                self.is_playing = false;
                            }
                            if ui.small_button(im_str!("Play")) {
                                self.is_playing = true;
                            };
                            ui.same_line(0.0);
                            if ui.small_button(im_str!("Stop")) {
                                self.is_playing = false;
                            };
                        }
                        ui.text(im_str!("Draw"));
                        ui.separator();
                        ui.checkbox(im_str!("Debug"), &mut self.draw_mode.debug_animation);
                        ui.same_line(0.0);
                        ui.checkbox(im_str!("Axes"), &mut self.draw_mode.show_axes);
                        ui.text(im_str!("Alpha"));

                        ui.separator();

                        imgui::Slider::new(im_str!("Collision"))
                            .range(0.0..=1.0)
                            .build(ui, &mut self.draw_mode.collision_alpha);
                        imgui::Slider::new(im_str!("Hurtbox"))
                            .range(0.0..=1.0)
                            .build(ui, &mut self.draw_mode.hurtbox_alpha);
                        imgui::Slider::new(im_str!("Hitbox"))
                            .range(0.0..=1.0)
                            .build(ui, &mut self.draw_mode.hitbox_alpha);
                    });

                ui.main_menu_bar(|| {
                    ui.menu(im_str!("State Editor"), true, || {
                        if imgui::MenuItem::new(im_str!("Reset")).build(ui) {
                            self.new_state = self.character_data.borrow().states[&self.id].clone();
                        }
                        if imgui::MenuItem::new(im_str!("Save to file")).build(ui) {
                            if let Ok(nfd::Response::Okay(path)) =
                                nfd::open_save_dialog(Some("json"), None)
                            {
                                let mut path = PathBuf::from(path);
                                path.set_extension("json");
                                State::save(&self.new_state, path);
                            }
                        }
                        if imgui::MenuItem::new(im_str!("Load from file")).build(ui) {
                            if let Ok(nfd::Response::Okay(path)) =
                                nfd::open_file_dialog(Some("json"), None)
                            {
                                let state = State::load_from_json(PathBuf::from(path));
                                self.new_state = state;
                            }
                        }
                        ui.separator();

                        if imgui::MenuItem::new(im_str!("Save and back")).build(ui) {
                            self.character_data
                                .borrow_mut()
                                .states
                                .insert(self.id, self.new_state.clone());
                            self.transition = Transition::Pop;
                        }
                        if imgui::MenuItem::new(im_str!("Back without saving")).build(ui) {
                            self.transition = Transition::Pop;
                        }
                    });
                });
            })
            .render(ctx);
        let animation_window_center = Matrix4::new_translation(&Vec3::new(150.0, 270.0, 0.0));
        if self.draw_mode.show_axes {
            graphics::set_transform(ctx, animation_window_center);
            graphics::apply_transformations(ctx)?;
            let x_axis = Mesh::new_line(
                ctx,
                &[[-140.0, 0.0], [140.0, 0.0]],
                1.0,
                Color::new(0.0, 0.0, 0.0, 1.0),
            )?;
            let y_axis = Mesh::new_line(
                ctx,
                &[[0.0, 0.0], [0.0, -150.0]],
                1.0,
                Color::new(0.0, 0.0, 0.0, 1.0),
            )?;
            graphics::draw(ctx, &x_axis, DrawParam::default())?;
            graphics::draw(ctx, &y_axis, DrawParam::default())?;
        }

        let offset = {
            let mut offset = Vec3::zeros();

            let (_, boxes) = self.new_state.hitboxes.get(self.frame);
            let recenter = boxes.collision.collision_graphic_recenter();
            offset.x -= recenter.x;
            offset.y -= recenter.y;

            offset
        };

        let offset = animation_window_center * Matrix4::new_translation(&offset);
        let assets = &self.assets.borrow();

        {
            let _lock = graphics::use_shader(ctx, &assets.shader);

            if self.draw_mode.debug_animation {
                let pc = self.character_data.borrow();
                pc.graphics[&pc.state_graphics_map[&self.id]].draw_at_time_debug(
                    ctx,
                    assets,
                    self.frame,
                    offset,
                    Default::default(),
                )?;
            } else {
                let pc = self.character_data.borrow();
                pc.graphics[&pc.state_graphics_map[&self.id]]
                    .draw_at_time(ctx, assets, self.frame, offset)?;
            }
        }

        let offset = {
            let mut offset = Vec3::zeros();
            let (_, boxes) = self.new_state.hitboxes.get(self.frame);
            offset.x -= boxes.collision.center.x.into_graphical();
            offset.y -= boxes.collision.half_size.y.into_graphical()
                - boxes.collision.center.y.into_graphical();

            offset
        };
        let offset = animation_window_center * Matrix4::new_translation(&offset);
        let (_, boxes) = self.new_state.hitboxes.get(self.frame);
        boxes.collision.draw(
            ctx,
            offset,
            Color::new(1.0, 1.0, 1.0, self.draw_mode.collision_alpha),
        )?;

        let offset = {
            let mut offset = Vec3::zeros();
            let (_, boxes) = self.new_state.hitboxes.get(self.frame);
            offset.y -= boxes.collision.half_size.y.into_graphical();

            offset
        };
        let offset = animation_window_center * Matrix4::new_translation(&offset);
        let draw_cross = |ctx: &mut Context, origin: Vec2| {
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
                DrawParam::default().dest([origin.x, origin.y]),
            )?;
            graphics::draw(
                ctx,
                &horizontal,
                DrawParam::default().dest([origin.x, origin.y]),
            )
        };
        graphics::set_transform(ctx, offset);
        graphics::apply_transformations(ctx)?;
        for item in self
            .new_state
            .spawns
            .iter()
            .filter(|item| item.frame == self.frame)
            .flat_map(|item| item.data.iter())
            .filter_map(|item| {
                if let Constructor::Position(position) = item {
                    Some(position)
                } else {
                    None
                }
            })
        {
            let offset = item.value.into_graphical();
            draw_cross(ctx, offset)?;
        }

        let (_, boxes) = self.new_state.hitboxes.get(self.frame);
        for hurtbox in boxes.hurtbox.iter() {
            hurtbox.draw(
                ctx,
                offset,
                Color::new(0.0, 1.0, 0.0, self.draw_mode.hurtbox_alpha),
            )?;
        }
        if let Some(attack_data) = &boxes.hitbox {
            for hitbox in attack_data.boxes.iter() {
                hitbox.draw(
                    ctx,
                    offset,
                    Color::new(1.0, 0.0, 0.0, self.draw_mode.hitbox_alpha),
                )?;
            }
        }

        graphics::present(ctx)
    }
}

impl<C: Character> TypedStateEditor<C> {
    pub fn new(
        character_data: Rc<RefCell<Data<C>>>,
        id: C::State,
        assets: Rc<RefCell<Assets>>,
    ) -> Self {
        let new_state = character_data.borrow().states[&id].clone();
        Self {
            new_state,
            character_data,
            assets,
            id,
            transition: Transition::None,
            frame: 0,
            is_playing: true,
            draw_mode: DrawMode {
                collision_alpha: 0.15,
                hurtbox_alpha: 0.15,
                hitbox_alpha: 0.15,
                debug_animation: true,
                show_axes: true,
            },
            graphic_inspect_state: Default::default(),
            spawner_info_inspect_state: Default::default(),
            flags_state: Default::default(),
            current_cancel_set_ui: Default::default(),
            cancels_state: Default::default(),
            hitbox_state: Default::default(),
        }
    }
}
