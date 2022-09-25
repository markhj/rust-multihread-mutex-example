use std::{
    sync::{
        Arc,
        Mutex,
        MutexGuard,
    },
    thread,
    time::Duration,
};

fn main() {
    // `x` is used to initialize the Mutex
    // You can use other data types than i32, even non-primitives such as structs
    let x : i32 = 0;

    // Set up Mutex
    // Mutex protects data shared between threads
    let mutex : Mutex<i32> = Mutex::new(x);

    // Set up two Arcs
    // The Arc type provides shared ownership of data in the HEAP memory
    // We need an Arc for each thread, because we move the variables into the threads when spawned
    let arc_1 : Arc<Mutex<i32>> = Arc::new(mutex);
    let arc_2 : Arc<Mutex<i32>> = arc_1.clone();

    // Spawn a thread which will increment the shared value
    thread::spawn(move || {
        loop {
            // Obtain the MutexGuard
            // The guard locks from this point on (i.e. unusable in other threads)
            let mut guard : MutexGuard<i32> = arc_1.lock().unwrap();

            // Increment value
            *guard += 1;

            // We must drop the mutex guard when we're done using it,
            // otherwise the guard is occupied here, which halts processing it in the other thread
            drop(guard);

            // Sleep the thread for a bit
            thread::sleep(Duration::from_millis(30));
        }
    });

    // Spawn a second thread to read (and output) the variable
    thread::spawn(move || {
        loop {
            // Note: We use the cloned arc_2, because arc_1 was moved into the first thread
            // Note: In this example the guard is immutable, but if needed, you could
            // also make this one mutable and write to it
            let guard : MutexGuard<i32> = arc_2.lock().unwrap();

            // Show the user what we see
            println!("Thread 2 sees `x` as {}", guard);

            // We need to free the guard, just like in the first thread
            drop(guard);

            // Put thread to sleep, so we don't spam the console
            thread::sleep(Duration::from_millis(500));
        }
    });

    // Keep the main thread alive for X seconds, to see the two
    // child threads processing
    thread::sleep(Duration::from_secs(10));
}
