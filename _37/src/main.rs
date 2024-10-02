use std::sync::mpsc;
use std::thread;

fn main() {
     let (tx,rc) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let val = rc.recv().unwrap();
    println!("{}",val);
}



// Message passing in rust
// Find the sum from 1 to 10^9
// Delegate the task to multiple threads and eventually combine the result
// Rust have channels
// Channels will be helping in sharing data from one thread to another
// Channel is a good programming concept to pass

// Use case
// One thread reading the data from Redis
// Another thread processing this data


// Channel
// Transmitter
    // this part of the code will be sending the data
// Reciever
    // this part of the code will be receiving the data
//