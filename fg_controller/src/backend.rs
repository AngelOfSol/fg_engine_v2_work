use fg_input::axis::Axis;

pub trait ControllerBackend {
    type ControllerId;
    fn poll(&mut self) -> Vec<()>;

    fn get(&self, id: Self::ControllerId) -> &ControllerState;
}

pub struct ControllerState {
    pub dpad: Axis,
    pub l_stick: Axis,
    pub r_stick: Axis,

    pub buttons: [bool; 12],
    // a b x y start select share ps4 l1 r1 l2 r2 lstick rstick
}
