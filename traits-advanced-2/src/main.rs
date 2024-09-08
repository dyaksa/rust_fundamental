mod shape;
mod circle;
mod square;

use crate::shape::Shape;

fn main() {
   let circle = circle::Circle{radius: 3.0};
   println!("Circle area: {}", circle.area());

    let square = square::Square{side: 4};
    println!("Square area: {}", square.area());
}
