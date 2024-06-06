pub mod conversion_utility;

#[path ="conversion.rs"]
pub mod conversion_util;


pub fn is_odd_number(number: i32) -> bool {
    number % 2 == 1
}