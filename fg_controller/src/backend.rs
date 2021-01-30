pub trait ControllerBackend {
    type ControllerId;
    fn poll(&mut self) -> Vec<()>;

    fn get(&self, id: Self::ControllerId) -> &ControllerState;
}

pub struct ControllerState {
    pub dpad: [i32; 2],
    pub left_stick: [i32; 2],
    pub right_stick: [i32; 2],

    pub buttons: [bool; 12],
    // a b x y start select share ps4 l1 r1 l2 r2 lstick rstick
}
