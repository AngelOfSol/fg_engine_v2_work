#[derive(Clone, Debug)]
pub struct InputCoalesce<I, V> {
    iter: I,
    value: Option<V>,
}

impl<I, V> InputCoalesce<I, V>
where
    I: Iterator<Item = V>,
    V: PartialEq,
{
    pub fn new(iter: I) -> Self {
        Self { iter, value: None }
    }
}

use std::iter::Iterator;

impl<I, V> Iterator for InputCoalesce<I, V>
where
    I: Iterator<Item = V>,
    V: PartialEq,
{
    type Item = (I::Item, usize);

    fn next(&mut self) -> Option<Self::Item> {
        let (mut count, value) = match self.value.take() {
            Some(value) => (1, value),
            None => (1, self.iter.next()?),
        };

        let new_value = loop {
            if let Some(new_value) = self.iter.next() {
                if new_value == value {
                    count += 1;
                } else {
                    break Some(new_value);
                }
            } else {
                break None;
            };
        };
        self.value = new_value;

        Some((value, count))
    }
}
