use crate::character::components::AttackInfo;
use crate::input::Facing;

#[derive(Debug, Clone)]
pub enum HitType<MoveId> {
    Whiff,
    Block(HitInfo<MoveId>),
    WrongBlock(HitInfo<MoveId>),
    Hit(HitInfo<MoveId>),
    CounterHit(HitInfo<MoveId>),
    Graze(HitInfo<MoveId>),
}

#[derive(Debug, Clone)]
pub enum HitInfo<MoveId> {
    Character {
        info: AttackInfo,
        move_id: MoveId,
        hitbox_id: usize,
        facing: Facing,
    },
    Bullet(AttackInfo, Facing),
}
impl<MoveId: Copy> HitInfo<MoveId> {
    pub fn get_attack_data(&self) -> &AttackInfo {
        match self {
            HitInfo::Character { ref info, .. } => info,
            HitInfo::Bullet(ref info, _) => info,
        }
    }

    pub fn get_facing(&self) -> Facing {
        match self {
            HitInfo::Character { facing, .. } => *facing,
            HitInfo::Bullet(_, facing) => *facing,
        }
    }

    pub fn should_pushback(&self) -> bool {
        match self {
            HitInfo::Character { .. } => true,
            HitInfo::Bullet(_, _) => false,
        }
    }

    pub fn get_hit_by_data(&self) -> Option<(MoveId, usize)> {
        if let HitInfo::Character {
            move_id, hitbox_id, ..
        } = self
        {
            Some((*move_id, *hitbox_id))
        } else {
            None
        }
    }
}
