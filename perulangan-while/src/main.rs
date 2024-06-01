use std::{thread::sleep, time::Duration};

fn main() {
    let mut i = 1;
    let max = 5;

    while i <= max {
        println!("index {}", i);
        i += 1;
    }

    /* nested while */
    let mut x = 1;
    let max_x = 5;

    while x <= max_x {
        let mut y = 1;
        let inner_x = x;

        while y <= inner_x {
            print!("* ");
            y += 1;
        }

        println!();
        x += 1;
    }

    /* delay */
    let mut a = 1;
    let max_a = 5;

    while a <= max_a{
        println!("index {}", a);
        a += 1;

        sleep(Duration::from_secs(1));
    }
}
