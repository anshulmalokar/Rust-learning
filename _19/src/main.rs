use std::collections::HashMap;
// Write a function that will be taking 
// a vector of tupels
// and returns a HashMap where the keys are the unique keys
// and values are all the corresponding values accociated with each key

fn main(){
    let tupels = vec![
        [String::from("anshul"),1],
        [String::from("anshul1",2)],
        [String::from("anshul2"),3],
    ];
    let map = groups_values_by_key(tupels);
    println!("{}",map);
}

fn groups_values_by_key(pairs: Vec<(String,i32)>) -> HashMap<String,i32>{
    let map = HashMap::new();
    for val in pairs{
        map.insert(val[0],val[1]);
    }
    return map;
}