use crate::input::Input;
use std::collections::HashMap;

#[macro_export]
macro_rules! make_command_list {
    ($($($input:expr),* => $state:expr),*) => {{
        let mut list = CommandList::new();
        $(
            $(
                list.insert($input, $state);
            )*
        )*
        list
    }};
}

#[derive(Clone, Debug)]
pub struct CommandList<S> {
    commands: HashMap<Input, Vec<S>>,
}

impl<S> CommandList<S> {
    pub fn new() -> Self {
        Self {
            commands: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: Input, value: S) {
        let bucket = self.commands.entry(key).or_insert_with(Vec::new);
        bucket.push(value);
    }

    pub fn get_commands(&self, inputs: &[Input]) -> Vec<&S> {
        inputs
            .iter()
            .map(|input| self.commands.get(input).map(|item| item.iter()))
            .filter(|list| list.is_some())
            .map(|item| item.unwrap())
            .flatten()
            .collect()
    }
}
