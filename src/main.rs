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
    // Important: `x` will not be affected by the arc and mutex variables
    // It will remain 0 throughout this program
    let x : i32 = 0;

    // Set up Mutex, and two Arcs to be used in each thread
    let mutex : Mutex<i32> = Mutex::new(x);
    let arc_1 : Arc<Mutex<i32>> = Arc::new(mutex);
    let arc_2 : Arc<Mutex<i32>> = arc_1.clone();

    // First, we spawn a thread which will increment the value of the
    // mutex variable
    thread::spawn(move || {
        loop {
            let mut guard : MutexGuard<i32> = arc_1.lock().unwrap();

            // Increment value
            *guard += 1;

            // It's crucial to drop the mutex guard when we're done using it,
            // otherwise the guard is occupied in this thread, which will in turn
            // halt processing it in the other thread
            drop(guard);

            // Sleep the thread for a bit
            thread::sleep(Duration::from_millis(30));
        }
    });

    // Second, we spawn a thread to read (and output) the variable
    thread::spawn(move ||{
        loop {
            // Notice: We use the cloned arc_2, because arc_1 was moved into the first thread
            // In this example the guard is immutable in this thread, but if needed, you could
            // also make this guard mutable and change the value
            let guard : MutexGuard<i32> = arc_2.lock().unwrap();

            // Show the user what we see
            println!("Thread 2 sees `x` as {}", guard);

            // Like in the first thread, we need to free the guard
            drop(guard);

            // Sleep thread, so we don't spam the console
            thread::sleep(Duration::from_millis(500));
        }
    });

    // Keep the main thread alive for X seconds, to see the two
    // child threads processing
    thread::sleep(Duration::from_secs(10));
}
