pub trait GenericMoveId: Clone + Copy {
    const STARTING_STATE: Self;
    const HITSTUN_AIR_START: Self;
    const HITSTUN_STAND_START: Self;
    const BLOCKSTUN_AIR_START: Self;
    const BLOCKSTUN_CROUCH_START: Self;
    const BLOCKSTUN_STAND_START: Self;
    const WRONGBLOCK_CROUCH_START: Self;
    const WRONGBLOCK_STAND_START: Self;

    const FLY_START: Self;
    const JUMP: Self;
    const BORDER_ESCAPE_JUMP: Self;
    const SUPER_JUMP: Self;
    const CROUCH_IDLE: Self;
    const STAND_IDLE: Self;
    const AIR_IDLE: Self;
    const FLY_CONTINUOUS: Self;
    const FLY_END: Self;
    const MELEE_RESTITUTION: Self;
    const KNOCKDOWN_START: Self;
}
