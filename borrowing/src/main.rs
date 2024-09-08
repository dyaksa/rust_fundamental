fn main() {
    // let msg_1 = String::from("Hello rust!");
    // let msg_2 = msg_1;
    // println!("msg_1 : {:?}, msg_2: {:?}", msg_1, msg_2); // Error

    // let mut msg_3 = String::from("Hello gun!!");
    // let msg_4 = &mut msg_3; // <---------- borrow operation

    // *msg_4 = String::from("Hello rust!!");

    // println!("msg_4 : {:?}", msg_4);
    // println!("msg_3 : {:?}", msg_3);

    // let msg_5 = String::from("Hello gun!!");
    // let msg_6 = &msg_5; // <---------- borrow operation
    // let msg_7 = &msg_5; // <---------- borrow operation
    // let msg_8 = &msg_5; // <---------- borrow operation

    // println!("msg_6 : {:?}", msg_6);
    // println!("msg_7 : {:?}", msg_7);
    // println!("msg_8 : {:?}", msg_8);

    // let mut msg_9 = String::from("Hello gun rust!!");
    // let msg_10 = &mut msg_9; // <---------- borrow operation
    // // let msg_11 = &mut msg_9; // <---------- borrow operation error

    // println!("msg_10 : {:?}", msg_10);

    // let mut msg_12 = String::from("Hello gun rust lagi!!");
    // let msg_13 = &msg_12; // <---------- borrow operation
    // let msg_14 = &mut msg_12; // <---------- borrow operation error

    // println!("msg_13 : {:?}", msg_13);
    // println!("msg_14 : {:?}", msg_14);

    let mut fact_one = String::from("Arthas is the true lich king");
    println!("fact_one : {:?}", fact_one);

    change_value(&mut fact_one);
    println!("fact_two : {:?}", fact_one);

    {
        let fact_two = &mut fact_one;
        *fact_two = String::from("There must always be a lich king");
        println!("fact_one : {:?}", fact_one);
    }

    if fact_one.contains("lich king") {
        let fact_three = &mut fact_one;
        *fact_three = String::from("Who is the real jailer?");
        println!("fact_one : {:?}", fact_one);
    }

    for _ in 0..1 {
        let fact_four = &mut fact_one;
        *fact_four = String::from("Is it Zovaal or Primus?");
        println!("fact_one : {:?}", fact_one);
    }

    let number = 10;
    let a = &number;

    let text = String::from("hello rust");
    let b = &text;

    let c = &24;
    let d = &false;
    let c = &String::from("borrows");
}

fn change_value(txt: &mut String) {
    *txt = String::from("Bolvar is better lich king")
}
