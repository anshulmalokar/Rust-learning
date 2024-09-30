fn main() {
    let mut s1 = String::from("Hello No Change");
    s1 = do_something(s1);
    println!("{}", s1);
}

fn do_something(mut s2: String) -> String{
    s2 = String::from("a");
    return s2;
}