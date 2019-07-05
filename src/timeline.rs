pub type Timeline<T> = Vec<(T, usize)>;

pub trait AtTime<T> {
    fn at_time(&self, time: usize) -> &T;
    fn try_time(&self, time: usize) -> Option<&T>;
    fn fix_duration(&mut self, duration: usize);
    fn duration(&self) -> usize;
}

impl<T> AtTime<T> for Timeline<T> {
    fn at_time(&self, time: usize) -> &T {
        self.try_time(time).expect("Time out of bounds.")
    }
    fn try_time(&self, mut time: usize) -> Option<&T> {
        for (data, duration) in self {
            if time < *duration {
                return Some(data);
            } else {
                time -= *duration;
            }
        }
        None
    }
    fn duration(&self) -> usize {
        self.iter().map(|(_, duration)| duration).sum()
    }
    fn fix_duration(&mut self, duration: usize) {
        if duration > 0 && !self.is_empty() {
            let diff = self.duration() as isize - duration as isize;
            if diff != 0 {
                if diff > 0 {
                    loop {
                        let diff = self.duration() as isize - self.duration() as isize;

                        let last_element = &mut self.last_mut().unwrap().1;
                        let new_duration = *last_element as isize - diff;
                        if new_duration <= 0 {
                            self.pop();
                        } else {
                            *last_element = new_duration as usize;
                            break;
                        }
                    }
                } else {
                    let last_element = &mut self.last_mut().unwrap().1;
                    *last_element += diff.abs() as usize;
                }
            }
        }
    }
}
