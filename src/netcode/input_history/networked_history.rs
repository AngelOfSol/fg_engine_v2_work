use super::InputRange;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Canon {
    Canon,
    Predicted,
    Empty,
}

#[derive(Debug)]
pub struct NetworkedHistory<T> {
    front_frame: usize,
    canon: Vec<Canon>,
    data: Vec<T>,
}

pub enum PredictionResult {
    Unpredicted,
    Correct,
    Wrong,
}

impl<T: Default + Clone + PartialEq> NetworkedHistory<T> {
    pub fn new() -> Self {
        Self {
            front_frame: 0,
            canon: vec![],
            data: vec![],
        }
    }

    fn adjust_frame(&self, frame: usize) -> Option<usize> {
        frame.checked_sub(self.front_frame)
    }
    fn adjust_idx(&self, idx: usize) -> usize {
        idx + self.front_frame
    }

    pub fn add_input(&mut self, frame: usize, data: T) -> PredictionResult {
        // this complex logic, and the array of canonicalness
        // help us not throw out data that has arrived, even if its not
        // data thats immediately necessary
        let relative_frame = self.adjust_frame(frame);
        if relative_frame.is_none() {
            dbg!("got data from the past");
            // input is too far in the past, so we can't actually care about it.
            return PredictionResult::Unpredicted;
        }
        let relative_frame = relative_frame.unwrap();

        if relative_frame == self.data.len() {
            self.canon.push(Canon::Canon);
            self.data.push(data);
            PredictionResult::Unpredicted
        } else if relative_frame > self.data.len() {
            self.canon.resize(relative_frame + 1, Canon::Empty);
            self.data.resize(relative_frame + 1, Default::default());

            self.canon[relative_frame] = Canon::Canon;
            self.data[relative_frame] = data;
            PredictionResult::Unpredicted
        } else {
            match self.canon[relative_frame] {
                Canon::Canon => {
                    //
                    if self.data[relative_frame] == data {
                    } else {
                        dbg!("canon data is different than recieved data");
                    }

                    PredictionResult::Unpredicted
                }
                Canon::Empty => {
                    self.canon[relative_frame] = Canon::Canon;
                    self.data[relative_frame] = data;
                    PredictionResult::Unpredicted
                }
                Canon::Predicted => {
                    self.canon[relative_frame] = Canon::Canon;
                    if self.data[relative_frame] == data {
                        PredictionResult::Correct
                    } else {
                        self.data[relative_frame] = data;
                        PredictionResult::Wrong
                    }
                }
            }
        }
    }
    pub fn has_input(&self, frame: usize) -> bool {
        self.adjust_frame(frame)
            .and_then(|frame| self.canon.get(frame))
            .map(|canon| *canon != Canon::Empty)
            .unwrap_or(false)
    }

    pub fn is_predicted_input(&self, frame: usize) -> bool {
        self.adjust_frame(frame)
            .and_then(|frame| self.canon.get(frame))
            .map(|canon| *canon == Canon::Predicted)
            .unwrap_or(false)
    }

    pub fn is_empty_input(&self, frame: usize) -> bool {
        self.adjust_frame(frame)
            .and_then(|frame| self.canon.get(frame))
            .map(|canon| *canon == Canon::Empty)
            .unwrap_or(true)
    }

    // TODO, rename to request_inputs
    // to show that its a best effort to get the query'd for set of inputs
    // and not a guarentee that you get the inputs you want
    // also input range returned is considered a closed interval
    // TODO: deal with what happens if you can't get any inputs
    pub fn get_inputs(&self, frame: usize, amt: usize) -> (InputRange, &[T]) {
        let frame = self.adjust_frame(frame).unwrap();
        let end_idx = self.data.len().min(frame + 1);
        let start_idx = end_idx.checked_sub(amt).unwrap_or(0);

        (
            InputRange {
                first: self.adjust_idx(start_idx),
                last: self.adjust_idx(end_idx - 1),
            },
            &self.data[start_idx..end_idx],
        )
    }
    pub fn predict(&mut self, frame: usize) {
        let frame = self.adjust_frame(frame).unwrap();
        let data = frame
            .checked_sub(1)
            .and_then(|frame| self.data.get(frame))
            .cloned()
            .unwrap_or_default();

        if frame == self.data.len() {
            self.canon.push(Canon::Predicted);
            self.data.push(data);
        } else {
            assert_eq!(
                self.canon[frame],
                Canon::Empty,
                "Predicted input should not already be predicted or canon."
            );

            self.canon[frame] = Canon::Predicted;
            self.data[frame] = data;
        }
    }

    pub fn repredict(&mut self, frame: usize) {
        let frame = self.adjust_frame(frame).unwrap();
        let data = frame
            .checked_sub(1)
            .and_then(|frame| self.data.get(frame))
            .cloned()
            .unwrap_or_default();

        assert_eq!(self.canon[frame], Canon::Predicted);
        self.data[frame] = data;
    }

    pub fn clean(&mut self, frame: usize) {
        let front_elements = self.adjust_frame(frame);

        if let Some(front_elements) = front_elements {
            if front_elements > 0 {
                self.data.drain(0..front_elements);
                let test = self
                    .canon
                    .drain(0..front_elements)
                    .all(|item| item == Canon::Canon);
                assert!(test, "dropped off inputs that were predicted or empty");
                self.front_frame = frame;
            }
        }
    }
}
