use chrono::{DateTime, Local, TimeZone};

fn main(){
    let now = Local::now();
    println!("now: {}", now);
}

// Package management
// Crates
//  Similar to jars and packages
//  We can bring in externa modules and make our code work