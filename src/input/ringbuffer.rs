use super::{Axis, InputState, BUFFER_LENGTH};
use std::ops::Index;

#[derive(Debug, Clone)]
pub struct RingBuffer {
    buffer: [InputState; BUFFER_LENGTH],
    head: usize,
}

impl RingBuffer {
    pub fn new() -> Self {
        Self {
            buffer: [Default::default(); 30],
            head: 0,
        }
    }
    pub fn push(&mut self, input: InputState) {
        self.head = self.head + BUFFER_LENGTH - 1;
        self.head %= BUFFER_LENGTH;
        self.buffer[self.head] = input;
    }

    pub fn top(&self) -> &InputState {
        &self[0]
    }

    pub fn iter(&self) -> RingBufferIter<'_> {
        RingBufferIter {
            buffer: &self,
            index: 0,
        }
    }
}

impl Index<usize> for RingBuffer {
    type Output = InputState;
    fn index(&self, idx: usize) -> &Self::Output {
        &self.buffer[(self.head + idx) % BUFFER_LENGTH]
    }
}

#[derive(Debug, Clone)]
pub struct DirectionIter<'ring> {
    buffer: &'ring RingBuffer,
    index: usize,
    current_axis: Axis,
}

impl<'ring> Iterator for DirectionIter<'ring> {
    type Item = (usize, Axis);
    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= BUFFER_LENGTH {
            None
        } else {
            let mut duration = 0;
            while self.buffer[self.index].axis == self.current_axis && self.index < BUFFER_LENGTH {
                duration += 1;
                self.index += 1;
            }
            let ret = Some((duration, self.current_axis));
            self.current_axis = self.buffer[self.index].axis;
            ret
        }
    }
}

#[derive(Debug, Clone)]
pub struct RingBufferIter<'ring> {
    buffer: &'ring RingBuffer,
    index: usize,
}

impl<'ring> RingBufferIter<'ring> {
    pub fn into_direction_iter(self) -> DirectionIter<'ring> {
        DirectionIter {
            buffer: self.buffer,
            index: self.index,
            current_axis: self.buffer[self.index].axis,
        }
    }
}

impl<'ring> Iterator for RingBufferIter<'ring> {
    type Item = InputState;
    fn next(&mut self) -> Option<Self::Item> {
        let index = self.index;
        self.index += 1;
        if index >= BUFFER_LENGTH {
            None
        } else {
            Some(self.buffer[index])
        }
    }
}
