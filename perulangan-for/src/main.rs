fn main() {
    /* keyword for in */
    for i in 1..10 {
        println!("i = {}", i);
    }

    for i in 1..=10 {
        println!("i = {}", i);
    }

    /* label perulangan */
    'perulangan: for i in 1..=10 {
        if i >= 5 {
            break 'perulangan;
        }

        println!("i = {}", i);
    }

    /* for in array */
    let names = ["Rust", "Go", "Typescript", "Java", "PHP"];

    for name in names {
        println!("{name}");
    }
}
