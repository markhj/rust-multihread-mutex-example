This is a quick example of multi-threading in Rust, where Mutex and Arc types are used to
read and write to the same memory.

## Trying it out
Clone this repository, open it in your Rust IDE of choice, and build the code.

The program will run for about 10 seconds, but you can alter this behavior by changing the ``thread::sleep`` executed
at the end of the main program.

## Prerequisites
No additional Crates required.

Written in ``Rust 1.63``.

## Demonstrated principles
- Multi-threading
- Mutex / Arc
- Ownership and borrowing in "real world" usage
- The importance of freeing up unused memory (occupying the ``MutexGuard`` for too long in one thread halts the other)
