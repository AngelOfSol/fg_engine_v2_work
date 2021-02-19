// use std::{
//     pin::Pin,
//     sync::Arc,
//     task::{Context, Poll},
//     time::{Duration, Instant},
// };

// use futures::Future;
// use smol::Executor;
// use turbulence::{BufferPool, Runtime};

// #[derive(Clone)]
// pub struct TurbulenceRuntime {
//     pub handle: Arc<Executor<'static>>,
// }

// impl From<Arc<Executor<'static>>> for TurbulenceRuntime {
//     fn from(value: Arc<Executor<'static>>) -> Self {
//         Self { handle: value }
//     }
// }

// impl Runtime for TurbulenceRuntime {
//     type Instant = Instant;

//     type Sleep = Sleeper;

//     fn spawn<F>(&self, future: F)
//     where
//         F: Future<Output = ()> + Send + 'static,
//     {
//         self.handle.spawn(future).detach()
//     }

//     fn now(&self) -> Self::Instant {
//         Instant::now()
//     }

//     fn elapsed(&self, instant: Self::Instant) -> Duration {
//         instant.elapsed()
//     }

//     fn duration_between(&self, earlier: Self::Instant, later: Self::Instant) -> Duration {
//         later.duration_since(earlier)
//     }

//     fn sleep(&self, duration: Duration) -> Self::Sleep {
//         Sleeper {
//             start_time: Instant::now(),
//             duration,
//         }
//     }
// }

// #[derive(Debug, Copy, Clone)]
// pub struct SimpleBufferPool(pub usize);

// impl BufferPool for SimpleBufferPool {
//     type Buffer = Box<[u8]>;

//     fn acquire(&self) -> Self::Buffer {
//         vec![0; self.0].into_boxed_slice()
//     }
// }

// pub struct Sleeper {
//     start_time: Instant,
//     duration: Duration,
// }

// impl Future for Sleeper {
//     type Output = ();

//     fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
//         let now = Instant::now();

//         if now.duration_since(self.start_time) >= self.duration {
//             cx.waker().clone().wake();
//             Poll::Ready(())
//         } else {
//             Poll::Pending
//         }
//     }
// }

// #[derive(Clone)]
// pub struct TR2<'a> {
//     pub handle: Arc<Executor<'a>>,
// }

// impl<'a> Runtime for TR2<'a> {
//     type Instant = Instant;

//     type Sleep = Sleeper;

//     fn spawn<F>(&self, future: F)
//     where
//         F: Future<Output = ()> + Send + 'static,
//     {
//         self.handle.spawn(future).detach()
//     }

//     fn now(&self) -> Self::Instant {
//         Instant::now()
//     }

//     fn elapsed(&self, instant: Self::Instant) -> Duration {
//         instant.elapsed()
//     }

//     fn duration_between(&self, earlier: Self::Instant, later: Self::Instant) -> Duration {
//         later.duration_since(earlier)
//     }

//     fn sleep(&self, duration: Duration) -> Self::Sleep {
//         Sleeper {
//             start_time: Instant::now(),
//             duration,
//         }
//     }
// }
