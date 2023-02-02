fn main() {
    const THREE_AND_A_BIT: f32 = 3.4028236;
    println!("{}", THREE_AND_A_BIT);
}

/*
Guess the outcome: (f32) 3.4028237, (f64) 3.4028236.  - This difference has to do with how Rust represents 32-bit floating point
numbers (the f32 type). Rust—like many other languages—represents
floating-point numbers using the IEEE-754 standard. https://www.tutorialspoint.com/explain-the-ieee-standard-754-floating-point-numbers-in-computer-architecture
Sometimes you can solve floating-point precision errors by using
a larger floating-point type. You can represent 3.4028236 with an
f64 that does NOT round up due to the precision, some numbers cannot be represented properly. This is called a rounding error.

three-and-a-bit git:(main) ✗ cargo run
   Compiling three-and-a-bit v0.1.0 (/Users/squeeko/RustBrainTeasers/three-and-a-bit)
    Finished dev [unoptimized + debuginfo] target(s) in 0.64s
     Running `target/debug/three-and-a-bit`
3.4028237 (f32)
➜  three-and-a-bit git:(main) ✗ cargo run
   Compiling three-and-a-bit v0.1.0 (/Users/squeeko/RustBrainTeasers/three-and-a-bit)
    Finished dev [unoptimized + debuginfo] target(s) in 0.25s
     Running `target/debug/three-and-a-bit`
3.4028236 (f64)
*/
