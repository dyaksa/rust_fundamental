fn main() {
    let some_data = "Dyaksa";
    println!("Hello, world! {}", some_data);

    let some_data = 10;
    println!("number: {}", some_data);

    let some_data = false;
    println!("boolean: {}", some_data);

    let some_data = 3.14;
    println!("float: {}", some_data);

    let name = "Dyaksa Jauharuddin";
    let mut race = "orc";
    let number = 1;

    println!("{}\t {}\t {}", name, race, number);

    {
        let name = "Wiro Sableng";
        race = "human";
        let number = 2;

        println!("{}\t {}\t {}", name, race, number);
    }

    println!("{}\t {}\t {}", name, race, number);
}
