fn main() {
    let x: i32 = 5;
    println!("{}",fib(x));
}

fn fib(num: i32) -> i32{
    let mut x: i32 = 0;
    let mut y: i32 = 1;
    let mut z: i32 = 0;

    if num == 0 {
        return x;
    }

    if num == 1{
        return y;
    }

    for _ in 0..num-2 {
        z = x + y;
        x = y;
        y = z;
    }

    return z;
}

// Code Fibonacii number
// 0 1 1 2 3 5 8 13
// algorithm
    // int x = 0;
    // int y = 0;
    // sum = 0
    // for(int i=2;i<n;i++){
    //    sum = x+y;
    //    x = y;
    //    y = sum
    // }
    // return sum;

// Learning for loops in rust
// mutable numbers in rust