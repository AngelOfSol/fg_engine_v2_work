pub type Timeline<T> = Vec<(T, usize)>;

pub trait AtTime<T> {
    fn at_time(&self, time: usize) -> &T;
    fn try_time(&self, time: usize) -> Option<&T>;
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
}
