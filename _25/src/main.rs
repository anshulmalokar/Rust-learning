fn main() {
    let vec = vec![1,2,3,4,5,6,7,8,9];
    let itr = vec.iter();
    let sum = itr.sum::<i32>();

    // This will not be working as the whole vector is consumed previously
    for val in itr{
        println!("{}",val);
    }

    println!("Sum: {}", sum);
}

// Consuming adapters
// Methods that call next are called consuming adapters
// Because calling next uses up the iterator
