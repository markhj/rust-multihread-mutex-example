This is a quick example of multi-threading in Rust, where two threads read from, and write to, shared data allocated in the HEAP memory.

The example also shows how ``Arc`` (Atomically Reference Counted) is used to share ownership, and ``Mutex`` guards/protects shared data.

## Trying it out
1) Clone this repository
2) Open it in your Rust IDE of choice
3) Build the code

The program will run for about 10 seconds.
You can alter this behavior by changing the ``thread::sleep`` executed at the end of the main program.

## Prerequisites
No additional Crates required.

Written in ``Rust 1.63``.

## Demonstrated principles
- Multi-threading
- Shared ownership of data with ``Arc``
- Guarding shared data with ``Mutex``
- The importance of freeing up unused memory (occupying the ``MutexGuard`` for too long in one thread halts the other)
