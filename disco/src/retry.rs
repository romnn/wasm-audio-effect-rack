use std::convert::From;
use std::result::Result;
// use tokio::time::{sleep, Duration};
use tokio;

#[derive(Debug)]
pub struct Fixed {
    duration: std::time::Duration,
}

impl Fixed {
    pub fn from_millis(millis: u64) -> Self {
        Fixed {
            duration: std::time::Duration::from_millis(millis),
        }
    }

    pub fn from_secs(secs: u64) -> Self {
        Fixed::from_millis(secs * 1000)
    }
}

impl Iterator for Fixed {
    type Item = std::time::Duration;

    fn next(&mut self) -> Option<std::time::Duration> {
        Some(self.duration)
    }
}

impl From<std::time::Duration> for Fixed {
    fn from(delay: std::time::Duration) -> Self {
        Self { duration: delay }
    }
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub enum OperationResult<T, E> {
    Ok(T),
    Retry(E),
    #[allow(dead_code)]
    Err(E),
}

impl<T, E> From<Result<T, E>> for OperationResult<T, E> {
    fn from(item: Result<T, E>) -> Self {
        match item {
            Ok(v) => OperationResult::Ok(v),
            Err(e) => OperationResult::Retry(e),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct RetryError<E> {
    pub error: E,
    pub total_delay: std::time::Duration,
    pub tries: u64,
}

pub async fn retry<I, O, R, E, OR>(iterable: I, operation: O) -> Result<R, RetryError<E>>
where
    I: IntoIterator<Item = std::time::Duration>,
    O: Fn(u64) -> OR,
    OR: futures::future::Future<Output = OperationResult<R, E>>,
{
    let mut iterator = iterable.into_iter();
    let mut current_try = 1;
    let mut total_delay = std::time::Duration::default();
    loop {
        match operation(current_try).await {
            OperationResult::Ok(value) => return Ok(value),
            OperationResult::Retry(error) => {
                if let Some(delay) = iterator.next() {
                    // there are still retries
                    println!(
                        "operation failed, waiting {} seconds ({})",
                        delay.as_secs(),
                        current_try
                    );
                    tokio::time::sleep(delay).await;
                    // tokio::time::delay_for(delay).await;
                    current_try += 1;
                    total_delay += delay;
                } else {
                    // no more retries left
                    return Err(RetryError {
                        error,
                        total_delay,
                        tries: current_try,
                    });
                }
            }
            OperationResult::Err(error) => {
                // operation signaled to stop retrying
                return Err(RetryError {
                    error,
                    total_delay,
                    tries: current_try,
                });
            }
        }
    }
}
