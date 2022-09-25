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
    // `x` is used to initialize the Mutex, it will remain 0 throughout the program
    let x : i32 = 0;

    // Set up Mutex and two Arcs
    // We need an Arc for each thread, because we instruct the variables to be moved
    // into the threads when spawned
    let mutex : Mutex<i32> = Mutex::new(x);
    let arc_1 : Arc<Mutex<i32>> = Arc::new(mutex);
    let arc_2 : Arc<Mutex<i32>> = arc_1.clone();

    // First, we spawn a thread which will increment the value of the Mutex
    thread::spawn(move || {
        loop {
            // Obtain the MutexGuard
            // The guard is locked from this point on (i.e. unusable in other threads)
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

    // Now, we spawn a second thread to read (and output) the variable
    thread::spawn(move || {
        loop {
            // Note: We use the cloned arc_2, because arc_1 was moved into the first thread
            // Note: In this example the guard is immutable in this thread, but if needed, you could
            // also make this guard mutable and change the value
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
