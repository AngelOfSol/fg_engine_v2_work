use std::marker::PhantomData;

use crate::InputState;

pub type InputBuffer<'a> = &'a [InputState];
pub type IResult<'a, T = InputBuffer<'a>> = Option<(InputBuffer<'a>, T)>;

pub trait ReadInput<'a, O>: Sized {
    fn read_input(&mut self, input: InputBuffer<'a>) -> IResult<'a, O>;

    fn or<R>(self, next: R) -> Or<Self, R>
    where
        R: ReadInput<'a, O>,
    {
        Or {
            first: self,
            second: next,
        }
    }
    fn flat_map<U, R, O1, O2>(self, next: U) -> FlatMap<Self, U, O1>
    where
        U: FnMut(O) -> R,
        R: ReadInput<'a, O2>,
    {
        FlatMap {
            first: self,
            second: next,
            _phantom: PhantomData,
        }
    }
}

impl<'a, O, F> ReadInput<'a, O> for F
where
    F: FnMut(InputBuffer<'a>) -> IResult<'a, O> + 'a,
{
    fn read_input(&mut self, i: InputBuffer<'a>) -> IResult<'a, O> {
        self(i)
    }
}

impl<'a, O1, O2, T, U> ReadInput<'a, (O1, O2)> for (T, U)
where
    T: ReadInput<'a, O1>,
    U: ReadInput<'a, O2>,
{
    fn read_input(&mut self, input: InputBuffer<'a>) -> IResult<'a, (O1, O2)> {
        let (input, first) = self.0.read_input(input)?;
        let (input, second) = self.1.read_input(input)?;

        Some((input, (first, second)))
    }
}
impl<'a, O1, O2, O3, T, U, V> ReadInput<'a, (O1, O2, O3)> for (T, U, V)
where
    T: ReadInput<'a, O1>,
    U: ReadInput<'a, O2>,
    V: ReadInput<'a, O3>,
{
    fn read_input(&mut self, input: InputBuffer<'a>) -> IResult<'a, (O1, O2, O3)> {
        let (input, first) = self.0.read_input(input)?;
        let (input, second) = self.1.read_input(input)?;
        let (input, third) = self.2.read_input(input)?;

        Some((input, (first, second, third)))
    }
}
impl<'a, O1, O2, O3, O4, T, U, V, W> ReadInput<'a, (O1, O2, O3, O4)> for (T, U, V, W)
where
    T: ReadInput<'a, O1>,
    U: ReadInput<'a, O2>,
    V: ReadInput<'a, O3>,
    W: ReadInput<'a, O4>,
{
    fn read_input(&mut self, input: InputBuffer<'a>) -> IResult<'a, (O1, O2, O3, O4)> {
        let (input, first) = self.0.read_input(input)?;
        let (input, second) = self.1.read_input(input)?;
        let (input, third) = self.2.read_input(input)?;
        let (input, fourth) = self.3.read_input(input)?;

        Some((input, (first, second, third, fourth)))
    }
}

pub struct Or<T, U> {
    first: T,
    second: U,
}

impl<'a, O, T, U> ReadInput<'a, O> for Or<T, U>
where
    T: ReadInput<'a, O>,
    U: ReadInput<'a, O>,
{
    fn read_input(&mut self, input: InputBuffer<'a>) -> IResult<'a, O> {
        self.first
            .read_input(input)
            .or_else(|| self.second.read_input(input))
    }
}

pub struct FlatMap<T, U, O1> {
    first: T,
    second: U,
    _phantom: PhantomData<O1>,
}

impl<'a, O1, O2, T, U, H> ReadInput<'a, O2> for FlatMap<T, U, O1>
where
    T: ReadInput<'a, O1>,
    U: FnMut(O1) -> H,
    H: ReadInput<'a, O2>,
{
    fn read_input(&mut self, input: InputBuffer<'a>) -> IResult<'a, O2> {
        let (buffer, output) = self.first.read_input(input)?;
        (self.second)(output).read_input(buffer)
    }
}
