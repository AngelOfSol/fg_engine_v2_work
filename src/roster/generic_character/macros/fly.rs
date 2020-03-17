macro_rules! impl_handle_fly {
    (fly_start: $fly_start:expr) => {
        fn handle_fly(move_id: MoveId, extra_data: &mut ExtraData) -> collision::Vec2 {
            if move_id == $fly_start {
                let fly_dir = extra_data.unwrap_fly_direction();
                *extra_data = ExtraData::FlyDirection(fly_dir);
                let speed = match fly_dir {
                    DirectedAxis::Forward => collision::Vec2::new(1_00, 0_00),
                    DirectedAxis::UpForward => collision::Vec2::new(0_71, 0_71),
                    DirectedAxis::DownForward => collision::Vec2::new(0_71, -0_71),
                    DirectedAxis::Backward => collision::Vec2::new(-1_00, 0_00),
                    DirectedAxis::UpBackward => collision::Vec2::new(-0_71, 0_71),
                    DirectedAxis::DownBackward => collision::Vec2::new(-0_71, -0_71),
                    DirectedAxis::Up => collision::Vec2::new(0_00, 1_00),
                    DirectedAxis::Down => collision::Vec2::new(0_00, -1_00),
                    _ => unreachable!(),
                };
                3 * speed / 4
            } else {
                collision::Vec2::zeros()
            }
        }
    };
}
