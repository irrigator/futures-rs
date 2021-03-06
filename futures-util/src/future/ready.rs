use core::marker::Unpin;
use core::mem::PinMut;
use futures_core::future::Future;
use futures_core::task::{self, Poll};

/// A future that is immediately ready with a value
#[derive(Debug, Clone)]
#[must_use = "futures do nothing unless polled"]
pub struct Ready<T>(Option<T>);

impl<T> Unpin for Ready<T> {}

impl<T> Future for Ready<T> {
    type Output = T;

    #[inline]
    fn poll(mut self: PinMut<Self>, _cx: &mut task::Context) -> Poll<T> {
        Poll::Ready(self.0.take().unwrap())
    }
}

/// Create a future that is immediately ready with a value.
pub fn ready<T>(t: T) -> Ready<T> {
    Ready(Some(t))
}
