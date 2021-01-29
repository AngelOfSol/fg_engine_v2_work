use crate::interpret::types::{IResult, InputBuffer, ReadInput};

pub fn alt<'a, T: Alt<'a, O>, O>(mut a: T) -> impl FnMut(InputBuffer<'a>) -> IResult<'a, O> {
    move |buffer: InputBuffer| a.read_input(buffer)
}

pub trait Alt<'a, O> {
    fn read_input(&mut self, buffer: InputBuffer<'a>) -> IResult<'a, O>;
}

impl<'a, O, T, U> Alt<'a, O> for (T, U)
where
    T: ReadInput<'a, O>,
    U: ReadInput<'a, O>,
{
    fn read_input(&mut self, input: InputBuffer<'a>) -> IResult<'a, O> {
        self.0
            .read_input(input)
            .or_else(|| self.1.read_input(input))
    }
}

impl<'a, O, T, U, V> Alt<'a, O> for (T, U, V)
where
    T: ReadInput<'a, O>,
    U: ReadInput<'a, O>,
    V: ReadInput<'a, O>,
{
    fn read_input(&mut self, input: InputBuffer<'a>) -> IResult<'a, O> {
        self.0
            .read_input(input)
            .or_else(|| self.1.read_input(input))
            .or_else(|| self.2.read_input(input))
    }
}

impl<'a, O, T, U, V, W> Alt<'a, O> for (T, U, V, W)
where
    T: ReadInput<'a, O>,
    U: ReadInput<'a, O>,
    V: ReadInput<'a, O>,
    W: ReadInput<'a, O>,
{
    fn read_input(&mut self, input: InputBuffer<'a>) -> IResult<'a, O> {
        self.0
            .read_input(input)
            .or_else(|| self.1.read_input(input))
            .or_else(|| self.2.read_input(input))
            .or_else(|| self.3.read_input(input))
    }
}
