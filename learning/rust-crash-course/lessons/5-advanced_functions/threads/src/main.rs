use std::thread;

fn main() {
    // Executed as main function of the thread! 
    let handle = thread::spawn(move || {
        // Do stuff!
    });
    
    //Do stuff simultaneously in the main thread

    // wait until thread has exited
    handle.join().unwrap();
}
