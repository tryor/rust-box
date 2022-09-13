use std::fmt;
use std::marker::PhantomData;

use super::Waker;

pub struct Sender<S, Item, F, R> {
    s: S,
    f: F,
    _item: PhantomData<Item>,
    _r: PhantomData<R>,
}

impl<S, Item, F, R> Clone for Sender<S, Item, F, R>
    where
        S: Clone,
        F: Clone,
{
    fn clone(&self) -> Self {
        Self {
            s: self.s.clone(),
            f: self.f.clone(),
            _item: PhantomData,
            _r: PhantomData,
        }
    }
}

impl<S, Item, F, R> fmt::Debug for Sender<S, Item, F, R>
    where
        S: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Sender").field("stream", &self.s).finish()
    }
}

impl<S, Item, F, R> Sender<S, Item, F, R>
    where
        S: Waker,
        F: Fn(&mut S, Item) -> R,
{
    pub(super) fn new(s: S, f: F) -> Self {
        Self {
            s,
            f,
            _item: PhantomData,
            _r: PhantomData,
        }
    }

    pub fn as_mut(&mut self) -> &mut S {
        &mut self.s
    }

    pub fn as_ref(&self) -> &S {
        &self.s
    }

    pub fn send(&mut self, v: Item) -> R {
        let res = (self.f)(&mut self.s, v);
        self.s.wake();
        res
    }
}
