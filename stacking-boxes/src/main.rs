// Adds the Box_syntax feature
// Add the "rustup default nightly" to use this "Experimental Feature"
#![feature(box_syntax)]
fn main() {
    // let c = Box::new([0u32; 10_000_000]);
    let c = box [0; 10_000_000];

    println!("{}", c.len());
}
/*
This is the result when we use the steps above, "rustup default nightly" and the
#![feature(box_syntax)]

 stacking-boxes git:(main) ✗ cargo run
   Compiling stacking-boxes v0.1.0 (/Users/squeeko/RustBrainTeasers/stacking-boxes)
    Finished dev [unoptimized + debuginfo] target(s) in 0.27s
     Running `target/debug/stacking-boxes`
10000000

*/

/*
Error shown

stacking-boxes git:(main) ✗ cargo run
   Compiling stacking-boxes v0.1.0 (/Users/squeeko/RustBrainTeasers/stacking-boxes)
    Finished dev [unoptimized + debuginfo] target(s) in 0.27s
     Running `target/debug/stacking-boxes`

thread 'main' has overflowed its stack
fatal runtime error: stack overflow
[1]    84792 abort      cargo run

In debug mode, the program will crash with the message thread ’main’ has overflowed
its stack. In release mode, the program will display the following output: 10000000.

10,000,000 32-bit integers requires 40Mb of memory to store—larger than
the default stack. Attempting to get around this limitation, the code uses Box
to place the structure on the heap. However, in debug mode, Rust first allocates
the array on the stack and then moves it to the heap, crashing the program.
To better understand the example, let’s look at how program memory is
arranged.

Stack - The stack stores local variables, function parameters, and the call stack—a
list of functions your program has called, and the point to which execution
should return when a function finishes execution. Uses "push" and "pop" to add to the stack
and remove from the stack respectively

The stack sees constant use while your program runs:
Whenever you create a local variable (a variable that exists only in the
current scope), the variable gets pushed to the stack. Local variables can
be references, in which case the reference is stored on the stack—but the
referred-to data may be anywhere.

Whenever you leave a scope, the scope’s variables are popped from the
stack.

Whenever you call a function, the parameters to the function are pushed
to the stack, along with the memory address to which the function
should return when its operation is complete.

The stack also provides a handy debugging tool. Every function call is
pushed to the stack; if your program crashes, Rust walks the stack to show
you a stack trace describing the state of your program when it crashed.

Heap - The heap—another area of memory that every program maintains—is limited
by your computer’s available memory, virtual memory, and operating system
limitations. Heap memory is large and may or may not be contiguous,
depending upon your operating system.
Storing data on the heap requires more steps than storing data on the stack.
First, Rust’s standard library needs to request a heap allocation that returns a
pointer to a usable area of memory. It then needs to store the pointer before it
can write data to the heap.
Reading data from the heap also requires a little more work: to read data,
your program first needs to read the pointer to determine where the heap data
is stored. Once it knows the location, the program can read the data from
there.
Because of the extra steps required for heap read/write access—particularly
with frequent allocations—accessing data on the heap can be a lot slower
than accessing data on the stack. Why? Because the CPU’s memory cache
will try its best to keep your heap data available—but heap-allocated data is
often large and is more likely to be evicted from the cache than the program’s
stack. The heap is also less likely to be contiguous in memory than the stack,
making cache misses more likely.

Tip: Vec automatically uses the Heap. Please use instead of Array....

The Stack and the Heap
https://doc.rust-lang.org/1.22.0/book/first-edition/the-stack-and-theheap.
html
Box Type
https://doc.rust-lang.org/std/boxed/struct.Box.html
box_syntax
https://doc.rust-lang.org/beta/unstable-book/language-features/boxsyntax.
html
*/
