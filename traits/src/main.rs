struct Circle {
    radius: f64
}

impl std::fmt::Debug for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Circle radius is {}", self.radius)
    }
} 

impl std::fmt::Display for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Circle radius is {}", self.radius)
    }
}
fn main() {
   let number = 12;
   println!("Number is {}", number);

   let txt = String::from("hello rust");
    println!("Text is {}", txt);

    let circler = Circle { radius: 12.0 };
    println!("Circle radius is {:?}", circler);

    println!("Circle radius is {}", circler);
}
