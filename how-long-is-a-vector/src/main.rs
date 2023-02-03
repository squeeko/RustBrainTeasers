fn main() {
    let mut my_vec = Vec::with_capacity(1);
    my_vec.push("Hello");
    println!("{}", my_vec.capacity());
    my_vec.push("World");
    println!("{}", my_vec.capacity());

}

/*
Vectors contain two things: a length indicating how many elements (items)
are stored within the vector and a buffer of contiguous heap memory that
contains the data for each of the items, one after the other. This buffer is often
larger than the total number of elements stored within the vector.
Vectors are a lot like arrays, but they have a variable size. You can create a
vector with a capacity of 0 using new, or you can create a vector with a userspecified
capacity size using with_capacity. The capacity represents the total size
of the vector.
When you add an item to a vector, the vector checks to see if the vector’s
length—number of actual items in the vector—is less than the vector’s
capacity. If it is, then adding to the vector is straightforward: the vector’s
length is incremented, and the item is moved to the next available space in
the vector. If there isn’t free space at the end of the buffer, the vector:

1. Allocates a new buffer with enough space for twice the new vector’s
length.

2. Moves the existing buffer to the beginning of the new buffer.

3. Releases the old buffer.

4. Increments length and adds the new item to the end of the new, larger,
buffer.

Rust’s vector growth strategy isn’t guaranteed in the language standard and
may change. At the time of this writing, Rust uses a “growth factor” of 2. In
most cases, the vector will double in size when it needs additional capacity. If
you are sequentially adding many items to a vector, this operation can cause a
lot of memory reallocation, potentially causing performance problems.
Vector Tips
When working with vectors, keep a few things in mind:
If you have a rough idea of how much data you might need to store, use
Vec::with_capacity to reserve an appropriate amount of space ahead of time
—doing so avoids memory reallocations altogether.
If you’re adding lots of data, try using Vec::extend so that Rust can see the
size of the data you’re adding and reallocate only once. extend only
avoids allocation when collecting data from a source with a known
length. Copying from one vector to another allows Rust to allocate
exactly the space it needs for the new vector because the length is
known. Likewise, any iterator that implements ExactSizeIterator benefits
from this optimization. An arbitrarily sized iterator may repeatedly
allocate because the size of the data you’re copying isn’t known ahead
of time.
Add elements to the end of your vectors using push rather than at a
specific slot using insert. Although insert gives you more control, it’s a lot
slower than push because Rust needs to rearrange the vector to make
room for your new element. If you need to insert an element at the front,
the VecDeque structure is a better choice.
Further Reading
std::vec::Vec
https://doc.rust-lang.org/std/vec/struct.Vec.html
*/
