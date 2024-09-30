// Write a function that will be taking a vector as an input and returns vector with even values.

fn main(){
    let c = vec![1,2,3];
    let mut a: Vec<i32> = Vec::new();
    a.push(1);
    a.push(2);
    a.push(3);
    a.push(4);
    let b = even_value_vector(&a);
    println!("{:?}", b);
    println!("{:?}", c);
}

fn even_value_vector(a: &Vec<i32>) -> Vec<i32>{
    let mut b: Vec<i32> = Vec::new();
    for i in 0..a.len(){
        if(a[i]%2 == 0){
            b.push(a[i]);
        }
    }
    return b;
}