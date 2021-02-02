#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Delay {
    current: usize,
    reset_to: usize,
}

impl Delay {
    pub fn new(reset_to: usize) -> Self {
        Self {
            current: 0,
            reset_to,
        }
    }
    pub fn delay(value: usize) -> Self {
        Self {
            current: value,
            reset_to: value,
        }
    }

    pub fn is_ready(&self) -> bool {
        self.current == 0
    }

    pub fn update(&mut self) -> bool {
        self.current = self.current.saturating_sub(1);
        self.is_ready()
    }

    pub fn ready(&mut self) {
        self.current = 0;
    }

    pub fn unready(&mut self) {
        self.current = self.reset_to;
    }
}
