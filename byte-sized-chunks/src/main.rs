fn main() {
    let mut counter: i8 = 0;

    loop {
        println!("{}", counter);
        counter += 1;
    }
}

/*
...
117
118
119
120
121
122
123
124
125
126
127
thread 'main' panicked at 'attempt to add with overflow', src/main.rs:6:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

if I run this in cargo run it will produce the error above, if I run in cargo run --release mode, the numbers will loop around
from 0 to 127, then -128 to -1

std::num::Wrapping
https://doc.rust-lang.org/std/num/struct.Wrapping.html
Two’s Complement
https://en.wikipedia.org/wiki/Two%27s_complement
Cargo Profiles
https://doc.rust-lang.org/cargo/reference/profiles.html
Rust Data Types
https://doc.rust-lang.org/book/ch03-02-data-types.html
*/
