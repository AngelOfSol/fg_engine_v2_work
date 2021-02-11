use futures::channel::oneshot::Receiver;

pub struct Pending<T>(pub(crate) Receiver<T>);
pub enum Query<T, E> {
    Waiting(Pending<Result<T, E>>),
    Ok(T),
    Err(E),
    Cancelled,
    NotStarted,
}

impl<T, E> Default for Query<T, E> {
    fn default() -> Self {
        Self::NotStarted
    }
}

impl<T, E> Query<T, E> {
    pub fn poll(&mut self) {
        if let Query::Waiting(receiver) = self {
            match receiver.0.try_recv() {
                Ok(value) => match value {
                    Some(Ok(value)) => *self = Self::Ok(value),
                    Some(Err(error)) => *self = Self::Err(error),
                    None => {}
                },
                Err(_) => *self = Self::Cancelled,
            }
        }
    }

    pub fn take(&mut self) -> Self {
        std::mem::take(self)
    }

    pub fn unwrap(self) -> T {
        if let Self::Ok(value) = self {
            value
        } else {
            panic!(
                "expected Ok(_), found {}",
                match self {
                    Query::Waiting(_) => {
                        "Waiting(_)"
                    }
                    Query::Ok(_) => {
                        unreachable!()
                    }
                    Query::Err(_) => {
                        "Err(_)"
                    }
                    Query::Cancelled => {
                        "Cancelled"
                    }
                    Query::NotStarted => {
                        "NotStarted"
                    }
                }
            )
        }
    }
}
