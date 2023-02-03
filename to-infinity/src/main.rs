//DON'T RUN THIS AS IT WILL CRASH YOUR SYSTEM!!!!!!!!

use std::cell::RefCell;
use std::rc::Rc;

type Link = Option<Rc<RefCell<Node>>>;

#[derive(Debug)]
struct Node {
    elem: i32,
    next: Link,
}

fn main() {
    let mut head = Some(Rc::new(RefCell::new(Node {
        elem: 1,
        next: None,
    })));
    head.as_mut().unwrap().borrow_mut().next = Some(Rc::new(RefCell::new(Node {
        elem: 2,
        next: head.clone(),
    })));

    println!("{:#?}", head);
}
/*
Output: Prints node1, node2, node1....infinity

Although Rust’s memory model makes it difficult to create linked lists,[20]
you can work around these difficulties by using "Rc" and "RefCell" which are
two low-level
structures that are designed to add flexibility to the borrow-checker. Here’s
what they do:

Rc provides reference counting. When you call get to access an Rc, the
reference count is increased by 1. Likewise, when you drop the
reference, the count is decreased by 1. When an Rc no longer has
references, the contents are deleted. With Rc, you can safely reference
the contained structure from other structures and guarantee that the
contents get deleted when you’re done with them.

RefCell provides a mutable memory location and makes borrow-checking
dynamic rather than static. When you borrow the contents of a RefCell,
Rust notes that the borrow has occurred at runtime rather than compile
time. Although a second mutable borrow can still fail, it will do so as a
runtime error rather than a compile-time error.

Generally, you should avoid creating circular structures because they’re
difficult to iterate safely and delete. Instead, use the Rust standard library’s
std::collections::LinkedList. Rust’s linked list handles node creation and linkage
for you.

Learn Rust with Entirely Too Many Linked Lists
https://rust-unofficial.github.io/too-many-lists/index.html
Reference Counting
https://en.wikipedia.org/wiki/Reference_counting
*/
