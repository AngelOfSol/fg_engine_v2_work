use crate::axis::Axis;

use super::{
    helper::take_while_m_n,
    types::{IResult, InputBuffer, ReadInput},
};

pub struct AxisRequired {
    motion_size: usize,
    axis: Axis,
}

impl<'a> ReadInput<'a, Axis> for AxisRequired {
    fn read_input(&mut self, buffer: InputBuffer<'a>) -> IResult<'a, Axis> {
        let axis = self.axis;
        let (buffer, _) = take_while_m_n(1, self.motion_size, move |input| axis == input.axis)
            .read_input(buffer)?;
        Some((buffer, self.axis))
    }
}

#[derive(Clone, Copy)]
pub struct AxisOptional {
    motion_size: usize,
    axis: Axis,
}

impl<'a> ReadInput<'a, Axis> for AxisOptional {
    fn read_input(&mut self, buffer: InputBuffer<'a>) -> IResult<'a, Axis> {
        let axis = self.axis;
        let (buffer, _) = take_while_m_n(0, self.motion_size, move |input| axis == input.axis)
            .read_input(buffer)?;
        Some((buffer, self.axis))
    }
}
pub fn axes(
    motion_size: usize,
) -> (
    impl Fn(Axis) -> AxisRequired + Copy,
    impl Fn(Axis) -> AxisOptional + Copy,
) {
    (
        move |axis: Axis| AxisRequired { motion_size, axis },
        move |axis: Axis| AxisOptional { motion_size, axis },
    )
}
