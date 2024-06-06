mod my_io;
mod my_number;

use my_number::conversion_utility::string_to_number;
use my_number::is_odd_number;
use my_number::conversion_util::print_end;

fn main() {
    println!("enter any number : ");
    let message = my_io::read_entry();
    println!("you entered: {}", message);

    let number = string_to_number(message);
    let result = is_odd_number(number);
    println!("is odd number : {}", result);
    println!("{}", print_end());
}
