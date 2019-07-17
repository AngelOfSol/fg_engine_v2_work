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
    new_disallow: String,
}

impl CancelSetUi {
    pub fn new() -> CancelSetUi {
        CancelSetUi {
            new_disallow: "".to_owned(),
        }
    }
    pub fn draw_ui(&mut self, ui: &Ui<'_>, data: &mut CancelSet<String>) {
        if ui.collapsing_header(im_str!("Always")).build() {
            ui.push_id("Always");
            ui.checkbox_set(MoveType::all(), &mut data.always);
            ui.pop_id();
        }
        if ui.collapsing_header(im_str!("On Block")).build() {
            ui.push_id("On Block");
            ui.checkbox_set(MoveType::all(), &mut data.block);
            ui.pop_id();
        }
        if ui.collapsing_header(im_str!("On Hit")).build() {
            ui.push_id("On Hit");
            ui.checkbox_set(MoveType::all(), &mut data.hit);
            ui.pop_id();
        }
        if ui.collapsing_header(im_str!("Disallowed")).build() {
            let mut to_delete = None;
            for item in data.disallow.iter() {
                ui.text(im_str!("{}", item));
                ui.same_line(0.0);
                if ui.small_button(im_str!("Delete")) {
                    to_delete = Some(item.clone());
                }
            }
            if let Some(item) = to_delete {
                data.disallow.remove(&item);
            }

            ui.input_string(im_str!("##Disallowed"), &mut self.new_disallow);
            ui.same_line(0.0);
            if ui.small_button(im_str!("Add")) {
                let new = std::mem::replace(&mut self.new_disallow, "".to_owned());
                data.disallow.insert(new);
            }
        }
    }
    pub fn draw_display_ui(ui: &Ui<'_>, data: &CancelSet<String>) {
        if ui.collapsing_header(im_str!("Always")).build() {
            for move_type in data.always.iter() {
                ui.text(im_str!("{}", move_type));
            }
        }
        if ui.collapsing_header(im_str!("On Block")).build() {
            for move_type in data.block.iter() {
                ui.text(im_str!("{}", move_type));
            }
        }
        if ui.collapsing_header(im_str!("On Hit")).build() {
            for move_type in data.hit.iter() {
                ui.text(im_str!("{}", move_type));
            }
        }
    }
}
