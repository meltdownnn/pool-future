//! A simple rust future pool implementation
//!
//! In rust, running single future is simple (using `.await`), and running all future at the same
//! time is also easy (using `futures::stream::FuturesUnordered`). However, fine-grained control of
//! the execution sequence is not that easy. Limiting the number of futures running at one time,
//! for example, is not intuitive. Thus, this simple crate provides a thin layer on top of
//! `FuturesUnordered` that provides the possibility to execute a given number of future at the same time.
#[cfg(test)]
mod test;
use futures::stream::{FuturesUnordered, StreamExt};
/// A simple furure pool that store elements in a vector
pub struct VectorFuturePool<T: std::future::Future> {
    pub elements: Box<Vec<T>>,
    pub pool_size: usize,
    future_pool: FuturesUnordered<T>,
}
impl<T: std::future::Future> core::ops::Deref for VectorFuturePool<T> {
    type Target = FuturesUnordered<T>;
    fn deref(&self) -> &Self::Target {
        &self.future_pool
    }
}
impl<T: std::future::Future> core::ops::DerefMut for VectorFuturePool<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.future_pool
    }
}
impl<T: std::future::Future> futures::Stream for VectorFuturePool<T> {
    type Item = T::Output;
    fn poll_next(
        self: core::pin::Pin<&mut Self>,
        cx: &mut futures::task::Context<'_>,
    ) -> futures::task::Poll<Option<Self::Item>> {
        let reference = core::pin::Pin::get_mut(self);
        reference.fill_pool();
        futures::stream::Stream::poll_next(core::pin::Pin::new(&mut reference.future_pool), cx)
    }
}
impl<T: std::future::Future> VectorFuturePool<T> {
    pub fn new(elements: Vec<T>, pool_size: usize) -> Self {
        Self {
            elements: Box::new(elements),
            pool_size,
            future_pool: FuturesUnordered::new(),
        }
    }
    fn fill_pool(&mut self) {
        while (self.future_pool.len() != self.pool_size) && (self.elements.len() != 0) {
            self.future_pool.push(self.elements.remove(0))
        }
    }
    async fn drain_future_pool(&mut self) -> Vec<T::Output> {
        let mut new_future_pool = FuturesUnordered::new();
        std::mem::swap(&mut self.future_pool, &mut new_future_pool);
        new_future_pool.collect().await
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
        self.fill_pool();
        self.drain_future_pool().await
    }
    /// Execute all futures in `self.elements`.
    ///
    /// When a future is completed, a new future will be called. Thus, there will be `pool_size`
    /// futures running at any time during the execution untill the amount of elements is not
    /// enough.
    pub async fn execute_till_complete(&mut self) -> Vec<<T as std::future::Future>::Output> {
        let mut result = Vec::with_capacity(self.elements.len() + self.future_pool.len());
        while (self.elements.len() != 0) | (self.future_pool.len() != 0) {
            self.fill_pool();
            result.append(&mut self.drain_future_pool().await)
        }
        result
    }
    /// Whether elements are empty.
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}
