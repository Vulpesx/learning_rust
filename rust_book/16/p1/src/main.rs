use std::thread;
use std::time::Duration;


fn main() {
    }

fn basics() {
    //using threads

    let handle = thread::spawn(|| {
        for i in 1..10 {
        println!("hi number {} from the spawned thread!", i);
        thread::sleep(Duration::from_millis(1));
        }
    });
    
    //forces current thread to wait for handle to finish
    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}

fn moving_data() {
    //moving data to another thread
    let v = vec![1,2,3];
    
    let handle = thread::spawn(move || {
        println!("Here's a vector {:?}", v);
    });

    handle.join().unwrap();

}

