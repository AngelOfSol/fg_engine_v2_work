use serde::{Deserialize, Serialize};

use std::collections::HashSet;
use std::fmt;
use std::fmt::Display;

use crate::imgui_extra::UiExtensions;
use imgui::*;

use crate::typedefs::HashId;

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize, Hash)]
pub enum MoveType {
    Idle,
    Walk,
    Jump,
    HiJump,
    Dash,
    Melee,
    Magic,
    MeleeSpecial,
    MagicSpecial,
    Super,
    Followup,
    Fly,
    AirDash,
    AirMelee,
    AirMagic,
    AirMeleeSpecial,
    AirMagicSpecial,
    AirSuper,
    AirFollowup,
}
const ALL_MOVE_TYPES: [MoveType; 19] = [
    MoveType::Idle,
    MoveType::Walk,
    MoveType::Jump,
    MoveType::HiJump,
    MoveType::Dash,
    MoveType::Melee,
    MoveType::Magic,
    MoveType::MeleeSpecial,
    MoveType::MagicSpecial,
    MoveType::Super,
    MoveType::Followup,
    MoveType::Fly,
    MoveType::AirDash,
    MoveType::AirMelee,
    MoveType::AirMagic,
    MoveType::AirMeleeSpecial,
    MoveType::AirMagicSpecial,
    MoveType::AirSuper,
    MoveType::AirFollowup,
];
impl MoveType {
    pub fn all() -> &'static [MoveType; 19] {
        &ALL_MOVE_TYPES
    }
}
impl Display for MoveType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                MoveType::Idle => "Idle",
                MoveType::Walk => "Walk",
                MoveType::Jump => "Jump",
                MoveType::HiJump => "High Jump",
                MoveType::Dash => "Dash",
                MoveType::Fly => "Fly",
                MoveType::Melee => "Melee",
                MoveType::Magic => "Magic",
                MoveType::MeleeSpecial => "Melee Special",
                MoveType::MagicSpecial => "Magic Special",
                MoveType::Super => "Super",
                MoveType::Followup => "Follow Up",
                MoveType::AirDash => "Air Dash",
                MoveType::AirMelee => "Air Melee",
                MoveType::AirMagic => "Air Magic",
                MoveType::AirMeleeSpecial => "Air Melee Special",
                MoveType::AirMagicSpecial => "Air Magic Special",
                MoveType::AirSuper => "Air Super",
                MoveType::AirFollowup => "Air Followup",
            }
        )
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Serialize)]
pub struct CancelSet<Id>
where
    Id: HashId,
{
    pub always: HashSet<MoveType>,
    pub hit: HashSet<MoveType>,
    pub block: HashSet<MoveType>,
    #[serde(default)]
    pub disallow: HashSet<Id>,
}

impl<Id: HashId> CancelSet<Id> {
    pub fn new() -> Self {
        Self {
            always: HashSet::new(),
            hit: HashSet::new(),
            block: HashSet::new(),
            disallow: HashSet::new(),
        }
    }
}

pub struct CancelSetUi {
    state_list: Vec<String>,
    state_list_ui_data: Vec<ImString>,
    new_disallow: String,
}

const GREEN: [f32; 4] = [0.2, 1.0, 0.2, 1.0];
const BLUE: [f32; 4] = [0.7, 0.7, 1.0, 1.0];
const RED: [f32; 4] = [1.0, 0.2, 0.2, 1.0];

impl CancelSetUi {
    pub fn new(state_list: Vec<String>, state_list_ui_data: Vec<ImString>) -> CancelSetUi {
        CancelSetUi {
            new_disallow: state_list.get(0).cloned().unwrap_or("".to_owned()),
            state_list_ui_data,
            state_list,
        }
    }
    pub fn draw_ui(&mut self, ui: &Ui<'_>, data: &mut CancelSet<String>) {
        for move_type in MoveType::all() {
            ui.text(&im_str!("{}:", move_type));
            ui.push_id(&format!("{}", move_type));
            let _token = ui.push_style_color(StyleColor::Text, GREEN);
            ui.checkbox_hash(im_str!("Always"), move_type, &mut data.always);
            let _token = ui.push_style_color(StyleColor::Text, BLUE);
            ui.same_line(0.0);
            ui.checkbox_hash(im_str!("Block"), move_type, &mut data.block);
            let _token = ui.push_style_color(StyleColor::Text, RED);
            ui.same_line(0.0);
            ui.checkbox_hash(im_str!("Hit"), move_type, &mut data.hit);
            ui.pop_id();
        }
        ui.separator();

        ui.text(im_str!("Disallowed"));
        let mut to_delete = None;
        for item in data.disallow.iter() {
            {
                let _token = ui.push_style_color(StyleColor::Text, RED);
                ui.text(im_str!("{}", item));
            }
            ui.same_line(0.0);
            if ui.small_button(im_str!("Delete")) {
                to_delete = Some(item.clone());
            }
        }
        if let Some(item) = to_delete {
            data.disallow.remove(&item);
        }

        ui.combo_items(
            im_str!("##Combo"),
            &self.state_list,
            &self.state_list_ui_data,
            &mut self.new_disallow,
            5,
        );

        //ui.input_string(im_str!("##Disallowed"), &mut self.new_disallow);

        if ui.small_button(im_str!("Add")) && self.new_disallow != "" {
            let new = std::mem::replace(&mut self.new_disallow, "".to_owned());
            data.disallow.insert(new);
        }
    }
    pub fn draw_display_ui(ui: &Ui<'_>, data: &CancelSet<String>) {
        ui.text(im_str!("Always"));
        for move_type in data.always.iter() {
            let _token = ui.push_style_color(StyleColor::Text, GREEN);
            ui.text(im_str!("{}", move_type));
        }
        ui.separator();
        ui.text(im_str!("On Block"));
        for move_type in data.block.iter() {
            let _token = ui.push_style_color(StyleColor::Text, BLUE);
            ui.text(im_str!("{}", move_type));
        }
        ui.separator();
        ui.text(im_str!("On Hit"));
        for move_type in data.hit.iter() {
            let _token = ui.push_style_color(StyleColor::Text, RED);
            ui.text(im_str!("{}", move_type));
        }
    }
}
