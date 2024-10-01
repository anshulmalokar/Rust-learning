use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(String::from("hello"), 10);
    map.insert(String::from("world"), 15);
    map.insert(String::from("universe"), 20);

    let iter = map.iter();

    for (k,v) in iter{
        println!("{}: {}", k, v);
    }

}
 // Iterators in hashmap