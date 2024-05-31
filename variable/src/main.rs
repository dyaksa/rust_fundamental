fn main() {
    println!("Hello, world!");
    println!("how");
    println!("are");
    println!("you");

    // variable
    let nama_variable = "predefined value";
    println!("{}", nama_variable);

    // immutable variable
    // let message_number = 1;
    // let message = "hello";

    // mutable variable
    let mut message_number = 1;
    let message1 = "hello";
    println!("message number {} : {}", message_number, message1);

    message_number = 2;
    let message2 = "world";
    println!("message number {} : {}", message_number, message2);

    message_number = 3;
    let message3: i8 = 24;
    println!("message number {1} : {0}", message3, message_number);

    // type inference
    // let var1 = 1;
    // let var2 = 1.0;

    // manifest typing
    // let var3: i8 = 1;

    let nmbr: i8;
    nmbr = 1;
    println!("nmbr : {}", nmbr);

    let (var1, var2) = (1,"Halo");
    println!("var1 : {}, var2: {}", var1, var2);

    let (var3, var4): (i8, &str) = (1, "dunia");
    println!("var3 : {}, var4: {}", var3, var4);

    let (var5, mut var6): (i8, &str) = (3, "How");
    println!("var5 : {}, var6: {}", var5, var6);
    var6 = "are";
    println!("var5 : {}, var6: {}", var5, var6);

    let data1 = 21i8;
    println!("data1 : {}", data1);

    let data2 = 24_i8;
    println!("data2 : {}", data2);

    // variable shadowing
    let x = 5;
    println!("x : {}", x);

    let x = x + 1;
    println!("x : {}", x);
}
