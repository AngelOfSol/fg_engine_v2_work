use imgui::Ui;
pub trait Inspect {
    fn inspect_mut(&mut self, ui: Ui<'_>);
}
