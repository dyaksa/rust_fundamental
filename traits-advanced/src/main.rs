use calculate_spec::Area; // <---- This is the important line
use calculate_spec::Circumference;

mod calculate_spec;
mod two_dimensional;

fn main() {
    let circle_one = new_circle(10);
    calculate_and_print_area("circle".to_string(), &circle_one);
    calculate_and_print("circle".to_string(), &circle_one);
    calculate_and_println("circle".to_string(), &circle_one);

    let square_one = new_square(15);
    calculate_and_print_area("square".to_string(), &square_one);
    calculate_and_print("square_one".to_string(), &square_one);
    calculate_and_println("square".to_string(), &square_one);
}

fn new_circle(radius: i32) -> impl Area + Circumference {
    let circle = two_dimensional::Circle{radius};
    circle
}

fn new_square(length: i32) -> impl Area + Circumference {
    let square = two_dimensional::Square{length};
    square
}

fn calculate_and_print_area(name: String, item: &(impl Area + Circumference)) {
    println!("Area of {}: {}", name, item.calculate_area());
    println!("Circumference of {}: {}", name, item.calculate_circumference());
}

// Trait bound syntax
fn calculate_and_print<T: Area + Circumference>(name: String, item: &T) {
    println!("Area of {}: {}", name, item.calculate_area());
    println!("Circumference of {}: {}", name, item.calculate_circumference());
}


// Trait bound syntax with where clause
fn calculate_and_println<T>(name: String, item: &T) where T: Area + Circumference {
    println!("Area of {}: {}", name, item.calculate_area());
    println!("Circumference of {}: {}", name, item.calculate_circumference());
}
