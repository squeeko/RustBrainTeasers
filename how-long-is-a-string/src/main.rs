const HELLO_WORLD: &'static str = "Hallò heimur";
fn main() {
    println!("{} is {} characters long", HELLO_WORLD, HELLO_WORLD.len());
    println!(
        "Proper length using the chars() and count() methods {} chars ",
        HELLO_WORLD.chars().count(),
    )

    // To represent the proper len(), we can use the .count() and .chars() methods
}

/*
pub struct String {
vec: Vec<u8>,
}
Strings are just a vector of bytes (u8), representing Unicode characters in an
encoding named UTF-8. Rust automatically translates your string to UTF-8.

When accessing individual characters in a string, it’s much safer to use
chars as opposed to directly accessing the byte array. Characters are
aware of Unicode boundaries—bytes are not. Printing the first 6 bytes of
“Können” will only print “Könne”. Printing the first 6 characters will
output the entire word.

Char
https://doc.rust-lang.org/std/primitive.char.html
String length
https://doc.rust-lang.org/std/string/struct.String.html#method.len
Unicode Symbol Reference
https://www.compart.com/en/unicode/
Wikipedia UTF-8
https://en.wikipedia.org/wiki/UTF-8
String Source Code
https://doc.rust-lang.org/src/alloc/string.rs.html

*/
