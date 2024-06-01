fn main() {
    let var1: &str = "Hello";
    println!("{}", var1);

    let var2 = "hello \
    \"rust\" \
    and \
    \"world\"";
    println!("{}", var2);

    let var3 = "Baris 1
    Baris 2
    Baris 3";
    println!("{}", var3);

    /* raw string */
    let var4 = r#"
    {
        "name": "Dyaksa",
        "age": 20,
    }
    "#;
    println!("{}", var4);

    let var5 = "
    {
        \"name\": \"Dyaksa\",
        \"age\": \"20\",
    }";
    println!("{}", var5);
}
