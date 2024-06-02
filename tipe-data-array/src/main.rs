use core::str;

fn main() {
    let mut numbers = [5,6,4,3,2,5];
    println!("array {:?}", numbers);

    let data0 = numbers[0];
    println!("data0 {}", data0);

    let data1 = numbers[1];
    println!("data1 {}", data1);

    numbers[1] = 16;
    numbers[3] = 8;
    println!("array {numbers:?}");

    let angka_integer = [4,5,3,6,7,8];
    println!("array {:?}", angka_integer);

    let angka_float = [22.7, 3.14, 22.1];
    println!("array {angka_float:?}");
    
    /* manifest typing & predefined value */
    let data_bool: [bool; 2] = [true, false];
    println!("array bool {:?}", data_bool);

    let data_unsigned: [u8; 3] = [1,2,3];
    println!("array unsigned {data_unsigned:?}");

    let data_numerik: [i32; 10] = [0; 10];
    println!("{:?}", data_numerik);

    let data_numerik1 = [4; 5];
    println!("{data_numerik1:?}");

    let named = ["Dyaksa", "Jauharuddin", "Nour"];
    let length = named.len();
    println!("length {}", length);

    let nameds: [&str; 3] = ["Dyaksa","Jauharuddin","Nour"];
    for name in nameds {
        println!("{}", name);
    } 

    for i in 0..nameds.len() {
        println!("array index ke {} = {}", i, named[i]);
    }

    let schools: [&str; 4] = ["SD", "SMP", "SMA", "Kuliah"];

    let mut idx = 0;
    while idx < schools.len() {
        println!("array index ke {} = {}", idx, schools[idx]);
        idx += 1;
    }

    let friends: [&str; 3] = ["Dyaksa", "Jauharuddin", "Nour"];
    let mut index = 0;
    loop {
        if index >= friends.len() {
            break;
        }
        println!("array index ke {} = {}", index, friends[index]);
        index += 1;
    }

    let parents: [&str; 2] = ["Dad", "Mom"];
    for (i, value) in parents.iter().enumerate() {
        println!("array index ke {} = {}", i, value);
    }

    /* nested array */
    let data_arr = [
        ["Dyaksa","Jauharuddin", "Nour"],
        ["Dad", "Mom","Grandpa"],
        ["SD", "SMP", "SMA"],
    ];

    for i in data_arr {
        for j in i {
            print!("{},", j);
        }
        println!();
    }
}
