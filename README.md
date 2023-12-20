# learn-rust
Going through the Rust Book

## Ownership

### Stack and Heap

Parts of memory available to you code to use at runtime.

Stack
* last in, first out
* push, pop
* All data stored here must have a known fixed size.

Heap
* you request a certain amount of data
* the memory allocator finds an big enough empty spot
* returns a pointer
* called "allocating on the heap"

Adding and accessing data in the stack is faster than the heap.

### Ownership Rules
* Each value inRust has an *owner*.
* There can only be one owner at a time.
* When the owner goes out of scope, the value will be dropped.

Heap data is *moved*. Stack-only data is copied.

## References and Borrowing

A *reference* is like a pointer, but is guaranteed to point to 
a valid value of a particular type for the lifetime of that
reference.
The action of creating a reference is called *borrowing*.
You cannot modify something you are borrowing.

### Mutable References

Use `&mut` for a mutable reference.

If you have a mutable reference to a value, you can have
no other reference to that value.

### The Rules of References
- At any given time, you can have either one mutable reference or 
any number of immutable references.
- References must always be valid.

