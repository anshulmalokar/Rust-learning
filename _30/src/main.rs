fn main(){
    let a = find_bigger(1,2);
    println!("{}",a);
}

fn find_bigger<T: std::cmp::PartialOrd>(a:T,b:T) -> T {
    if a > b{
        return a;
    }
    return b;
}