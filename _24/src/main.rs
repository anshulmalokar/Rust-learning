fn main(){
    let mut v1 = vec![1,2,3];

    let itr = v1.into_iter();

    for val in itr{
        println!("{}",val);
    }
    // This cannot be done
    // The iter will take the ownership from the vector
    println!("{:?}",v1);
    println!("Hello, world!");
}
