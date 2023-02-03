fn main() {
    // let life_the_universe = &mut 41;
    // *life_the_universe += 1;
    // println!("Life, the Universe and Everything {}", life_the_universe);

    let mut life = 40;
    let the_universe = &mut life;
    *the_universe += 2;
    println!("{}", the_universe);
}
/*
Output 

 cargo run
   Compiling mutable-immutables v0.1.0 (/Users/jallgood/RustBrainTeasers/mutable-immutables)
    Finished dev [unoptimized + debuginfo] target(s) in 0.20s
     Running `target/debug/mutable-immutables`
Life, the Universe and Everything 42

Notice a few tricks in play here:
You can declare a reference to a literal.[30] When you do, Rust creates a
temporary area of memory containing the desired value, and because the
literal is mutable, you can change it.[31]
The life_the_universe reference itself remains immutable—once you define
the reference, it forever points to the same area of memory, and you
can’t change it.
You can de-reference your immutable reference using the * operator, which
gives you mutable access to its contents.
*/

/*
Part 2 code output

❯ cargo run
   Compiling mutable-immutables v0.1.0 (/Users/jallgood/RustBrainTeasers/mutable-immutables)
    Finished dev [unoptimized + debuginfo] target(s) in 0.15s
     Running `target/debug/mutable-immutables`
42

Making life a mutable variable clearly marks life as a variable that expects
changes. So when you take a mutable reference to life (the_universe), you expect
to be able to change its contents. The mutability and borrow-checking rules
enforced by the Rust compiler are still obeyed:
life is mutable and may change.
the_universe is immutable because it will always point at life once it’s been
set.
De-referencing the_universe allows you to change the contents of life.
Rust’s borrow-checking rules still apply to mutable references. You
can’t mutably borrow the memory area pointed to by life more than once,
can’t reference it once it has left the active scope, and can’t mutably
share it between threads without synchronization primitives.

References and Borrowing
https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html
Mutability (Rust by Example)
https://doc.rust-lang.org/rust-by-example/scope/borrow/mut.html
*/