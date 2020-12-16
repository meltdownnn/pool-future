//! A simple rust future pool implementation
//!
//! In rust, running single future is simple (using `.await`), and running all future at the same
//! time is also easy (using `futures::stream::FuturesUnordered`). However, fine-grained control of
//! the execution sequence is not that easy. Limiting the number of futures running at one time,
//! for example, is not intuitive. Thus, this simple crate provides a thin layer on top of
//! `FuturesUnordered` that provides the possibility to execute a given number of future at the same time.
use futures::stream::{FuturesUnordered, StreamExt};
/// A simple furure pool that store elements in a vector
pub struct VectorFuturePool<T: std::future::Future> {
    pub elements: Vec<T>,
    pub pool_size: usize,
}
impl<T: std::future::Future> VectorFuturePool<T> {
    pub fn new(elements: Vec<T>, pool_size: usize) -> Self {
        Self {
            elements,
            pool_size,
        }
    }
    /// Execute several future at the same time.
    ///
    /// Under normal situation, it will execute `pool_size` futures at once. If
    /// `self.elements.len()` are less
    /// than `pool_size`, only the remaining elements will be executed.
    ///
    /// Please note that it will only execute one "pool" of vector, meaning that there are maximum
    /// `pool_size` async functions running, but when a future is completed, no new function will
    /// be called.
    pub async fn execute(&mut self) -> Vec<<T as std::future::Future>::Output> {
        let remove_len = if self.elements.len() >= self.pool_size {
            self.pool_size
        } else {
            self.elements.len()
        };
        let current_execution: FuturesUnordered<_> = self.elements.drain(..remove_len).collect();
        current_execution.collect::<Vec<T::Output>>().await
    }
    /// Execute all futures in `self.elements`.
    ///
    /// When a future is completed, a new future will be called. Thus, there will be `pool_size`
    /// futures running at any time during the execution untill the amount of elements is not
    /// enough.
    pub async fn execute_till_complete(&mut self) -> Vec<<T as std::future::Future>::Output> {
        let remove_len = if self.elements.len() >= self.pool_size {
            self.pool_size
        } else {
            return self.execute().await;
        };
        let mut first_branch: FuturesUnordered<_> = self.elements.drain(..remove_len).collect();
        let mut result = Vec::new();
        while let Some(i) = first_branch.next().await {
            result.push(i);
            if self.elements.len() != 0 {
                // next() should internally call poll_next(), so this should be fine.
                first_branch.push(self.elements.remove(0));
            }
        }
    result
    }
    /// Whether elements are empty.
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

