use futures::prelude::*;
use tokio::{ main, task };

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

#[main]
async fn main() {
    println!("Hello, world!");
}
