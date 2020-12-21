use crate::hitbox::Hitbox;
use crate::typedefs::collision::Vec2;
use inspect_design::Inspect;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, PartialEq, Serialize, Inspect, Default)]
pub struct AttackData<AttackId> {
    pub id: usize,
    pub boxes: Vec<Hitbox>,
    pub data_id: AttackId,
}
impl<AttackId> AttackData<AttackId> {
    pub fn new(id: AttackId) -> Self {
        Self {
            id: 0,
            boxes: vec![],
            data_id: id,
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Serialize, Inspect, Default)]
pub struct HitboxSet<AttackId> {
    pub collision: Hitbox,
    pub hurtbox: Vec<Hitbox>,
    pub hitbox: Option<AttackData<AttackId>>,
}

impl<AttackId> HitboxSet<AttackId> {
    pub fn new() -> Self {
        Self {
            collision: Hitbox::with_half_size(Vec2::new(1_000, 5_000)),
            hurtbox: vec![],
            hitbox: None,
        }
    }
}

mod inspect {
    use super::{AttackData, HitboxSet};
    use crate::game_object::constructors::InspectOld;
    use imgui::*;

    impl<AttackId> InspectOld for HitboxSet<AttackId>
    where
        AttackData<AttackId>: InspectOld,
    {
        fn inspect_mut_old(&mut self, ui: &Ui<'_>) {
            TabBar::new(im_str!("Hitbox Set")).build(ui, || {});

            ui.text("Collision");
            self.collision.inspect_mut_old(ui);
            if CollapsingHeader::new(im_str!("Hitboxes")).build(ui) {
                self.hurtbox.inspect_mut_old(ui);
            }
        }
    }
}
