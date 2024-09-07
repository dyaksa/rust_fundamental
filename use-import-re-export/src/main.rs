use std::env::{current_dir, args};
mod mesagging;

fn main() {
    let package_path = current_dir().unwrap();
    println!("package name {:?}", package_path);

    for i in 1..=args().len() {
        let each_arg = args().nth(i);
        if each_arg != None {
            println!("arg {} is {:?}", i, each_arg.unwrap());
        }
    }

    mesagging::say_hello_world();
}
