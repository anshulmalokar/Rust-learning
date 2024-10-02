use std::sync::mpsc;
use std::thread;

fn main(){
    let (tx,re) = mpsc::channel();

    for i in 1..10{
        let producer = tx.clone();
        thread::spawn(move || {
           let mut ans :u64= 0;
           for val in i*1000000..(i+1)*1000000 - 1{
               ans += val;
           }
           producer.send(ans).unwrap();
        });
    }
    drop(tx);
    let mut ans: u64= 0;
    for val in re{
        println!("{}","recieved");
        ans += val;
    }
    println!("{}",ans);
}

// Problem statement
// Write a code that finds the sum from 1 to 10^8
// Use threads to make sure you use all the threads of your machine