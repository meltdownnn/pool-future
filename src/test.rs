#[test]
fn check_init() {
    super::VectorFuturePool::new(vec![plus(1, 2), plus(1, 1), plus(1, 3), plus(1, 4)], 2);
}
#[test]
fn check_drain() {
    let mut result = futures::executor::block_on(super::VectorFuturePool::new(vec![plus(1, 2), plus(1, 1), plus(1, 3), plus(1, 4)], 2).execute_till_complete());
    result.sort();
    assert_eq!(result, vec![2, 3, 4, 5]);
}
#[test]
fn check_add() {
    let mut pool = super::VectorFuturePool::new(vec![plus(1, 2), plus(1, 1), plus(1, 3), plus(1, 4)], 2);
    pool.elements.push(plus(1, 0));
    let mut result = futures::executor::block_on(pool.execute_till_complete());
    result.sort();
    assert_eq!(result, vec![1, 2, 3, 4, 5]);
}
#[test]
fn check_execute() {
    futures::executor::block_on(super::VectorFuturePool::new(vec![plus(1, 2), plus(1, 1), plus(1, 3), plus(1, 4)], 2).execute());
}
#[test]
fn check_next() {
    use futures::StreamExt;
    let mut pool = super::VectorFuturePool::new(vec![plus(1, 2), plus(1, 1), plus(1, 3), plus(1, 4)], 2);
    while let Some(_) = futures::executor::block_on(pool.next()) {}
    assert!(pool.is_empty());
}
async fn plus<T: std::ops::Add>(a: T, b: T) -> T::Output {
    a + b
}
