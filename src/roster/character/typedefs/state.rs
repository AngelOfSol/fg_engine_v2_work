pub trait StateConsts {
    const GAME_START: Self;
    const ROUND_START: Self;
    const DEAD: Self;
    const HIT_GROUND: Self;
    const AIR_IDLE: Self;
    const STAND: Self;
    const CROUCH: Self;
    const UNTECH: Self;
    const FLY: Self;
    const FLY_END: Self;
    const AIR_HITSTUN: Self;
    const GROUND_HITSTUN: Self;
    const GUARD_CRUSH: Self;
    const AIR_BLOCKSTUN: Self;
    const STAND_BLOCKSTUN: Self;
    const CROUCH_BLOCKSTUN: Self;
    const STAND_WRONG_BLOCKSTUN: Self;
    const CROUCH_WRONG_BLOCKSTUN: Self;
}
