use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SmpEntry<Id> {
    command: Id,
    time: usize,
}

#[derive(Debug, Clone)]
pub struct SmpList<Id> {
    pub smp_list: HashMap<Id, usize>,
    pub first_command: Option<SmpEntry<Id>>,
}

impl<Id> Default for SmpList<Id> {
    fn default() -> Self {
        Self {
            smp_list: HashMap::new(),
            first_command: None,
        }
    }
}
