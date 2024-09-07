use rand::Rng;

fn main() {
    let x: i8 = 5;
    println!("x = {}", x);

    {
        println!("x di dalam block expression 👇");
        let x: i8 = x + 1;
        println!("x = {}", x);
    }

    let a: i32 = {
        let n = rand::thread_rng().gen_range(0..100);
        n * 2
    };

    println!("a = {}", a);

    {
        let total;
        {
            let n = rand::thread_rng().gen_range(0..50);
            let c = n * 2;

            {
                total = c + 100;
            }
        }

        println!("total = {}", total);
    }

    let mut total_a = 0;

    'append_with_even_number: {
        let n = rand::thread_rng().gen_range(0..100);

        if n % 2 == 1 {
            break 'append_with_even_number;
        }

        total_a += n;
    }

    println!("total_a = {}", total_a);
}
