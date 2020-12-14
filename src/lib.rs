//! A simple rust future pool implementation
//!
//! In rust, running single future is simple (using `.await`), and running all future at the same
//! time is also easy (using `futures::stream::FuturesUnordered`). However, fine-grained control of
//! the execution sequence is not that easy. Limiting the number of futures running at one time,
//! for example, is not intuitive. Thus, this simple crate provides a thin layer on top of
//! `FuturesUnordered` that provides the possibility to execute a given of future at the same time.
use futures::stream::StreamExt;
pub struct FuturePool<T: std::future::Future + core::marker::Unpin> {
    pub elements: Vec<T>,
    pub pool_size: usize
}
impl<T: std::future::Future + core::marker::Unpin> FuturePool<T> {
    pub fn new(elements: Vec<T>, pool_size: usize) -> Self {
        Self {
            elements,
            pool_size
        }
    }
    pub async fn execute(&mut self) -> Vec<<T as std::future::Future>::Output> {
        let remove_len = if self.elements.len() >= self.pool_size {
            self.pool_size
        } else {
            self.elements.len()
        };
        let current_execution: futures::stream::FuturesUnordered<_> = self.elements.drain(..remove_len).collect();
        current_execution.collect::<Vec<T::Output>>().await
    }
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}
