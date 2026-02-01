// visualisation_module/src/utils/queue.rs

use crossbeam::queue::SegQueue;
use std::sync::Arc;

pub struct SharedQueue<T> {
    inner: Arc<SegQueue<T>>,
}

impl<T> Clone for SharedQueue<T> {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}

impl<T> SharedQueue<T> {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(SegQueue::new()),
        }
    }

    #[inline]
    pub fn push(&self, item: T) {
        self.inner.push(item);
    }

    #[inline]
    pub fn pop(&self) -> Option<T> {
        self.inner.pop()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.inner.len()
    }
}
