async fn hello() {
    println!("Hello World!");
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>>{
    // Won't return anything until we add it the Executors queue. See methods below
    // hello();
    hello().await;

    // Output - Hello World!
    /*
     waiting-for-godot git:(main) ✗ cargo run
   Compiling waiting-for-godot v0.1.0 (/Users/squeeko/RustBrainTeasers/waiting-for-godot)
    Finished dev [unoptimized + debuginfo] target(s) in 0.44s
     Running `target/debug/waiting-for-godot`
Hello World!
    */
    Ok(())
}

/*
NO Output

You explored asynchronous task scheduling in Puzzle 19, Sleepless in Tokio,
where you learned that tasks are executed by your program’s Executor, running
until they yield control back to the task queue. Tasks only yield when they
return a value or error, call another asynchronous task, finish execution, or
call yield_now. Tasks also don’t start when you call them. When you call a
function decorated with the async keyword, it returns a Future.[36] Calling an
async-decorated function doesn’t execute the function—instead, it packages it
up for future execution.

Here's the lifecycle of an "async" function

1. Create an asynchronous function, using the async keyword.
async fn my_function() {...}

2. Executing the function creates a "Future", wrapping your function in a promise of 
a future execution
let promise = my_function(); ----> Returns a Future<my_function>    
                                    my_function doesn't execute yet!

3. Add the Future<my_function> to the Executor's task queue:
You may use any of these methods

my_function().await
spawn(promise)
join!(promise, -other futures-)
select!(promise, -other futures-)

Executing an async function is a two-step process. The first creates a promise
of future execution. Separating these steps gives you the flexibility to decide
how you want to actually run your function.

Asynchronous Programming in Rust
https://rust-lang.github.io/async-book/
Rust Futures
https://doc.rust-lang.org/std/future/trait.Future.html
Demystifying Closures, Futures and async-await in Rust-Part 3: Async &
Await
https://medium.com/@alistairisrael/demystifying-closures-futures-andasync-await-in-rust-part-3-async-await-9ed20eede7a4

https://medium.com/swlh/demystifying-closures-futures-and-async-await-in-rust-part-1-closures-97e531e4dc50

https://levelup.gitconnected.com/demystifying-closures-futures-and-async-await-in-rust-part-2-futures-abe95ab332a2
*/
