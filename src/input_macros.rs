#[macro_export]
macro_rules! numpad {
    (236$button:ident) => {
        Input::QuarterCircle(Direction::Forward, Button::$button)
    };
    (214$button:ident) => {
        Input::QuarterCircle(Direction::Backward, Button::$button)
    };
    (623$button:ident) => {
        Input::DragonPunch(Direction::Forward, Button::$button)
    };
    (421$button:ident) => {
        Input::DragonPunch(Direction::Backward, Button::$button)
    };

    ($dir:tt $button:ident) => {
        Input::Button(read_axis!($dir), ButtonSet::Single(Button::$button))
    };

    ($dir:tt [$button:ident] ) => {
        Input::Holding(read_axis!($dir), ButtonSet::Single(Button::$button))
    };

    (66) => {
        Input::DoubleTap(read_axis!(6))
    };
    (44) => {
        Input::DoubleTap(read_axis!(4))
    };
    ($dir:tt) => {
        Input::Idle(read_axis!($dir))
    };
}

#[macro_export]
macro_rules! read_axis {
    (1) => {
        DirectedAxis::DownBackward
    };
    (2) => {
        DirectedAxis::Down
    };
    (3) => {
        DirectedAxis::DownForward
    };
    (4) => {
        DirectedAxis::Backward
    };
    (5) => {
        DirectedAxis::Neutral
    };
    (6) => {
        DirectedAxis::Forward
    };
    (7) => {
        DirectedAxis::UpBackward
    };
    (8) => {
        DirectedAxis::Up
    };
    (9) => {
        DirectedAxis::UpForward
    };
}
