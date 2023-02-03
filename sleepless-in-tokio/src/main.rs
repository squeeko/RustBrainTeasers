use tokio::join;
use std::thread;
use std::time::Duration;

/* async */ fn count_and_wait(n: u64) -> u64 {
    println!("Starting {}", n);
    std::thread::sleep(Duration::from_millis(n * 100));
    println!("Returning {}", n);
    n

}
// #[tokio::main]
/* async */ fn main() -> Result<(), Box<dyn std::error::Error>> {
    /*
    Join runs multiple tasks concurrently and returns when they all
    complete execution.
    */
    // join!(count_and_wait(1), count_and_wait(2), count_and_wait(3));
    let a = thread::spawn(|| count_and_wait(1));
    let b = thread::spawn(|| count_and_wait(2));
    let c = thread::spawn(|| count_and_wait(3));
    a.join().unwrap();
    b.join().unwrap();
    c.join().unwrap();
    Ok(())
}

/*
Async Threaded Output 

Starting 1
Returning 1
Starting 2
Returning 2
Starting 3
Returning 3

Purely Threaded Output

Starting 1
Starting 3
Starting 2
Returning 1
Returning 2
Returning 3

The outcome is surprising because the join macro promises to run the three
instances of count_and_wait concurrently, but the output shows that the tasks are
running sequentially, which tends to surprise newcomers to Rust’s async
system. Understanding the differences between asynchronous and thread
programming can help you avoid pitfalls—and help you pick the right model
for your program.
Asynchronous programs and multithreaded programs operate differently,
each with their own strengths and weaknesses. Asynchronous (Future-based)
tasks aren’t the same as threaded tasks, and they require some care to ensure
that they operate concurrently. However, it’s entirely possible to run an
asynchronous program on one thread.
In a threaded model, each task operates inside a full operating systemsupported
thread. Threads are scheduled independently of other threads and
processes. An asynchronous model stores tasks in a task queue and runs them
until they yield control back to the executing program.


Asynchronous Programming in Rust
https://rust-lang.github.io/asyncbook/
01_getting_started/01_chapter.html
Rust Futures
https://github.com/rust-lang/futures-rs
Tokio
https://github.com/rayon-rs/rayon
Async-Std
https://github.com/async-rs/async-std
Rayon
https://github.com/rayon-rs/rayon
*/
