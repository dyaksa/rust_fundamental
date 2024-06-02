fn main() {
    println!("Hello, world!");
    greet();
    greet_custom_message("Dyaksa", "Bagaimana Kabarmu?");
    let volume = calculate_box_volume(40, 30, 10);
    println!("Volume of the box is: {0}", volume);
    let volume1 = calculate_box_volume1(40, 30, 10);
    println!("Volume of the box is: {0}", volume1);
    let volume2 = calculate_box_volume_2(1, 4, 5);
    let message2 = format!("Volume of the box is: {0}", volume2);
    greet_custom_message("Dyaksa", message2.as_str());

    println!("Score Message: {0}", get_score_message(90.0));
    println!("Score Message: {0}", get_score_message(70.0));
    println!("Score Message: {0}", get_score_message(50.0));

    let gcm = greet_custom_message("Dyaksa", "Apa Kabar?");
    println!("greet custom message: {:?}", gcm);
}

fn greet(){
    println!("Hello How Are you!");
}

fn greet_custom_message(name: &str, message: &str) {
    println!("Hi, {0}! {1}", name, message);
}

fn calculate_box_volume(width: u32, height: u32, length: u32) -> u32 {
    let result = width * height * length;
    return result;
}

fn calculate_box_volume1(width: u32, height: u32, length: u32) -> u32 {
    let volume = width * height * length;
    volume
}

fn calculate_box_volume_2(width: u32, height: u32, length: u32) -> u32 {
    width * height * length
}

/* conditional return value */
fn get_score_message(score: f32) -> &'static str {
    if score > 80.0 {
        return "Great Job!";
    } 
    
    if score > 60.0 {
        return "Not Bad!";
    }

    "You Can Do Better!"
}
