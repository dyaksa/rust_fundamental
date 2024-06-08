mod models;

fn main() {
   let game_one = models::game::GameConsole {
    name: String::from("Valorant"),
   };

   println!("Game: {:#?}", game_one);

   let color = models::color::Color(255, 0, 0);
    println!("Color: {:#?}", color);
}
