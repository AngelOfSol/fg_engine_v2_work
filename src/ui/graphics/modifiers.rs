use crate::graphics::keyframe::{Coordinates, EaseType, Keyframe, Keyframes, Modifiers};
use crate::imgui_extra::UiExtensions;
use imgui::*;
use strum::IntoEnumIterator;

pub struct ModifiersUi;

impl ModifiersUi {
    pub fn draw_ui(ui: &Ui<'_>, mods: &mut Modifiers) {
        ui.combo_items(
            im_str!("System"),
            &mut mods.coord_type,
            &Coordinates::iter().collect::<Vec<_>>(),
            &|item| im_str!("{}", item).into(),
        );
        if ui
            .collapsing_header(im_str!("Rotation"))
            .default_open(false)
            .build()
        {
            let id = ui.push_id("Rotation");
            draw_ui_keyframes(ui, &mut mods.rotation);
            id.pop(ui);
        }
        if ui
            .collapsing_header(im_str!("Scale"))
            .default_open(false)
            .build()
        {
            let id = ui.push_id("Scale");
            if ui
                .collapsing_header(im_str!("X"))
                .default_open(false)
                .build()
            {
                let id = ui.push_id("X");
                draw_ui_keyframes(ui, &mut mods.scale[0]);
                id.pop(ui);
            }
            if ui
                .collapsing_header(im_str!("Y"))
                .default_open(false)
                .build()
            {
                let id = ui.push_id("Y");
                draw_ui_keyframes(ui, &mut mods.scale[1]);
                id.pop(ui);
            }

            id.pop(ui);
        }
        if ui
            .collapsing_header(im_str!("Offset"))
            .default_open(false)
            .build()
        {
            let id = ui.push_id("Offset");
            if ui
                .collapsing_header(&im_str!(
                    "{}",
                    match mods.coord_type {
                        Coordinates::Cartesian => "X",
                        Coordinates::Polar => "Radius",
                    }
                ))
                .default_open(false)
                .build()
            {
                let id = ui.push_id("X");
                draw_ui_keyframes(ui, &mut mods.coords[0]);
                id.pop(ui);
            }
            if ui
                .collapsing_header(&im_str!(
                    "{}",
                    match mods.coord_type {
                        Coordinates::Cartesian => "Y",
                        Coordinates::Polar => "Theta",
                    }
                ))
                .default_open(false)
                .build()
            {
                let id = ui.push_id("Y");
                draw_ui_keyframes(ui, &mut mods.coords[1]);
                id.pop(ui);
            }

            id.pop(ui);
        }
    }
}
fn draw_ui_keyframes(ui: &Ui<'_>, frames: &mut Keyframes) {
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
        frames.frames.insert(0, Keyframe::default());
    }
}

fn draw_ui_keyframe(ui: &Ui<'_>, frame: &mut Keyframe) -> bool {
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
