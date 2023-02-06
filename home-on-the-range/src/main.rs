fn main() {
    .. .. ..;
}

/*
No Output - BUT will compile and run

The .. symbol has a few meanings in Rust:
.. without numbers indicates a range containing every available value.

1..3 creates a range that starts at 1 and stops at 2.

1.. creates a range that starts at 1 and continues forever.

..10 creates a range that includes every number up to but not
including 10.

Used on an indexed type, [..] creates a RangeFull value.

In a match or if let statement, MyOption(field, ..) ignores all other parameters
that you haven’t explicitly named.

The example is accumulating range expressions. No ranges are provided, but
the syntax remains technically valid. The example compiles, but it is very
unlikely that you will ever need it.

Rust has a built-in solution: the todo! macro.

Rust Weird Expressions
https://github.com/rust-lang/rust/blob/master/src/test/ui/weird-exprs.rs
*/
