// fn double_it(n: i32) -> i32 {
//     n * 2
// }

// fn double_it(n: f32) -> f32 {
//     n * 2.0
// }


fn double_it<T>(n: T) -> T where T: std::ops::Mul<Output = T> + From<i32> {
    n * 2.into()
}


fn main() {
    println!("10 * 2 = {:#?}", double_it(10));
}
/*

Error output without Generics

cargo run
   Compiling double-or-nothing v0.1.0 (/Users/jallgood/RustBrainTeasers/double-or-nothing)
error[E0428]: the name `double_it` is defined multiple times
 --> src/main.rs:5:1
  |
1 | fn double_it(n: i32) -> i32 {
  | --------------------------- previous definition of the value `double_it` here
...
5 | fn double_it(n: f32) -> f32 {
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^ `double_it` redefined here
  |
  = note: `double_it` must be defined only once in the value namespace of this module

For more information about this error, try `rustc --explain E0428`.
error: could not compile `double-or-nothing` due to previous error

Error WITH Generics NOT being specific enough!

cargo run
   Compiling double-or-nothing v0.1.0 (/Users/jallgood/RustBrainTeasers/double-or-nothing)
error[E0369]: cannot multiply `T` by `{integer}`
  --> src/main.rs:11:7
   |
11 |     n * 2
   |     - ^ - {integer}
   |     |
   |     T
   |
help: consider restricting type parameter `T`
   |
10 | fn double_it<T: std::ops::Mul<i32, Output = T>>(n: T) -> T {
   |               ++++++++++++++++++++++++++++++++

Again with not enough specificity

cargo run
   Compiling double-or-nothing v0.1.0 (/Users/jallgood/RustBrainTeasers/double-or-nothing)
error[E0308]: mismatched types
  --> src/main.rs:11:9
   |
10 | fn double_it<T>(n: T) -> T where T: std::ops::Mul<Output = T> + From<i32> {
   |              - this type parameter
11 |     n * 2
   |         ^ expected type parameter `T`, found integer
   |
   = note: expected type parameter `T`
                        found type `{integer}`
help: call `Into::into` on this expression to convert `{integer}` into `T`
   |
11 |     n * 2.into()
   |          +++++++

FINALLY successful output either in i32, f32, f64...etc


T is added as a generic type to the function signature, and the number to
double is defined as requiring type T.
Rust generics constrain the types with which a generic function can
operate using the where keyword. You can separate requirements with plus
symbols.

The first constraint requires that T implement std::ops::Mul, meaning the type
must support the addition operator, and the output of the multiplication
must be of type T.

You add a second constraint with + From <i32>, requiring that the input type
be constructable from an integer. If you didn’t require this constraint, * 2
would fail to compile because Rust could not guarantee that the digit 2
could be converted into a type compatible with the function parameter’s
type.

Within the function body, you’ve established that the n parameter must
support multiplication and be compatible with the integer 2. You can
perform the multiplication using normal Rust operators.

Generics (Rust by Example)
https://doc.rust-lang.org/rust-by-example/generics.html
Rust Generics
https://doc.rust-lang.org/book/ch10-01-syntax.html
Learn Rust—Generics
https://learning-rust.github.io/docs/b4.generics.html
The Rust Programming Language—Generics
http://web.mit.edu/rustlang_
v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/firstedition/
generics.html
*/
