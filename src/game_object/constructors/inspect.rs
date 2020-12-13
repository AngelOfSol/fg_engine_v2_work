use enum_dispatch::*;
use imgui::Ui;

#[enum_dispatch]
pub trait Inspect {
    fn inspect_mut(&mut self, _ui: &Ui<'_>) {}
}
