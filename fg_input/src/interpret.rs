use types::InputBuffer;

use crate::{Facing, Input};

mod axis;
mod button_set;
mod double_tap;
mod dragon_punch;
mod helper;
mod quarter_circle;
mod super_jump;
mod types;

use helper::*;

pub fn interpret(
    facing: Facing,
    buffer_size: usize,
    grace_period: usize,
    motion_size: usize,
    buffer: InputBuffer<'_>,
) -> Vec<Input> {
    let buttons = (0..buffer_size.min(buffer.len()))
        .into_iter()
        .map(|start| {
            peek(button_set::interpret_buffered(grace_period))(&buffer[..buffer.len() - start])
        })
        .flatten()
        .find(|(_, buttons)| !buttons.is_empty());

    buttons
        .map(|(button_set_buffer, buttons)| {
            vec![
                // read dragon punch
                dragon_punch::interpret(motion_size)(button_set_buffer)
                    .map(|(_, direction)| Input::DragonPunch(direction, buttons)),
                // read quarter circle
                quarter_circle::interpret(motion_size)(button_set_buffer)
                    .map(|(_, direction)| Input::QuarterCircle(direction, buttons)),
                // read press button
                next_axis(button_set_buffer)
                    .map(|(_, axis)| Input::PressButton(buttons, axis.into())),
            ]
            .into_iter()
        })
        .into_iter()
        .flatten()
        .chain(vec![
            // read super jump
            super_jump::interpret(motion_size)(buffer)
                .map(|(_, axis)| Input::SuperJump(axis.into())),
            // read dash motions
            double_tap::interpret(motion_size)(buffer)
                .map(|(_, axis)| Input::DoubleTap(axis.into())),
            // read idle
            next_axis(buffer).map(|(_, axis)| Input::Idle(axis.into())),
        ])
        .flatten()
        .map(|item| match facing {
            Facing::Right => item,
            Facing::Left => item.invert(),
        })
        .collect()
}
