use ggez::graphics;
use std::collections::HashMap;

pub struct Assets {
    pub images: HashMap<String, graphics::Image>,
}

impl Assets {
    pub fn new() -> Self {
        Self {
            images: HashMap::new(),
        }
    }
}
