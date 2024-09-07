use std::time::{SystemTime, UNIX_EPOCH};

type Inch = u64;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

type Coordinate = Point;

fn main() {
    let height: Inch = 10;
    println!("height: {height}");

    let height_in_u64: u64 = height as u64;
    println!("height_in_u64: {height_in_u64}");

    let p: Point = Point {x: 10, y: 20};
    println!("p: {:?}", p);

    let mut  q: Coordinate = p as Coordinate;
    q.x = 30;
    println!("q: {:?}", q);

    let mut r: Point = q as Point;
    r.y = 40;
    println!("r: {:?}", r);

    let number: i32 = 20;
    println!("number: {number}");

    let number_in_f64: f64 = number as f64;
    println!("number_in_f64: {number_in_f64}");

    let number_in_u64: u64 = number as u64;
    println!("number_in_u64: {number_in_u64}");

    let new_number = 2.45 as f64;
    println!("new_number: {new_number}");

    let timestamp: u64 = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

    println!("timestamp: {timestamp}");
    println!("timestamp as u16 {}", timestamp as u16);
    println!("timedtamp u16 as u64 {:?}", (timestamp as u16) as u64);
    
}
