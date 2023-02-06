#[derive(Debug)]
struct Parser<'a> {
    body: String,
    subtext: &'a str,
}
/*
THIS WILL NOT COMPILE!!!!!
fn main() {
    let mut document = Parser {
        body: "Hello".to_string(),
        subtext: "",
    };
    document.subtext = &document.body;

    let b = document;
    println!("{:#?}", b);
}
*/

// See fix here - placing the self referential structure in it's own scope
fn main() {
    {
        let mut document = Parser {
            body: "Hello".to_string(),
            subtext: "",
        };
        document.subtext = &document.body;
    }
}

/*


Why Use Self-Referential Structures?
If your structure holds a large amount of data, storing references to part of the
stored data can be very useful. For example, a parser might need to store
references to parts of the source code it’s reading. Here are a few suggestions
to help you when you need to do this:

Consider extracting self-referential systems into separate structures that
refer to the parent variable. This solution makes deletion of the objects
explicit: you can safely drop the child object at any time, but Rust’s
lifetime protection still guarantees that the parent object must outlive its
children.

Limit the use of self-referencing objects to short-lived scopes that can be
safely deleted.

If you’re referencing index data, you could store the index to which you
are referring rather than a direct reference/pointer to the referenced data.

If all else fails, use reference counting (Rc) and weak pointers to
untangle your data.[35]

Rust’s borrow-checking and lifetime checking features can sometimes add a
little complexity to your code. It’s a trade-off: on one hand, it’s very difficult
to create dangerous code. On the other, sometimes you’d like to perform a
safe operation but have to jump through a few extra hoops to prove that your
operation is safe to Rust.

Validating References with Lifetimes
https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html
References and Borrowing
http://web.mit.edu/rustlang_
v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/firstedition/
references-and-borrowing.html
*/
