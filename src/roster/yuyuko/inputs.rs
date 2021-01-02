use crate::input::Input;
use std::collections::HashMap;

use super::commands::CommandId;

pub type InputMap = HashMap<Input, Vec<CommandId>>;
