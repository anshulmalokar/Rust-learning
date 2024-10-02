use std::thread;
use std::time::Duration;

fn main()  {
    // thread::spawn(|| {
    //    for i in 1..5{
    //        println!("Running form the spwan thread {}",i);
    //    }
    //    thread::sleep(Duration::new(1,0));
    // });

    // awaiting the thread before running the main thread
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Running form the spwan thread1 {}", i);
            thread::sleep(Duration::new(1,0));
        }
    });

    // This will be doing the await
    handle.join().unwrap();

    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Running form the spwan thread2 {}", i);
            thread::sleep(Duration::new(1,0));
        }
    });

    // This will be doing the await
    handle.join().unwrap();

    for i in 1..5{
        println!("Running from main {}",i);
        thread::sleep(Duration::new(1,0));
    }
}