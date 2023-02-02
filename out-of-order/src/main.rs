fn main() {
    let mut floats = vec![3.1, 1.2, 4.5, 0.3];
    // floats.sort(); See the fix below by using the "partial_cmp()" function.

    floats.sort_by(|a, b| a.partial_cmp(b).unwrap());
    println!("{:#?}", floats);
}

/*
The code yields the error below becuase it needs to sort first and "floats" are not always seen as numbers

error[E0277]: the trait bound `{float}: Ord` is not satisfied
   --> src/main.rs:3:5
    |
3   |     floats.sort();
    |     ^^^^^^ ---- required by a bound introduced by this call
    |     |
    |     the trait `Ord` is not implemented for `{float}`
    |
    = help: the following other types implement trait `Ord`:
              i128
              i16
              i32
              i64
              i8
              isize
              u128
              u16
            and 4 others
note: required by a bound in `slice::<impl [T]>::sort`
   --> /Users/squeeko/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/src/rust/library/alloc/src/slice.rs:204:12
    |
204 |         T: Ord,
    |            ^^^ required by this bound in `slice::<impl [T]>::sort`

For more information about this error, try `rustc --explain E0277`.
error: could not compile `out-of-order` due to previous error
➜  out-of-order git:(main) ✗


Rust makes it easy to sort vectors of most types. A vector’s sort function can
sort a vector of strings alphabetically or a vector of integers numerically
without issue. So why doesn’t sorting a vector of floating-point numbers
work? Since floating-point numbers aren’t always numbers (more on that in a
minute), they aren’t always naturally sortable. Rust generally makes sorting
values easy, but it’s also careful to avoid implicit behavior that can surprise
the programmer.

Rust introduced the PartialOrd and PartialEq traits—and the accompanying
partial_cmp function—to represent numeric types that are generally comparable
but may feature cases in which two numbers cannot be naturally compared or
ordered.

The partial_cmp function (provided via the PartialOrd trait) returns an Option. You
can access the contained ordering information by calling unwrap on the result.
Unwrapping an empty option will crash your program if the value could not
be ordered—because an INFINITY or NaN snuck into your data.
If you’re sure that you won’t be dealing with invalid floating-point numbers,
you can simply unwrap the results of partial_cmp and use that to sort your data in
the sort_by function.
If your code might encounter an invalid value, you can use unwrap_or to
provide a default sort order for invalid numbers:

use std::cmp::Ordering::Less;

let mut floats = vec![
3.1, 1.2, 4.5, 0.3, std::f32::INFINITY, std::f32::NAN
];

floats.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Less));

The even longer sort_by call works and is safe with invalid numbers, but it’s a
lot of code to type whenever you need to sort a slice of floating-point
numbers. To help solve this problem, here’s a handy function you can keep in
your toolbox to simplify this process:

fn float_sort<T : PartialOrd>(data: &mut [T]) {
use std::cmp::Ordering::Less;
data.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Less));
}

You can then use the float_sort function to safely sort any slice (collection of
numbers, typically the contents of an array or vector) of floating-point
numbers:

let mut floats = vec![
3.1, 1.2, 4.5, 0.3, std::f32::INFINITY, std::f32::NAN
];
float_sort(&mut floats);

Ord trait
https://doc.rust-lang.org/std/cmp/trait.Ord.html
PartialOrd trait
https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html
f32 primitive
https://doc.rust-lang.org/std/primitive.f32.html
*/
