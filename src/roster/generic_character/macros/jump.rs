macro_rules! impl_handle_jump {
    (jump: $jump:pat, super_jump: $super_jump:pat, border_escape: $border_escape:pat) => {
        fn handle_jump(
            flags: &Flags,
            data: &Properties,
            move_id: MoveId,
            extra_data: &mut ExtraData,
        ) -> collision::Vec2 {
            if flags.jump_start {
                let axis = extra_data.unwrap_jump_direction();
                *extra_data = ExtraData::None;
                match move_id {
                    $jump => {
                        if !axis.is_horizontal() {
                            data.neutral_jump_accel
                        } else {
                            data.directed_jump_accel
                                .component_mul(&collision::Vec2::new(
                                    axis.direction_multiplier(true),
                                    1,
                                ))
                        }
                    }
                    $super_jump | $border_escape => {
                        if !axis.is_horizontal() {
                            data.neutral_super_jump_accel
                        } else {
                            data.directed_super_jump_accel
                                .component_mul(&collision::Vec2::new(
                                    axis.direction_multiplier(true),
                                    1,
                                ))
                        }
                    }
                    _ => panic!("jump_start not allowed on non jump moves"),
                }
            } else {
                collision::Vec2::zeros()
            }
        }
    };
}
