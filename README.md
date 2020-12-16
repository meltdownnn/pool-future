# pool-future: a thread pool-like future library in rust

In rust, running single future is simple (using `.await`), and running all future at the same time is also easy (using `futures::stream::FuturesUnordered`). However, fine-grained control of the execution sequence is not that easy. Limiting the number of futures running at one time, for example, is not intuitive. Thus, this simple crate provides a thin layer on top of `FuturesUnordered` that provides the possibility to execute a given number of future at the same time.

For example,
```rust
async fn a_async_function(id: usize) -> String {
    id.to_string()
}
VectorFuturePool::new((0..10).iter().map(|x| a_async_function(x)).collect::<Vec<_>>(), 3).execute_till_complete();
```
will make sure no more than 3 functions run at the same time.
