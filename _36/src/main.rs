use std::thread;
use std::time::Duration;

fn main() {
    println!("Hello, world!");
    let v = vec![1,2,3];
    let handle = thread::spawn(move || {
        //Do something with the vector
        let itr = v.iter();
        for val in itr{
            println!("{}", val);
            thread::sleep(Duration::from_secs(1));
        };
    });

    handle.join().unwrap();

    for i in 1..10{
        println!("{}",i);
        thread::sleep(Duration::from_millis(1000));
    }
}