// use std::io::stdin;
// use std::io::{stdin, repeat};
use std::io::*;

fn main() {
    println!("enter message : ");

    let mut message = String::new();

    let stdin_reader = stdin();

    let reader_res = stdin_reader.read_line(&mut message);

    if reader_res.is_err() {
        println!("error message : {:?}", reader_res.err());
        return;
    }

    println!("message : {}", message);
}
