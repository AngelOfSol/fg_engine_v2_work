use super::InputRange;

#[derive(Debug)]
pub struct LocalHistory<T> {
    front_frame: usize,
    data: Vec<T>,
}

impl<T: Default + Clone> LocalHistory<T> {
    pub fn new() -> Self {
        Self {
            front_frame: 0,
            data: vec![],
        }
    }

    fn adjust_frame(&self, frame: usize) -> Option<usize> {
        frame.checked_sub(self.front_frame)
    }
    fn adjust_idx(&self, idx: usize) -> usize {
        idx + self.front_frame
    }

    pub fn add_input(&mut self, data: T) -> usize {
        self.data.push(data);
        self.front_frame + self.data.len() - 1
    }

    pub fn has_input(&self, frame: usize) -> bool {
        self.adjust_frame(frame)
            .and_then(|frame| self.data.get(frame))
            .is_some()
    }

    pub fn get_inputs(&self, frame: usize, amt: usize) -> (InputRange, &[T]) {
        let frame = self.adjust_frame(frame).unwrap();
        let end_idx = self.data.len().min(frame + 1);
        let start_idx = end_idx.saturating_sub(amt);

        (
            InputRange {
                first: self.adjust_idx(start_idx),
                last: self.adjust_idx(end_idx).saturating_sub(1),
            },
            &self.data[start_idx..end_idx],
        )
    }

    pub fn clean(&mut self, frame: usize) {
        let front_elements = self.adjust_frame(frame);

        if let Some(front_elements) = front_elements {
            if front_elements > 0 {
                self.data.drain(0..front_elements);
                self.front_frame = frame;
            }
        }
    }
}
