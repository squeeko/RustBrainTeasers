fn double_it(n: u64, _: i32) -> u64 {
    n * 2
}

fn main() {
    let one: i32 = 1;
    let n = double_it(one as _, 3);
    println!("{:?}", n);
}
/*
When you combine type inference
issues with the potential precision loss from the as keyword, you’re inviting
trouble. In many cases, you should use into (or try_into) instead of as altogether.

1. Using into() is precise and optimizes very well.
2. try_into() lets you handle failed conversions.
3. Use as type when you are certain that conversions are safe.
4. Use as _ when you are really stuck.

Rust’s underscore (or placeholder) symbol has different meanings in
different contexts:
When used as a variable name prefix (for example, _ignore_me : i32), the
underscore indicates to Rust that the variable is deliberately unused and
suppresses “unused variable” warnings.
When used as an entire variable name, you’re telling Rust that you never
intend to use the variable. When used in a match statement (for example, _
=> { .. }), the underscore indicates a default action. If no other match branch
is selected, then the default action will be evaluated.
Underscores can be used with functions that return a value marked with
#[must_use]. For example, let _ = my_important_function() will ignore the result of
the function, suppressing errors or warnings that you’re not using the
result.


Underscore
https://runrust.miraheze.org/wiki/Underscore
As
https://doc.rust-lang.org/std/keyword.as.html


*/
