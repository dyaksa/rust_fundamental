fn main() {
    let number_a: i8 = 10;
    if number_a < 8 {
        println!("number a lebih kecil dari 8");
    }

    let result_a = number_a > 8;
    if result_a {
        println!("number a {} lebih besar dari 8", number_a);
    }

    /* if ,else if ,else */
    let number_b: i16 = 100;
    if number_b > 128 {
        println!("number b lebih besar dari 128");
    }else if number_b <= 100 {
        println!("number b kurang dari sama dengan 100");
    }else {
        println!("tidak keduanya");
    }

    /* nested if */
    let number_c: i8 = 9;

    if number_c >= 6 {
        if number_c >= 9 {
            println!("memuaskan!");
        }else if number_c >= 7 {
            println!("bagus!");
        }else {
            println!("cukup!");
        }
    }else {
        if number_c >= 4 {
            println!("belajar lagi!");
        }else {
            println!("gagal!");
        }
    }

    /* returning from if */
    let number_d: i8 = 10;
    let result_d: bool;

    if number_d >= 10 {
        result_d = true;
    }else {
        result_d = false;
    };

    println!("result d: {}", result_d);

    let number_e = 8;
    let result_e: bool = if number_e >= 8 {
        true
    }else {
        false
    };

    println!("result e: {}", result_e);

        // identasi
    let number_f: i8 = 5;
    let result_f: bool = 
    if number_f >= 8 {
        true
    }else {
        false
    };

    println!("result f: {}", result_f);

    let number_g: i8 = 7;
    let result_g: &str = if number_g >= 8 {
        "lebih besar dari 8"
    }else {
        "lebih kecil dari 8"
    };

    println!("result g: {}", result_g);
}
