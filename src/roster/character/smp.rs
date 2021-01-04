use std::{collections::HashMap, hash::Hash};

use super::typedefs::Timed;

#[derive(Debug, Clone)]
pub struct SmpList<Id> {
    pub smp_list: HashMap<Id, usize>,
    pub first_command: Option<Timed<Id>>,
}

impl<Id> Default for SmpList<Id> {
    fn default() -> Self {
        Self {
            smp_list: HashMap::new(),
            first_command: None,
        }
    }
}

impl<Id: Hash + Eq> SmpList<Id> {
    pub fn reset(&mut self) {
        self.first_command = None;
        self.smp_list.clear();
    }

    pub fn push(&mut self, associated_command: Timed<Id>) {
        if let Some(first_command) = &self.first_command {
            if &associated_command != first_command
                && associated_command.time > first_command.time
                && !self.smp_list.contains_key(&associated_command.id)
            {
                self.smp_list
                    .insert(associated_command.id, associated_command.time);
            }
        } else {
            self.first_command = Some(associated_command);
        }
    }

    pub fn should_smp(&self, most_recent_command: Timed<Id>) -> bool {
        self.smp_list
            .get(&most_recent_command.id)
            .map(|old_time| most_recent_command.time > *old_time)
            .unwrap_or(false)
    }
}
