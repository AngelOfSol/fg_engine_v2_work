use crate::graphics::keyframe::{
    Coordinates, EaseType, Keyframes, Modifiers, OldKeyframe, OldKeyframes,
};
use crate::imgui_extra::UiExtensions;
use imgui::*;
use inspect_design::traits::{Inspect, InspectMut};
use strum::IntoEnumIterator;

#[derive(Default)]
pub struct ModifiersUi {
    state: <Keyframes as Inspect>::State,
}

impl ModifiersUi {
    pub fn draw_ui(&mut self, ui: &Ui<'_>, mods: &mut Modifiers) {
        ui.combo_items(
            im_str!("System"),
            &mut mods.coord_type,
            &Coordinates::iter().collect::<Vec<_>>(),
            &|item| im_str!("{}", item).into(),
        );
        if imgui::CollapsingHeader::new(im_str!("Rotation"))
            .default_open(false)
            .build(ui)
        {
            mods.rotation.inspect_mut("rotation", &mut self.state, ui);
        }
        if imgui::CollapsingHeader::new(im_str!("Scale"))
            .default_open(false)
            .build(ui)
        {
            let id = ui.push_id("Scale");
            if imgui::CollapsingHeader::new(im_str!("X"))
                .default_open(false)
                .build(ui)
            {
                let id = ui.push_id("X");
                draw_ui_keyframes(ui, &mut mods.scale[0]);
                id.pop(ui);
            }
            if imgui::CollapsingHeader::new(im_str!("Y"))
                .default_open(false)
                .build(ui)
            {
                let id = ui.push_id("Y");
                draw_ui_keyframes(ui, &mut mods.scale[1]);
                id.pop(ui);
            }

            id.pop(ui);
        }
        if imgui::CollapsingHeader::new(im_str!("Offset"))
            .default_open(false)
            .build(ui)
        {
            let id = ui.push_id("Offset");
            if imgui::CollapsingHeader::new(&im_str!(
                "{}",
                match mods.coord_type {
                    Coordinates::Cartesian => "X",
                    Coordinates::Polar => "Radius",
                }
            ))
            .default_open(false)
            .build(ui)
            {
                let id = ui.push_id("X");
                draw_ui_keyframes(ui, &mut mods.coords[0]);
                id.pop(ui);
            }
            if imgui::CollapsingHeader::new(&im_str!(
                "{}",
                match mods.coord_type {
                    Coordinates::Cartesian => "Y",
                    Coordinates::Polar => "Theta",
                }
            ))
            .default_open(false)
            .build(ui)
            {
                let id = ui.push_id("Y");
                draw_ui_keyframes(ui, &mut mods.coords[1]);
                id.pop(ui);
            }

            id.pop(ui);
        }

        if imgui::CollapsingHeader::new(im_str!("Alpha##Modifier"))
            .default_open(false)
            .build(ui)
        {
            let id = ui.push_id("Alpha##Modifier");
            draw_ui_keyframes(ui, &mut mods.alpha);
            id.pop(ui);
        }

        if imgui::CollapsingHeader::new(im_str!("Value"))
            .default_open(false)
            .build(ui)
        {
            let id = ui.push_id("Value");
            draw_ui_keyframes(ui, &mut mods.value);
            id.pop(ui);
        }
    }
}
fn draw_ui_keyframes(ui: &Ui<'_>, frames: &mut OldKeyframes) {
    let mut resort = false;

    let element_count = frames.frames.len();
    let mut id = 0;
    frames.frames.drain_filter(|keyframe| {
        id += 1;
        let id = ui.push_id(id);

        resort = resort || draw_ui_keyframe(ui, keyframe);

        let delete = element_count > 1 && ui.small_button(im_str!("Delete"));

        ui.separator();

        id.pop(ui);

        delete
    });

    if resort {
        frames.frames.sort_by_key(|item| item.frame);
    }

    if ui.small_button(im_str!("New Keyframe")) {
        frames.frames.insert(0, OldKeyframe::default());
    }
}

fn draw_ui_keyframe(ui: &Ui<'_>, frame: &mut OldKeyframe) -> bool {
    ui.input_whole(im_str!("Frame"), &mut frame.frame).unwrap();
    let ret = ui.is_item_deactivated_after_edit();
    ui.input_float(im_str!("Value"), &mut frame.value).build();
    ui.combo_items(
        im_str!("Ease Type"),
        &mut frame.function,
        &EaseType::iter().collect::<Vec<_>>(),
        &|item| im_str!("{}", item).into(),
    );

    ret
}
