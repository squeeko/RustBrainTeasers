fn main() {
    let x: u64 = 4_294_967_291;
    let y = x as u32;

    // if x == y {
    if x == y.into() {
        println!("x equals y");
    } else {
        println!(" x does not equal y")
    }

    println!("{:#?} {:#?}", x, y);
}

/*
type-conversion git:(main) ✗ cargo run
   Compiling type-conversion v0.1.0 (/Users/squeeko/RustBrainTeasers/type-conversion)
error[E0308]: mismatched types
 --> src/main.rs:5:13
  |
5 |     if x == y {
  |             ^ expected `u64`, found `u32`
  |
help: you can convert a `u32` to a `u64`
  |
5 |     if x == y.into() {
  |              +++++++

  THis will still fail because the u64 number is out of the range of u32.
    If I lower the value which is in the bounds of u32, the conversion works in this case I lessened the number
    from 4_294_967_296 to 4_294_967_291 and also I used the .into() method. YOu can also use the TryInto trait.


     type-conversion git:(main) ✗ cargo run
   Compiling type-conversion v0.1.0 (/Users/squeeko/RustBrainTeasers/type-conversion)
    Finished dev [unoptimized + debuginfo] target(s) in 0.27s
     Running `target/debug/type-conversion`
x equals y
4294967291 4294967291

Be careful with floating-point to integer conversions because Rust
always rounds down. With that in mind, it’s better to indicate the desired
behavior with my_float.floor to round down, my_float.ceil to round up, or
my_float.round to perform normal numerical rounding. If you want
rounding, perform the rounding before you use as.

For conversions that may be possible, Rust provides another trait: TryInto. The
following code uses try_into to attempt to convert between a u64 and a u32:
use std::convert::TryInto;
let z: u32 = (5000_u64).try_into().expect("Conversion error");
The try_into function returns a Result type. You can access the contents as you
do with other Result types. For example, you can:

unwrap the contents and crash if the conversion failed.

unwrap_or to substitute a default value.

match on the Result to handle the error explicitly.

use expect.

*/
