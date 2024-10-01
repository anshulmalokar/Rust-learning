fn main() {
    let vec = vec![1,2,3,4,5];
    let iter = vec.iter();
    let iter2 = vec.iter();
    let iter3 = vec.iter();

    let itr2  = iter.map(|x| x+1);

    let itr3 = iter2.map(|x| x+3);

    let itr4 = iter3.filter(|x| *x%2 == 0);

    for val in itr4{
        println!("{}", val);
    }

    // for val in itr3{
    //     println!("{}", val);
    // }
    //
    // for val in itr2 {
    //     println!("{}", val);
    // }

}
// Iterator adapter
// This is an iterator that will be returning another iterator