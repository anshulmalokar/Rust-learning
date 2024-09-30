fn main() {

    let my_string = String::from("anshul");
    let index: Option<i32> = find_first_a(my_string);
    match index{
        Some(index) => println!("{}", index),
        None => println!("No 'a' found"),
    }
}

fn find_first_a(str: String) -> Option<i32>{
    for (index,character)  in str.chars().enumerate(){
        if character == 'a' {
            return Some(index as i32);
        }
    }
    return None;
}


fn fund_first_word(str: String) -> Option<String>{

}

struct Entity{
    name: String,
    age: i32,
}

impl Entity{
    fn new(name: String, age: i32) -> Entity{}
    fn find(&self,name: String) -> Option<Entity>{

    }
}
// match ans{
//      Some(entity) => ,
//      None
// }

fn fun() -> Option<Entity>{

}

// enum Option<T>{
//     None,
//     Some(T)
// }
// Learning
// Two important enums
// null values and error handeling in rust
// Error handeling is done using enums in rust
// Options
    // none/null/nil values can be returned
    // error
// similar code in typescript