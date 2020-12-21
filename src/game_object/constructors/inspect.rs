use enum_dispatch::*;
use imgui::*;

#[enum_dispatch]
pub trait InspectOld {
    fn inspect_mut_old(&mut self, _ui: &Ui<'_>) {}
}

impl<T: InspectOld + Default> InspectOld for Vec<T> {
    fn inspect_mut_old(&mut self, ui: &Ui<'_>) {
        ui.text(im_str!("Len: {}", self.len()));
        let mut to_delete = None;
        for (idx, item) in self.iter_mut().enumerate() {
            let id = ui.push_id(idx as i32);
            ui.text(im_str!("[{}]", idx));
            ui.same_line(0.0);
            if ui.small_button(im_str!("Delete")) {
                to_delete = Some(idx);
            }
            item.inspect_mut_old(ui);
            ui.separator();
            id.pop(ui);
        }
        if let Some(idx) = to_delete {
            self.remove(idx);
        }
        if ui.small_button(im_str!("Push Default")) {
            self.push(T::default());
        }
    }
}
