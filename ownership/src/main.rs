// fn do_something() {
//     let data_one = "one";
// }

// #[derive(Debug)]
// struct Profile{}
// fn main() {
//     let data_two = "two";
//    {
//     let data_three = "three";
//    }

//    do_something();

//    if true {
//     let data_four = "four";
//    }

//    let x = 24;
//    let y = x;

//    println!("x : {}, y : {}", x, y);

// //    let a = String::from("hello");
// //    let b = a;
// //    println!("a : {:?}, b : {:?}", a, b); 

// //    let p = Profile{};
// //    let np = p;
// //    println!("p: {:?}, np: {:?}", p, np);
// }
fn do_something() -> String {
    let mut k = String::from("Hello");

    {
        let m = String::from("Hello Rust");
        // let n = String::from("Hello World");
        k = m;

        // println!("m: {:?}", m);
    }

   return k;
}

fn say_hello(param: String) {
    println!("param: {:?}", param);
}

fn main() {
    do_something();

    let msg1 = String::from("Hello rust");
    let msg2 = msg1;
    let msg3 = msg2;

    let msg4 = msg2; // error msg2 is moved to msg3
    println!("msg1: {:?}", msg1); // error msg1 is moved to msg2

    say_hello(msg3.clone()); // clone msg3
    println!("msg3: {:?}", msg3); // error msg3 is moved to say_hello function
}
