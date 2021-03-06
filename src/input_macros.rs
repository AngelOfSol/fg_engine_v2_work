#[macro_export]
macro_rules! numpad {
    (236$button:ident) => {
        Input::QuarterCircle(Direction::Forward, Button::$button.into())
    };
    (214$button:ident) => {
        Input::QuarterCircle(Direction::Backward, Button::$button.into())
    };
    (623$button:ident) => {
        Input::DragonPunch(Direction::Forward, Button::$button.into())
    };
    (421$button:ident) => {
        Input::DragonPunch(Direction::Backward, Button::$button.into())
    };

    (release $button:ident) => {
        Input::ReleaseButton(Button::$button.into())
    };
    (release $button:ident $button2:ident) => {
        Input::ReleaseButton(Button::$button | Button::$button2)
    };

    (press $button:ident) => {
        Input::PressButton(DirectedAxis::Neutral, Button::$button.into())
    };
    (press $button:ident $button2:ident) => {
        Input::PressButton(DirectedAxis::Neutral, Button::$button | Button::$button2)
    };

    ($dir:tt $button:ident) => {
        Input::PressButton(read_axis!($dir), Button::$button.into())
    };
    ($dir:tt $button:ident $button2:ident) => {
        Input::PressButton(read_axis!($dir), Button::$button | Button::$button2)
    };

    (66) => {
        Input::DoubleTap(read_axis!(6))
    };
    (44) => {
        Input::DoubleTap(read_axis!(4))
    };
    (29) => {
        Input::SuperJump(read_axis!(9))
    };
    (28) => {
        Input::SuperJump(read_axis!(8))
    };
    (27) => {
        Input::SuperJump(read_axis!(7))
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
