// const fn fib(n: u128) -> u128 {
//     let mut a = 1;
//     let mut b = 1;
//     for _ in 2..n {
//         let temp = a + b;
//         a = b;
//         b = temp;
//     }
//     b
// }

const fn fib(n: u128) -> u128 {
    let mut a = 1;
    let mut b = 1;
    let mut counter = 2;
    while counter < n {
        let tmp = a + b;
        a = b;
        b = tmp;
        counter += 1;
    }
    b
}
/*
This output works
constant-loops git:(main) ✗ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/constant-loops`
Fib 0 = 1
Fib 1 = 1
Fib 2 = 1
Fib 3 = 2
Fib 4 = 3

*/
fn main() {
    for i in 0..5 {
        println!("Fib {} = {}", i, fib(i));
    }
}

/*

Output will not complie as for the reasons stated below.
Marking a function as const causes the function to run at compile time rather
than at runtime. When a function runs at compile time, the compiler
calculates the results beforehand from constant inputs, which can help speed
up complex calculations that you might need later.
Suppose your program requires a lot of Fibonacci numbers. Without a const
function, your program would need to recalculate the numbers as needed, and
possibly more than once. However, by using a const function, you can store
these numbers as constant values in your program, dramatically improving
your program’s performance.
const functions—a relatively new Rust feature—are gradually becoming more
powerful. However, at the time of this publication, you cannot use the
following Rust features inside of a constant function:[38]
Floating-point operations (you can move them around, but you can’t
work with them).
Dynamic trait types.
Generic bounds on parameters other than Sized.
Raw pointer operations.
Union (enum) field access.
transmute and similar memory operations.
As it turns out, for loops fall into the unavailable category because they
require a Range—prohibited because of the generic bounds restriction. This
makes the example code fail to compile.
Other loop types work fine, though. For example, you can rewrite the
example using a while loop:

Using Constant Functions
As you just learned, constant functions can shift some of the calculation
burden to compile time, helping to speed up your program’s execution. Here
are a few scenarios in which constant functions are useful:

Programs often rely on the results of complex calculations with limited
sets of input. With const variables and functions, you can build lookup
tables of the required results, skipping the need to perform these
calculations at runtime.

Sometimes your program relies on a predetermined piece of math, yet
showing the work can help explain what your program does. Moving
that work to compile time removes the runtime performance penalty for
performing the calculation—and you’re still free to tweak the math in
the source code.

Note, however, that constant functions have some serious limitations on the
types of data they can use. An alternative is to write a separate program to
calculate a lookup table and then copy and paste the results into a const
variable.

Const Functions
https://doc.rust-lang.org/reference/const_eval.html#const-functions

*/
