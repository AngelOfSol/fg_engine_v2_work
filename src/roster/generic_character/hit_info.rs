use crate::character::components::AttackInfo;
use crate::input::Facing;

#[derive(Debug, Clone)]
pub enum HitType {
    Whiff,
    Block(HitInfo),
    WrongBlock(HitInfo),
    Hit(HitInfo),
    CounterHit(HitInfo),
    Graze(HitInfo),
}

#[derive(Debug, Clone)]
pub enum HitInfo {
    Character {
        info: AttackInfo,
        hit_hash: u64,
        facing: Facing,
    },
    Bullet(AttackInfo, Facing),
}
impl HitInfo {
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

    pub fn get_hit_by_data(&self) -> Option<u64> {
        if let HitInfo::Character { hit_hash, .. } = self {
            Some(*hit_hash)
        } else {
            None
        }
    }
}
