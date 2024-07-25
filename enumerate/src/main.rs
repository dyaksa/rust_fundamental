mod contants;

#[allow(dead_code)]
#[derive(Debug)]
enum Food {
    Apple,
    Carrot,
    Potato,
    OtherFood(String),
    MieSatan{level: i32, is_bowl: bool},
}
fn main() {
    // let nasi_goreng: String = String::from("Nasi Goreng");
    let makanan_favorite: Food = Food::MieSatan {level: 5, is_bowl: true};

    match  makanan_favorite {
        Food::Apple => println!("Apple is my favorite"),
        Food::Carrot => println!("Carrot is my favorite"),
        Food::Potato => println!("Potato is my favorite"),
        Food::OtherFood(n) => println!("{} is my favorite", n),
        Food::MieSatan { level, is_bowl } => {
            if is_bowl {
                println!("Mie Satan level {} is my favorite with bowl", level);
            } else {
                println!("Mie Satan level {} is my favorite withour bowl", level);
            }
        },
        _ => println!("I don't have favorite food"),
        
    }
    println!("Hello, world!");

    let company: contants::Company = contants::Company::Apple;

    match company {
        contants::Company::Apple => println!("Apple company"),
        _ => println!("Other company"),
    }
}
