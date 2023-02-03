use std::mem::size_of;

// Returns the proper amount of bytes without Rust structure optimizations
#[repr(packed)]
struct VeryImportantMessage {
    _message_type: u8,
    _destination: u16,
}

fn main() {
    println!(
        "VeryImportantMessage occupies {} bytes",
        size_of::<VeryImportantMessage>()
    );
}

/*

WITHOUT Rust structure optimizations returning the proper byte structure
structure-sizing git:(main) ✗ cargo run
   Compiling structure-sizing v0.1.0 (/Users/squeeko/RustBrainTeasers/structure-sizing)
    Finished dev [unoptimized + debuginfo] target(s) in 0.32s
     Running `target/debug/structure-sizing`
VeryImportantMessage occupies 3 bytes

WITH Rust structure optimizations
structure-sizing git:(main) ✗ cargo run
   Compiling structure-sizing v0.1.0 (/Users/squeeko/RustBrainTeasers/structure-sizing)
    Finished dev [unoptimized + debuginfo] target(s) in 0.50s
     Running `target/debug/structure-sizing`
VeryImportantMessage occupies 4 bytes

By default, Rust makes two promises about the in-memory representation of
your structures:
Structures may be sized differently than their contents for performance
reasons.
Structures may store data in a different order internally than you
specified if the optimizer believes it will aid performance.
Most modern CPUs align data on 32-bit boundaries in memory and cache.
Accessing 8 bits (one byte) or 16 bits (two bytes) is fast because the CPU
provides primitives to do so, and the structures can be packed along 32-bit
boundaries.
A 24-bit (3 byte) structure doesn’t naturally align to a 32-bit memory map, so
by default, Rust wastes 8 bits of memory per struct to ensure fast access to the
structure in your computer’s memory. This behavior is especially helpful
when you’re dealing with arrays or other contiguous blocks of 3-byte
structures because every other structure would start at the 24th bit of a 32-bit
block, reducing cache and read efficiency.

The Fix: You can turn off both of Rust’s structure optimizations using a decoration
named #[repr()], which gives you control over how a struct is represented in
memory:

#[repr(C)] declares that you require interoperability with the C language.
Rust won’t rearrange the content of your structure.

#[repr(packed)] tells Rust not to waste space on your structure. This can
carry a small performance penalty but guarantees that structures are
exactly the right size.
You can combine these decorations.
*/
