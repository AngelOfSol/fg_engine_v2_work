use serde::{Deserialize, Serialize};

use std::collections::HashSet;
use std::fmt;
use std::fmt::Display;

use crate::imgui_extra::UiExtensions;
use imgui::*;


#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, Serialize, Hash)]
pub enum MoveType {
    Idle,
    Walk,
    Jump,
    HiJump,
    Dash,
    Fly,
    Melee,
    Magic,
    MeleeSpecial,
    MagicSpecial,
    Super,
}
const ALL_MOVE_TYPES: [MoveType; 11] = [
    MoveType::Idle,
    MoveType::Walk,
    MoveType::Jump,
    MoveType::HiJump,
    MoveType::Dash,
    MoveType::Fly,
    MoveType::Melee,
    MoveType::Magic,
    MoveType::MeleeSpecial,
    MoveType::MagicSpecial,
    MoveType::Super,
];
impl MoveType {
    pub fn all() -> &'static [MoveType; 11] {
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
            }
        )
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Serialize)]
pub struct CancelSet {
    always: HashSet<MoveType>,
    hit: HashSet<MoveType>,
    block: HashSet<MoveType>,
}

impl CancelSet {
    pub fn new() -> Self {
        Self {
            always: HashSet::new(),
            hit: HashSet::new(),
            block: HashSet::new(),
        }
    }
}

pub struct CancelSetUi;

impl CancelSetUi {
    pub fn new() -> CancelSetUi {
        CancelSetUi
    }
    pub fn draw_ui(&mut self, ui: &Ui<'_>, data: &mut CancelSet) {
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
    }
    pub fn draw_display_ui(ui: &Ui<'_>, data: &CancelSet) {
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
