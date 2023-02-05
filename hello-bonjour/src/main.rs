fn main() {
    let hello = || println!("Hello World!");
    let hello = || println!("Bonjour le monde");
    hello();
}
/*
hello-bonjour git:(main) ✗ cargo run
   Compiling hello-bonjour v0.1.0 (/Users/squeeko/RustBrainTeasers/hello-bonjour)
warning: unused variable: `hello`
 --> src/main.rs:2:9
  |
2 |     let hello = || println!("Hello World!");
  |         ^^^^^ help: if this is intentional, prefix it with an underscore: `_hello`
  |
  = note: `#[warn(unused_variables)]` on by default

warning: `hello-bonjour` (bin "hello-bonjour") generated 1 warning (run `cargo fix --bin "hello-bonjour"` to apply 1 suggestion)
    Finished dev [unoptimized + debuginfo] target(s) in 1.55s
     Running `target/debug/hello-bonjour`
Bonjour le monde

Function
shadowing rules don’t apply to closures (sometimes known as lambda
functions).
Closures don’t actually have names. The variable that holds a closure is a
pointer to the area of memory containing the function. The variables that
point to the closures are subject to the same shadowing rules as other
variables (Puzzle 13, Reverse the Polarity of the Neutron Flow). Function
name mangling doesn’t apply, because closures don’t have function names to
mangle; they’re, instead, identified by the variable that points to them.

You may create as many shadow lambda functions as you like, subject to the
same scoping rules as variables. Once you’ve re-declared an identifier to
point to a different closure, the original variable remains inaccessible until—
or if—the new declaration leaves the active scope.
Shadowing closure definitions in this way isn’t particularly useful—it
illustrates the fact that closures obey variable rather than function shadowing
rules.
Rather than creating a closure and immediately replacing it, you probably
want to select one to run. You can perform this selection at compilation time
with static dispatch, or at runtime with dynamic dispatch.

Static Dispatch
Static dispatch allows your program to make behavioral decisions at compile
time. Two major approaches to implementing static dispatch in Rust are
conditional compilation and constant functions.

Dynamic Dispatch
Dynamic dispatch is a fancy computer-science phrase for “use a match
statement to decide what to do at runtime.”

Features
https://doc.rust-lang.org/cargo/reference/features.html
Const Functions
https://doc.rust-lang.org/reference/const_eval.html#const-functions
Closures
https://doc.rust-lang.org/book/ch13-01-closures.html
*/
