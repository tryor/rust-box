use std::marker::Unpin;
use std::pin::Pin;
use std::task::{Context, Poll};

use futures::Stream;

#[allow(unreachable_pub)]
pub use self::into_stream::IntoStream;
#[allow(unreachable_pub)]
pub use self::sender::Sender;

mod into_stream;
mod sender;

pub trait Waker {
    fn wake(&self);
}

impl<T: ?Sized> QueueExt for T {}

pub trait QueueExt {
    fn into_stream<Item, F>(self, f: F) -> IntoStream<Self, Item, F>
        where
            Self: Sized + Unpin,
            F: Fn(Pin<&mut Self>, &mut Context<'_>) -> Poll<Option<Item>>,
    {
        assert_stream::<Item, _>(IntoStream::new(self, f))
    }

    fn sender<Item, F, R>(self, f: F) -> Sender<Self, Item, F, R>
        where
            Self: Sized + Waker,
            F: Fn(&mut Self, Item) -> R,
    {
        Sender::new(self, f)
    }
}

// Just a helper function to ensure the streams we're returning all have the
// right implementations.
#[inline]
pub(crate) fn assert_stream<T, S>(stream: S) -> S
    where
        S: Stream<Item=T>,
{
    stream
}