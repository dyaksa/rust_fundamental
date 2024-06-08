use std::vec;

struct User {
    name: String,
    sign_in_count: u64,
    affiliation: Vec<String>,
    active: bool
}

struct Car {
    brand: String,
    year: u64,
    price: u64,
}

struct Point {
    x: u64,
    y: u64
}

#[derive(Debug)]
struct Game {
    name: String,
}

struct Color(i32, i32, i32);

fn main() {
    let color_one = Color(255,0,0);
    println!("r: {}, g: {}, b: {}", color_one.0, color_one.1, color_one.2);

    let user_one = User {
        name: String::from("Dyaksa Jauharuddin"),
        sign_in_count: 12,
        affiliation: vec![
            String::from("winandi"),
            String::from("Nando"),
        ],
        active: true,
    };

    println!("name {}", user_one.name);
    println!("sign_in_count {}", user_one.sign_in_count);
    println!("affiliation {:?}", user_one.affiliation);
    println!("active {}", user_one.active);

    let mut user_two = User{
        name: String::from("Anggiani"),
        sign_in_count: 11,
        affiliation: vec![
            String::from("Shopee"),
            String::from("Tokopedia")
        ],
        active: false,
    };

    user_two.name = String::from("value changed");
    user_two.affiliation.pop();
    user_two.active = true;

    println!("name {}", user_two.name);
    println!("sign_in_count {}", user_two.sign_in_count);
    println!("affiliation {:?}", user_two.affiliation);
    println!("active {}", user_two.active);

    let car_one: Car;
    car_one = Car {
        brand: String::from("Toyota"),
        year: 2021,
        price: 200000000,
    };

    println!("brand {}", car_one.brand);
    println!("year {}", car_one.year);
    println!("price {}", car_one.price);

    let car_two: Car = Car {
        brand: String::from("Honda"),
        ..car_one
    };
    println!("brand {}", car_two.brand);
    println!("year {}", car_two.year);
    println!("price {}", car_two.price);

    let brand: String = String::from("Audi");
    let year: u64 = 2021;
    let price: u64 = 300000000;
    let car_three = new_car(brand, year, price);

    println!("brand {}", car_three.brand);
    println!("year {}", car_three.year);
    println!("price {}", car_three.price);

    let point_one = Point{x: 10, y: 20};

    let Point{x: _, y: y1} = point_one;
    println!("y1 {}", y1);

    let game: Game = Game {
        name: String::from("Valorant"),
    };
    println!("{:#?}", game);
}

fn new_car(brand: String, year: u64, price: u64) -> Car {
    Car {
        brand,
        year,
        price,
    }
}
