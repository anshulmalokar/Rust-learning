fn main() {
    let vec = vec![1,2,3,4,5];

    let iter = vec.iter();

    let itr = iter.filter(|x| *x%2 == 0).map(|x| x*2);
    let vec2: Vec<i32> = itr.collect();
    for val in vec2{
        println!("{}", val);
    }

}

// Write a logic to first filter all the odd values
// then double each value
// and then create a new vector