use std::cell::RefCell;

use super::typed_character_editor::EDITOR_BACKGROUND;
use crate::character::components::AttackInfo;
use crate::{
    app_state::{AppContext, AppState, Transition},
    roster::character::{data::Data, typedefs::Character},
};
use ggez::{graphics, Context, GameResult};
use imgui::*;
use inspect_design::traits::{Inspect, InspectMut};
use std::rc::Rc;

enum Status {
    DoneAndSave,
    DoneAndQuit,
    NotDone,
}

pub struct TypedAttackEditor<C: Character> {
    path: C::Attack,
    frame: usize,
    resource: AttackInfo,
    character_data: Rc<RefCell<Data<C>>>,
    ui_data: <AttackInfo as Inspect>::State,
    done: Status,
    transition: Transition,
}
impl<C: Character> AppState for TypedAttackEditor<C> {
    fn update(&mut self, ctx: &mut Context, _: &mut AppContext) -> GameResult<Transition> {
        while ggez::timer::check_update_time(ctx, 60) {
            self.frame = self.frame.wrapping_add(1);
        }

        match self.done {
            Status::NotDone => Ok(std::mem::replace(&mut self.transition, Transition::None)),
            Status::DoneAndSave => {
                let mut cd = self.character_data.borrow_mut();
                cd.attacks.insert(self.path, self.resource.clone());
                Ok(Transition::Pop)
            }
            Status::DoneAndQuit => Ok(Transition::Pop),
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
        graphics::clear(ctx, EDITOR_BACKGROUND.into());
        imgui
            .frame()
            .run(|ui| {
                imgui::Window::new(im_str!("Editor"))
                    .size([1920.0, 1060.0], Condition::Always)
                    .position([0.0, 20.0], Condition::Always)
                    .draw_background(true)
                    .movable(false)
                    .resizable(false)
                    .collapsible(false)
                    .title_bar(false)
                    .build(ui, || {
                        self.resource.inspect_mut("attack", &mut self.ui_data, ui);
                    });

                ui.main_menu_bar(|| {
                    ui.menu(im_str!("Attack Info Editor"), true, || {
                        if imgui::MenuItem::new(im_str!("Reset")).build(ui) {
                            self.resource = Default::default();
                            self.ui_data = Default::default();
                        }
                        ui.separator();
                        if imgui::MenuItem::new(im_str!("Save and back")).build(ui) {
                            self.done = Status::DoneAndSave;
                        }
                        if imgui::MenuItem::new(im_str!("Back without save")).build(ui) {
                            self.done = Status::DoneAndQuit;
                        }
                    });
                });
            })
            .render(ctx);

        graphics::present(ctx)
    }
}

impl<C: Character> TypedAttackEditor<C> {
    pub fn new(path: C::Attack, character_data: Rc<RefCell<Data<C>>>) -> Self {
        let resource = character_data.borrow().attacks[&path].clone();
        Self {
            path,
            frame: 0,
            resource,
            character_data,
            ui_data: Default::default(),
            done: Status::NotDone,
            transition: Transition::None,
        }
    }
}
