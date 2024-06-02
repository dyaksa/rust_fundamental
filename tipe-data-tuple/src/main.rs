fn main() {
    let tuple_a = ("Dyaksa", 29, ["Bandung", "Indonesia"], 40);
    println!("tuple {:?}", tuple_a);

    println!("index 0: {:?}", tuple_a.0);
    println!("index 1: {:?}", tuple_a.1);
    println!("index 2: {:?}", tuple_a.2);
    println!("index 3: {:?}", tuple_a.3);

    let mut tuple_b: (&str,[&str; 3], bool) = ("Dyaksa", ["Bandung", "Indonesia", "Asia"], false);
    println!("before tuple_b {:?}", tuple_b);
    tuple_b.0 = "Dyaksa jr";
    tuple_b.1[0] = "California";
    tuple_b.1[1] = "USA";
    tuple_b.1[2] = "America";
    tuple_b.2 = true;
    println!("after tuple_b {:?}", tuple_b);

    let mut tuple_c: (&str, [&str; 3], bool) = ("default", [""; 3], false);
    tuple_c.0 = "Dyaksa";
    tuple_c.1 = ["Bandung", "Indonesia", "Asia"];
    tuple_c.2 = true;
    println!("tuple_c {:?}", tuple_c);

    /* packing tuple */
    let name = "Dyaksa Jauharuddin Nour";
    let age = 29;
    let programming_language = ["Rust", "Go", "PHP", "Typescript"];
    let is_married = false;

    let tuple_d = (name, age, programming_language, is_married);
    println!("name\t: {}", tuple_d.0);
    println!("age\t: {}", tuple_d.1);
    println!("programming_language\t: {:?}", tuple_d.2);
    println!("is_married\t: {}", tuple_d.3);

    /* unpacking tuple */
    let tuple_e = ("Dyaksa", 29, ["Bandung", "Indonesia"], false);
    let (name, age, address, is_married) = tuple_e;
    println!("name\t: {}", name);
    println!("age\t: {}", age);
    println!("address\t: {:?}", address);
    println!("is_married\t: {}", is_married);

    /* tuple tanpa isi */
    let tuple = ();
    println!("tuple {:?}", tuple);

}
