use futures::future::join_all;
use futures::prelude::*;
use log::*;
use std::io::Write;
use tokio::{task, time};

/**
async fn our_async_program() {
    todo!()
}

fn fib_cpu_intensive(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        n => fib_cpu_intensive(n - 1) + fib_cpu_intensive(n - 2)
    }
}

async fn app() {
    // Spawning a Future on the runtime (when you want to run futures concurrently)
    let concurent_future = task::spawn(our_async_program());

    // Spawning blocking or CPU-intensive tasks
    let threadpool_future = task::spawn_blocking(|| fib_cpu_intensive(30));
    todo!()
}
 */

// Just a generic Result type to ease error handling for us. Errors in multithreaded
// async contexts needs some extra restrictions

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

async fn get_and_analyze(n: usize) -> Result<(u64, u64)> {
    time::sleep(time::Duration::from_millis(1000)).await;
    let response: reqwest::Response = reqwest::get("https://example.com").await?;
    info!("Dataset {}", n);

    let req_body = response.text().await?;

    let res = task::spawn_blocking(move || analyze(&req_body)).await?;
    info!("Processed {}", n);
    Ok(res)
}

fn analyze(txt: &str) -> (u64, u64) {
    let txt = txt.as_bytes();

    let ones = txt
        .iter()
        .fold(0u64, |acc, b: &u8| acc + (b.count_ones() as u64));
    let zeros = txt
        .iter()
        .fold(0u64, |acc, b: &u8| acc + (b.count_zeros() as u64));

    (ones, zeros)
}

async fn app() -> Result<()> {
    let mut futures = vec![];

    for i in 1..=10 {
        let fut = task::spawn(get_and_analyze(i));
        futures.push(fut);
    }

    let results = join_all(futures).await;

    let mut total_ones = 0;
    let mut total_zeros = 0;

    // Returning errors using `?` in iterators can be a bit difficult. Using a
    // simple for loop to inspect and work with our results can often be more
    // ergonomic
    for result in results {
        // `spawn_blocking` returns a `JoinResult` we need to unwrap first
        let ones_res: Result<(u64, u64)> = result?;
        let (ones, zeros) = ones_res?;
        total_ones += ones;
        total_zeros += zeros;
    }

    info!(
        "Ration of ones/zeros: {:.02}",
        (total_ones as f64) / (total_zeros as f64)
    );
    Ok(())
}

// short handed way if you dont need to edit options
// #[main]
// async fn main() {
//     env_logger::init();
//     app().await;
// }

// another way to write main() with tokio
fn main() {}
