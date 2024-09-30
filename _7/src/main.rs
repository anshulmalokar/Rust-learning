enum Shape{
    Rectangle(f64,f64),
    Circle(f64),
    Square(f64)
}

fn main() {
    let my_direction = Direction::NORTH;
    let circle = Shape::Circle(10.0);
    let rect = Shape::Rectangle(10.0,10.0);
    let square = Shape::Square(10.0);
    my_direction_fun(my_direction);
}

fn my_direction_fun(dir: Direction){
    print!("{}", "Hello");
}

// Pattern match on these variables
fn calculateArea(shape: Shape) -> f64{
    return match shape{
        Shape::Rectangle(w,h) => w*h,
        Shape::Circle(r) => 3.14*r,
        Shape::Square(s) => s*s
    };
}

enum Direction{
    NORTH,
    SOUTH,
    EAST,
    WEST
}

// Learning enums in rust
// Enums are used to define a set of named values. They are useful when you have a variable
// How to access value 
// Extract values using pattern matching