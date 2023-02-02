fn main() {
    if 0.1 + 0.2 == 0.3 {
        println!("Arithmetic still works!")
    } else {
        println!("Please reboot the universe!")
    }
}

/*
Again this is due to floats and how they compare. Rule of thumb, never compare floats or if so then use
the floor() or ceil() methods. See the "three-and-a-bit" package.

https://github.com/mikedilger/float-cmp
Float Comparison Warning from Clippy
https://rust-lang.github.io/rust-clippy/master/index.html#float_cmp
*/
