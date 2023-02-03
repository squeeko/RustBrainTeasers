fn main() {
    loop {
        let buffer = (0..1000).collect::<Vec<u32>>();
        std::mem::forget(buffer);
        print!(".");
    }
}
/*
......... forever!
https://doc.rust-lang.org/std/mem/fn.forget.html - basically this is a memory leak. Rust will NOT run the destructor and
forgets the reference to the area of memory the forgotten variable is pointing to. Interestingly enough here's a quote from
Huon Wilson (Rust Core Team Alumni) concerning memroy leaks.

"Put simply: memory unsafety is doing something with invalid data, a
memory leak is not doing something with valid data.
The Rust documentation adds to this statement:
forget is not marked as unsafe, because Rust’s safety guarantees do not
include a guarantee that destructors will always run. For example, a
program can create a reference cycle using Rc, or call process::exit to exit
without running destructors. Thus, allowing mem::forget from safe code
does not fundamentally change Rust’s safety guarantees."

std::mem::Forget
https://doc.rust-lang.org/std/mem/fn.forget.html
std::boxed::Box
https://doc.rust-lang.org/std/boxed/struct.Box.html
*/
