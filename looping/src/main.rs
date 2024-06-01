fn main() {
    /* loop break */
    // let mut i = 1;
    // let max_i = 100;

    // loop {
    //     println!("i = {}", i);
    //     i += 1;

    //     if i > max_i {
    //         break;
    //     }
    // }

    // println!("perulangan selesai!");

    /* nested loop */
    let mut x = 1;
    let max_x = 5;

    loop {
        let mut y = max_x;
        let inner_y = x;

        loop {
            print!("* ");
            y -= 1;

            if y < inner_y {
                break;
            }
        }

        println!();
        x += 1;
        if x > max_x {
            break;
        }
    }

    /* continue */
    let mut a = 0;
    let max_a = 15;

    loop {
        a += 1;

        if a % 2 == 0 {
            continue;
        }

        println!("nilai = {}", a);
        if a >= max_a {
            break;
        }
    }

    /* fibonaci */
    let mut fib = (0, 1);
    let max_fib = 20;

    loop {
        let next_fib = fib.0 + fib.1; // 1 --> 2 --> 3 --> 5
        if next_fib > max_fib {
            break;
        }

        println!("{} ", next_fib);
        fib = (fib.1, next_fib); // switch (1,1)  --> (1,2) --> (2,3) --> (3,5)
    }

    /* label loop */
    let mut i = 0;
    let max_i = 5;

    'mainLoop: loop {
        let mut j = 0;
        let max_j = i;

        loop {
            if i > max_i {
                break 'mainLoop;
            }

            print!("* ");

            j += 1;
            if j >= max_j {
                break;
            }
        }

        i += 1;
        println!();
    }

    /* returning from loop */
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("result = {}", result);
}
