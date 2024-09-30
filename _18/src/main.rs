use std::collections::HashMap;
fn main() {
    let mut users:HashMap<String,i32> = HashMap::new();
    users.insert(String::from("Hello"),1);
    users.insert(String::from("Hello1"),1);
    users.insert(String::from("Hello2"),1);
    users.insert(String::from("Hello3"),1);
    users.insert(String::from("Hello4"),1);
    let val = users.get("Hello");
    match val {
        Some(val) => println!("{}", val),
        None => {}  
    }
    let val2 = users.get("Hello1");
    match val2{
        Some(val) => {
            println!("{}", val);
        },
        None => {}
    }
}
// HashMap in rust