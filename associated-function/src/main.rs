use lego::LegoSet;

mod lego;
mod model;

fn main() {
    let extreme_offroader = lego::LegoSet::new(
        40299, 
        String::from("4X4 X-treme Off-Roader"), 
        String::from("offroad"), 
        25
    );

    println!("{:?}", LegoSet::what_is_lego());

    println!("{:#?}", extreme_offroader);
    println!("The Lego set code {} {} is a {} set for ages {} and up.", extreme_offroader.code, extreme_offroader.name, extreme_offroader.category, extreme_offroader.age_minimum);
    lego::LegoSet::what_is_lego();

    let red = model::Color::red();
    let green = model::Color::green();
    let blue = model::Color::blue();

    println!("{:#?}", red);
    println!("{:#?}", green);
    println!("{:#?}", blue);

    let random_color = model::Color(100, 200, 50);
    println!("{:#?}", random_color);
}
