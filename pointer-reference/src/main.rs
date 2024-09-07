use rand::Rng;

fn main() {
    let mut number: i32 = 10;
    println!("Number is {:?}", number);

    let pointer_number: &mut i32 = &mut number;
    println!("Pointer number is {:p}", pointer_number);

    *pointer_number = 12; 

    let underlaying_number: i32 = *pointer_number;
    println!("Underlaying number is {:?}", underlaying_number);

    println!("number is {:?}", number);

    // tidak mengalokasikan memory lagi
    let number_one = 32;
    let number_two = &number_one;
    println!("number two is {:?}", number_two);

    let mut n = 24;
    for _ in 0..=5 {
        change_value(&mut n);
        println!("n is {:?}", n)
    }
}

fn change_value(n: &mut i32) {
    *n = generate_random_number();
}

fn generate_random_number() -> i32 {
    rand::thread_rng().gen_range(0..1000)
}
