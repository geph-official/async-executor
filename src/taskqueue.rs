use std::{collections::VecDeque, sync::Mutex};

use async_task::Runnable;
use concurrent_queue::ConcurrentQueue;

#[derive(Debug)]
pub struct GlobalQueue {
    inner: ConcurrentQueue<Runnable>,
}

impl Default for GlobalQueue {
    fn default() -> Self {
        Self {
            inner: ConcurrentQueue::unbounded(),
        }
    }
}

impl GlobalQueue {
    pub fn push(&self, task: Runnable) {
        // eprintln!("pushing global queue length {}", self.inner.len());
        self.inner.push(task).unwrap()
    }

    pub fn pop(&self) -> Option<Runnable> {
        self.inner.pop().ok()
    }
}

#[derive(Debug)]
pub struct LocalQueue {
    inner: parking_lot::Mutex<VecDeque<Runnable>>,
}

impl Default for LocalQueue {
    fn default() -> Self {
        Self {
            inner: Default::default(),
        }
    }
}

impl LocalQueue {
    pub fn push(&self, task: Runnable) -> Result<(), Runnable> {
        // eprintln!("pushing local queue length {}", self.inner.len());
        self.inner.lock().push_front(task);
        Ok(())
    }

    pub fn pop(&self) -> Option<Runnable> {
        self.inner.lock().pop_front()
    }

    pub fn steal_global(&self, other: &GlobalQueue) {
        let count = (other.inner.len() + 1) / 2;
        let mut inner = self.inner.lock();

        if count > 0 {
            // // Don't steal more than fits into the queue.
            // if let Some(cap) = self.inner.capacity() {
            //     count = count.min(cap - self.inner.len());
            // }

            // Steal tasks.
            for _ in 0..count {
                if let Some(t) = other.pop() {
                    inner.push_front(t);
                } else {
                    break;
                }
            }
        }
    }

    pub fn steal_local(&self, other: &LocalQueue) {
        // let mut inner = self.inner.lock();
        // let mut their_inner = other.inner.lock();
        // let count = (other.inner.lock().len() + 1) / 2;

        // if count > 0 {
        //     // Steal tasks.
        //     for _ in 0..count {
        //         if let Some(t) = other.inner.lock().pop_back() {
        //             // assert!(self.inner.push(t).is_ok());
        //             self.inner.lock().push_front(t);
        //         } else {
        //             break;
        //         }
        //     }
        // }
    }
}
