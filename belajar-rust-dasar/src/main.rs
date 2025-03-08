fn main() {
    println!("Hello, world!");
    print!("hello, world!\n");
    println!("Hello, world!");
}


#[test]
fn hello_test() {
    println!("hello world");
}

#[test]
fn test_variable() {
    let name = "Dyaksa Jauharuddin Nour";
    println!("Hello {}", name);
}

#[test]
fn test_mutable() {
    let mut name = "Dyaksa Jauharuddin Nour";
    println!("Hello {}", name);

    name = "Dyaksa";
    println!("Hello {}", name);
}

#[test]
fn static_typing() {
    let name = "Dyaksa Jauharuddin Nour";
    println!("Hello {}", name);

    // name = 100;
    println!("Hello {}", name);
}

#[test]
fn test_shadowing() {
    let name = "Dyaksa Jauharuddin Nour";
    println!("Hello {}", name);

    let name = 100;
    println!("Hello {}", name);
}

#[test]
fn explicit() {
    let number: i32 = 20;

    println!("Number: {}", number);
}

#[test]
fn number() {
    let a: i8 = 10;
    println!("Number: {}", a);

    let b: f32 = 10.5;
    println!("Number: {}", b);
}

#[test]
fn number_conversion() {
    let a: i8 = 18;
    println!("Number: {}", a);

    let b: i16 = a as i16;
    println!("Number: {}", b);

    let c: i32 = a as i32;
    println!("Number: {}", c);

    let d: i64 = 1000000000;
    let e: i8 = d as i8;

    println!("Number: {}", e);
}

#[test]
fn numeric_operation() {
    let a: i32 = 10;
    let b: i32 = 5;

    let c = a * b;
    println!("Result: {}", c);
}

#[test]
fn augmented_assignment() {
    let mut a = 10;
    a += 10;

    println!("Result: {}", a);

    a -= 10;

    println!("Result: {}", a);
}

#[test]
fn boolean() {
    let a = true;
    let b: bool = false;

    println!("{} {}", a, b);
}

#[test]
fn comparison() {
    let a = 10;
    let b = 20;

    let result = a > b;

    println!("Result: {}", result);
}

#[test]
fn boolean_operator() {
    let absen = 75;
    let nilai_akhir = 75;

    let lulus_abses = absen >= 75;
    let lulus_nilai_akhir = nilai_akhir >= 80;

    let result = lulus_abses && lulus_nilai_akhir;

    println!("lulus: {}", result);
}

#[test]
fn char_type() {
    let a: char = 'a';
    let b: char = 'b';

    println!("{} {}", a, b);
}

#[test]
fn tuple() {
    let data : (i8, i8, bool) = (10, 20, true);

    println!("{:?}", data);
    // println!("{}", data.0);
    // println!("{}", data.1);
    // println!("{}", data.2);

    let (a, b, _) = data;
    println!("{} {}", a, b);
}

#[test]
fn mutable_tupe() {
    let mut data: (i8, i16, bool) = (10, 20, true);

    println!("{:?}", data);

    let (a, b, c) = data;
    println!("{} {} {}", a, b, c);

    data.0 = 100;
    data.1 = 200;
    data.2 = false;

    println!("{:?}", data);
}

fn unit() {
    println!("Hello, world!");
}

#[test]
fn test_unit() {
    let result = unit();
    println!("{:?}", result);

    let unit = ();
    println!("{:?}", unit);
}

#[test]
fn array() {
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", array);

    let a = array[0];
    let b = array[1];
    let c = array[2];
    let d = array[3];
    let e = array[4];

    println!("{} {} {} {} {}", a, b, c, d, e);
}

#[test]
fn mutable_array() {
    let mut array: [i32; 5] = [1,2,3,4,5];
    println!("{:?}", array);

    array[0] = 100;
    array[1] = 200;

    println!("{:?}", array);

    let length: usize = array.len();
    println!("Length: {}", length);
}

#[test]
fn multidimentional_array() {
    let array: [[i32; 2]; 3] = [
        [1, 2],
        [3, 4],
        [5, 6]
    ];

    println!("{:?}", array);
    println!("{}", array[0][0]);
    println!("{}", array[0][1]);
    println!("{}", array[1][1]);
}

const MAXIMUM: i32 = 100;

#[test]
fn constant(){
    const MINIMUM: u8 = 0;
    println!("{} {}", MAXIMUM, MINIMUM);
}

#[test]
fn variable_scope() {
    let nama = "Dyaksa Jauharuddin";

    {
        println!("{}", nama);
        let middle = "Jauharuddin";
        println!("{}", middle);
    }

    // println!("{}", middle);
}

#[test]
fn stack_heap() {
    function_a();
    function_b();
}

fn function_a() {
    let a = 10;
    let b = String::from("Dyaksa");

    println!("{} {}", a, b);
}

fn function_b(){
    let a = 20;
    let b = String::from("Jauharuddin");

    println!("{} {}", a, b);
}

#[test]
fn string_slice(){
    let name: &str = " Dyaksa Jauharuddin Nour ";
    let trim: &str = name.trim();

    println!("{}", name);
    println!("{}", trim);

    // let mut username: &str = "dyaksa";
    // username = "jauharuddin";
    // println!("{}", username);
}

#[test]
fn string_type() {
    let mut name: String = String::from("Dyaksa Jauharuddin");
    name.push_str(" Nour");
    println!("{}", name);

    let budi = name.replace("Dyaksa", "Budi");
    println!("{}", budi);

    println!("{}", name);
    println!("{}", budi)
}

#[test]
fn ownership_rules() {
    let a = 10;

    {
        let b = 20;
        println!("{}", b);
    }
    
    println!("{}", a);
}

#[test]
fn data_copy() {
    let a = 10;
    let b = a;

    println!("{} {}", a, b);
}

#[test]
fn ownership_movement() {
    let a = String::from("Dyaksa Jauharuddin Nour");

    let b = a; // ownership movement

    println!("{}", b);
}

#[test]
fn clone() {
    let a: String = String::from("Dyaksa Jauharuddin Nour");
    let b: String = a.clone();

    println!("{} {}", a, b);
}

#[test]
fn if_expression() {
    let value = 5;
    let result = if value >= 8 {
        "Good"
    } else if value >= 5 {
        "Not Bad"
    } else {
        "Bad"
    };

    // if value >= 8 {
    //     result = "Good";
    // } else if value >= 5 {
    //    result = "Not Bad";
    // } else{
    //     result = "Bad";
    // }

    println!("{}", result);
}

#[test]
fn loop_statement(){
    let mut counter = 0;

    loop {
        counter += 1;

        if counter >= 10 {
            break;
        } else if counter % 2 == 0 {
            continue;
        }

        println!("{}", counter);
    }
}

#[test]
fn loop_return_value() {
    let mut count = 0;

    let result = loop {
        count += 1;

        if count > 10 {
            break count * 2;
        }
    };

    println!("{}", result);
}

#[test]
fn loop_label() {
    let mut number = 0;
    'outer: loop {
        let mut i = 0;
        loop {
            if number > 10 {
                break 'outer;
            }

            println!("{} x {} = {}", i, number, i * number);
            i += 1;
            if i > 10 {
                break;
            }
        } 
        number += 1;    
    }
}

#[test]
fn shile_loop() {
    let mut counter = 0;

    while counter <= 10 {
        if counter % 2 == 0 {
            println!("{}", counter);
        }

        counter += 1;
    }
}

#[test]
fn array_iteration() {
    let array: [char; 5] = ['a', 'b', 'c', 'd', 'e'];
    let mut index = 0;

    while index < array.len() {
        println!("value {}", array[index]);
        index += 1;
    }
}

#[test]
fn for_loop() {
    let array: [char; 5] = ['a', 'b', 'c', 'd', 'e'];

    for value in array {
        println!("value {}", value);
    }
}

#[test]
fn range_exclusive() {
    let array: [char; 5] = ['a', 'b', 'c', 'd', 'e'];

    let range = 0..5;
    println!("{:?}", range.start);
    println!("{:?}", range.end);

    for index in range {
        println!("value {}", array[index]);
    }
}

#[test]
fn range_inclusive() {
    let array: [char; 5] = ['a', 'b', 'c', 'd', 'e'];

    for i in 0..=4 {
        println!("{}", array[i])
    }
}

fn say_hello(nama: &str) {
    println!("hello rust! {}", nama);
}

#[test]
fn test_say_hello() {
    say_hello("Dyaksa");
}

fn factorial_loop(n: i32) -> i32 {
    let mut result = 1;

    if n < 0 {
        return 0;
    }

    for i in 1..=n {
        result *= i;
    }

    return result;
}

#[test]
fn test_factorial_loop() {
    let result = factorial_loop(5);
    println!("{}", result);

    // let result: i32 = factorial_loop(0);
    // println!("{}", result);
}

fn print_text(value: String, time: u32) {
    if time == 0 {
        return;
    }else {
        println!("{}", value)
    }

    print_text(value, time - 1);
}

#[test]
fn test_print_text() {
    print_text(String::from("Dyaksa Jauharuddin"), 10);
}

fn factorial_recursive(n: u32) -> u32 {
    if n <= 1 {
        return 1;
    }

    return n * factorial_recursive(n - 1)
}

#[test]
fn test_factorial_resursive() {
    let result = factorial_recursive(5);

    println!("{}", result);
}

fn stack_number(n: i32) {
    println!("{}", n)
}

fn hi(name: String) {
    println!("{}", name)
}

#[test]
fn print_hi() {
    let number = 10;
    stack_number(number);
    println!("{}", number);

    let name: String = String::from("Dyaksa");
    hi(name);
    // println!("{}", name)
}

fn full_name(first_name: String, last_name: String) -> (String, String, String) {
    let full_name = format!("{} {}", first_name, last_name);

    return (first_name, last_name, full_name);
}

#[test]
fn test_full_name() {
    let first_name = String::from("Dyaksa");
    let last_name: String = String::from("Jauharuddin");

    let (first_name, last_name, name) = full_name(first_name, last_name);

    println!("{}", name);
    println!("{}", first_name);
    println!("{}", last_name);
    // println!("{}", first_name);
}

fn greetings(first_name: &String, last_name: &String) -> String {
    format!("{} {}", first_name, last_name)
}

#[test]
fn test_greetings() {
    let first_name = String::from("Dyaksa");
    let last_name : String = String::from("Jauharuddin");

    let name = greetings(&first_name, &last_name);

    println!("{}", name);
    println!("{}", first_name);
    println!("{}", last_name);
}

fn change_value(name: &mut String) { // harus mutable reference
    name.push_str("Jauharuddin");
}

#[test]
fn test_change_value() {
    let mut name = String::from("Dyalsa"); // harus mutable variable
    let borrow_1 = &mut name;
    // let borrow2 = &mut name;

    // change_value(&mut name);
    change_value(borrow_1);
    change_value(&mut name);
    change_value(&mut name);
    println!("{}", name);
}

fn get_full_name(first_name: &String, last_name: &String) -> String {
    let name = format!("{} {}", first_name, last_name);
    return name;
}

#[test]
fn test_get_full_name() {
    let first_name = String::from("Dyaksa");
    let last_name: String = String::from("Jauharuddin");
    let name = get_full_name(&first_name, &last_name);
    println!("{}", name);
}

#[test]
fn slice_reference() {
    let array: [i32;10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let ref_1: &[i32] = &array[..];
    println!("{:?}", ref_1);

    let ref_2: &[i32] = &array[0..5];
    println!("{:?}", ref_2);

    let ref_3: &[i32] = &array[5..];
    println!("{:?}", ref_3);

    println!("{:?}", array);
}

#[test]
fn string_slice_ref() {
    let full_name = String::from("Dyaksa Jauharuddin nour");
    let slice_1 = &full_name[..];
    println!("{}", slice_1);

    let slice_2 = &full_name[0..5];
    println!("{}", slice_2);

    let slice_3 = &full_name[14..];
    println!("{}", slice_3);
}

struct Person {
    first_name: String,
    middle_name: String,
    last_name: String,
    age: u32,
}

impl Person {
    fn say_hello(&self, name: &str) {
        println!("hello {} my name is {}", name, self.first_name);
    }
}

fn print_person(person: &Person) {
    println!("{}", person.first_name);
    println!("{}", person.last_name);
    println!("{}", person.middle_name);
    println!("{}", person.age);
}

#[test]
fn struct_person() {
    let person: Person = Person{
        first_name: String::from("Dyaksa"),
        middle_name: String::from("Jauharuddin"),
        last_name: String::from("Nour"),
        age: 29,
    };

    print_person(&person);
}

#[test]
fn init_short_hand() {
    let first_name = String::from("Dyaksa");
    let middle_name: String = String::from("Jauharuddin");
    let last_name: String = String::from("Nour");
    let age = 29;

    let mut person = Person{
        first_name,
        middle_name,
        last_name,
        age
    };

    print_person(&person);

    person.first_name = String::from("Dyaksaaaa");

    let person_2: Person = Person{
        first_name: person.first_name.clone(),
        middle_name: person.middle_name.clone(),
        last_name: person.last_name.clone(),
        ..person
    };
    print_person(&person_2);

    println!("{}", person.first_name);
}

#[test]
fn test_method() {
    let first_name = String::from("Dyaksa");
    let middle_name = String::from("Jauharuddin");
    let last_name = String::from("Nour");

    let person: Person = Person{
        first_name,
        last_name,
        middle_name,
        age: 29
    };

    person.say_hello("Tatkhi");
}

struct GeoPoint(f64, f64);

impl GeoPoint {
    fn new(long: f64, lat: f64) -> GeoPoint {
        GeoPoint(long, lat)
    }
}

#[test]
fn tuple_struct() {
    let geo_point: GeoPoint = GeoPoint(-6.787877, 67.9989);

    println!("{}", geo_point.0);
    println!("{}", geo_point.1);
}

#[test]
fn test_associated_function() {
    let geo_point = GeoPoint::new(-67.938982, 67.998293);

    println!("lat {}", geo_point.0);
    println!("long {}", geo_point.1);
}

struct Nothing;

#[test]
fn test_nothing() {
    let _nothing_1: Nothing = Nothing;
    let _nothing_2: Nothing = Nothing{}; // tidak niat digunakan wkwkwk
}

enum Level {
    Regular,
    Premium,
    Platinum,
}

#[test]
fn test_enum() {
    let _level: Level = Level::Regular;

    match _level {
        Level::Regular => println!("Regular"),
        Level::Premium => println!("Premium"),
        Level::Platinum => println!("Platinum"),
    }
}


enum Payment {
    CreditCard(String),
    BankTransfer(String, String),
    Ewallet(String, String),
}

impl Payment {
    fn pay(&self, amount: u32) {
        match self {
            Payment::CreditCard(number) => println!("Paying with credit card {} amount {}", number, amount),
            Payment::BankTransfer(bank, number) => println!("Paying with bank transfer {} {} amount {}", bank, number, amount),
            Payment::Ewallet(wallet, number) => println!("Paying with ewallet {} {} amount {}", wallet, number, amount),
        }
    }
}

#[test]
fn test_payment() {
    let _credit_card = Payment::CreditCard(String::from("1234567890"));
    let _bank_transfer = Payment::BankTransfer(String::from("BCA"), String::from("1234567890"));
    let _ewallet = Payment::Ewallet(String::from("OVO"), String::from("1234567890"));

    _credit_card.pay(100);
    _bank_transfer.pay(200);
    _ewallet.pay(300);
}

#[test]
fn test_match_value() {
    let name = "Budi";

    match name {
        "Dyaksa" | "Budi" | "Joko" => println!("Hello Dyaksa"),
        "Jauharuddin" => println!("Hello Jauharuddin"),
        _ => println!("Hello {}", name),
    }
}

#[test]
fn test_range_pattern(){
    let value = 100;
    match value {
        1..=100 => println!("1 - 100"),
        101..=200 => println!("101 - 200"),
        _ => println!("other"),
    }
}

#[test]
fn destructure_pattern(){
    let geopoint = GeoPoint(100.0, 200.0);

    match geopoint {
        GeoPoint(long, 0.0) => println!("long {} lat 0.0", long),
        GeoPoint(0.0, lat) => println!("long 0.0 lat {}", lat),
        GeoPoint(long, lat) => println!("long {} lat {}", long, lat),
    }

    let perrson = Person{
        first_name: String::from("Dyaksa"),
        middle_name: String::from("Jauharuddin"),
        last_name: String::from("Nour"),
        age: 29,
    };

    match  perrson {
        Person{first_name, last_name, ..} => println!("{} {}", first_name, last_name),
    }
}

#[test]
fn destructuring_struct_tuple() {
    let geopoint = GeoPoint(100.0, 200.0);

    match geopoint {
        GeoPoint(long, _) => println!("long {}", long),
    }
}

#[test]
fn test_match_expression(){
    let value = 0;

    let result = match value {
        0 => "zero",
        1 => "one",
        _ => "other",
    };

    // let result = match value {
    //     0 => "zero",
    //     1 => "one",
    //     _ => "other",
    // };

    println!("{}", result);
}

type Age = u8;
type IdentityNumber = String;

struct Customer {
    name: String,
    age: Age,
    identity_number: IdentityNumber,
}

#[test]
fn test_type_alias(){
    let customer = Customer{
        name: String::from("Dyaksa"),
        age: 29,
        identity_number: String::from("1234567890"),
    };

    println!("{}", customer.name);
    println!("{}", customer.age);
    println!("{}", customer.identity_number);
}

pub mod model {
    pub struct User {
        pub first_name: String,
        pub middle_name: String,
        pub last_name: String,
        pub age: u8,
    }

    impl User {
        pub fn new(first_name: &String, middle_name: &String, last_name: &String, age: u8) -> User {
            User{
                first_name: first_name.clone(),
                middle_name: middle_name.clone(),
                last_name: last_name.clone(),
                age: age.clone(),
            }
        }

        pub fn say_hello(&self) {
            println!("Hello, my name is {}", self.first_name);
        }
    }

    #[test]
    fn test_module(){
        let first_name = String::from("Dyaksa");
        let middle_name = String::from("Jauharuddin");
        let last_name = String::from("Nour");
        let age = 29;

        let user = User::new(&first_name, &middle_name, &last_name, age);
        println!("{}", first_name);
        user.say_hello();
    }
}

// mod first {
//     pub fn say_hello(){
//         println!("hello rust");
//     }
// }

// mod second {
//     pub fn say_hello(){
//         println!("hello from second module");
//     }
// }

// use first::say_hello as shf;
// use second::say_hello as shs;


mod first;
mod second;
mod third;

use std::{cell::RefCell, collections::{BTreeMap, BTreeSet, HashMap, HashSet, LinkedList, VecDeque}, fmt::Debug, ops::{Add, Deref}, rc::Rc};

use first::say_hello as shf;
use second::say_hello as shs;

#[test]
fn test_module(){
    shf();
    shs();
    first::second::third::say_hello();
}

trait CanSayHello {
    fn new(first_name: String, middle_name: String, last_name: String, email: String, address: String) -> Self;
    fn hello(&self) -> String {
        format!("HELLO")
    }
    fn say_hello(&self) -> String;
    fn say_hello_to(&self, name: &str) -> String;
}

trait CanSayGoodbye {
    fn say_goodbye(&self) -> String;
    fn say_goodbye_to(&self, name: &str) -> String;
}

struct User {
    first_name: String,
    middle_nae: String,
    last_name: String,
    email: String,
    address: String,
}

impl CanSayHello for User{
    fn new(first_name: String, middle_name: String, last_name: String, email: String, address: String) -> Self {
        User{
            first_name,
            middle_nae: middle_name,
            last_name,
            email,
            address,
        }
    }

    fn hello(&self) -> String {
        format!("hello my name is {}", self.first_name)
    }

    fn say_hello(&self) -> String {
        format!("Hello, my name is {}", self.first_name)
    }

    fn say_hello_to(&self, name: &str) -> String {
        format!("Hello, {} my name is {}", name, self.first_name)
    }
}

impl CanSayGoodbye for User {
    fn say_goodbye(&self) -> String {
        format!("Goodbye, my name is {}", self.first_name)
    }

    fn say_goodbye_to(&self, name: &str) -> String {
        format!("Goodbye, {} my name is {}", name, self.first_name)
    }
}

fn say_hello_to_user(user: &impl CanSayHello) {
    println!("{}", user.say_hello());
}

fn hello_and_goodbye(user: &(impl CanSayHello + CanSayGoodbye)) {
    println!("{}", user.say_hello());
    println!("{}", user.say_goodbye());
}

#[test]
fn test_trait() {
    let first_name = String::from("Dyaksa");
    let middle_name = String::from("Jauharuddin");
    let last_name = String::from("Nour");
    let email = String::from("test@yopmail.com");
    let address = String::from("Jakarta");

    let user = User::new(first_name, middle_name, last_name, email, address);

    say_hello_to_user(&user);

    println!("{}", user.say_goodbye());

    hello_and_goodbye(&user);

    // User::say_hello(&user);
    // CanSayHello::say_hello(&user);

    // println!("{}", user.say_hello());
    // println!("{}", user.say_hello_to("Chika"));
    // println!("{}", user.hello());
}

struct SimplePerson {
    name: String,
}

impl CanSayGoodbye for SimplePerson {
    fn say_goodbye(&self) -> String {
        format!("Goodbye, my name is {}", self.name)
    }

    fn say_goodbye_to(&self, name: &str) -> String {
        format!("Goodbye, {} my name is {}", name, self.name)
    }
}

fn create_simple_person(name: String) -> impl CanSayGoodbye {
    SimplePerson{name}
}

#[test]
fn test_trait_return() {
    let person = create_simple_person(String::from("Dyaksa"));
    println!("{}", person.say_goodbye());
}

trait CanSay: CanSayHello + CanSayGoodbye {
    fn say(&self) {
        println!("{}", self.say_hello());
    }
}

struct SimpleMan {
    name: String,
}

impl CanSayHello for SimpleMan {
    fn new(first_name: String, middle_name: String, last_name: String, email: String, address: String) -> Self {
        SimpleMan{
            name: first_name,
        }
    }

    fn hello(&self) -> String {
        todo!()
    }

    fn say_hello(&self) -> String {
        todo!()
    }

    fn say_hello_to(&self, name: &str) -> String {
        todo!()
    }
}

impl CanSayGoodbye for SimpleMan {
    fn say_goodbye(&self) -> String {
        todo!()
    }

    fn say_goodbye_to(&self, name: &str) -> String {
        todo!()
    }
}

impl CanSay for SimpleMan{}


struct Point<T = i32> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn get_x(&self) -> &T {
        &self.x
    }

    fn get_y(&self) -> &T {
        &self.y
    }
}

trait GetValue<T> where T: PartialOrd {
    fn get_value(&self) -> &T;
}


impl<T> GetValue<T> for Point<T> where T: PartialOrd{
    fn get_value(&self) -> &T {
        &self.x
    }
}

#[test]
fn test_generic_method() {
    let point = Point{x: 10, y: 20};
    println!("{}", point.get_x());
    println!("{}", point.get_y());
    println!("{}", point.get_value());
}

#[test]
fn test_generic_struct() {
    let point = Point::<i32>{x: 10, y: 20};
    let float_point = Point::<f32>{x: 10.5, y: 20.5};

    println!("{:?}", point.x);
    println!("{:?}", float_point.y);
}

enum Value<T> {
    Single,
    Double(T),
}

#[test]
fn test_generic_enum() {
    let double = Value::<i32>::Double(10);

    match double {
        Value::Single => println!("Single"),
        Value::Double(value) => println!("Double {}", value),
    }
}


struct Hi<T = SimplePerson> where T: CanSayGoodbye {
    value: T,
}

#[test]
fn test_generic_bound() {
    let hi = Hi::<SimplePerson> {
        value: SimplePerson{name: String::from("Dyaksa")},
    };

    println!("{}", hi.value.say_goodbye());
}

fn min<T: PartialOrd>(a: T, b: T) -> T {
    if a < b {
        a
    } else {
        b
    }
}

#[test]
fn generic_in_fn() {
    let min = min(10, 20);
    println!("{}", min);
}


struct Apple {
    quantity: u32,
}

impl Add for Apple {
    type Output = Apple;

    fn add(self, rhs: Self) -> Self::Output {
        Apple{
            quantity: self.quantity + rhs.quantity,
        }
    }
}

#[test]
fn test_operator_add(){
    let apple1 = Apple{quantity: 10};
    let apple2 = Apple{quantity: 20};

    let result = apple1 + apple2;

    println!("{}", result.quantity);
}

fn double(x : Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i * 2),
    }
}

#[test]
fn test_option(){
    let result = double(Some(10));
    println!("{:?}", result.unwrap());

    let none = double(None);
    println!("{:?}", none);
}

impl PartialEq for Apple {
    fn eq(&self, other: &Self) -> bool {
        self.quantity == other.quantity
    }
}

impl PartialOrd for Apple {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.quantity.partial_cmp(&other.quantity)
    }
}

#[test]
fn comparator(){
    let apple_1 = Apple{quantity: 10};
    let apple_2 = Apple{quantity: 20};

    println!("{}", apple_1 == apple_2);
    println!("{}", apple_1 < apple_2);
    println!("{}", apple_1 > apple_2);
}


#[test]
fn string_manipulation(){
    let s = String::from("Dyaksa Jauharuddin Nour");

    println!("{}", s.to_uppercase());
    println!("{}", s.to_lowercase());

    println!("{}", s.replace("Dyaksa", "Budi"));
    println!("{}", s.replace("Jauharuddin", "Joko"));
    println!("{}", s.replace("Nour", "Santoso"));

    println!("{}", s.contains("Dyaksa"));
    println!("{}", s.contains("Jauharuddin"));
    println!("{}", s.contains("Nour"));
}

struct Category {
    id: String,
    name: String,
}

impl Debug for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Category")
            .field("id", &self.id)
            .field("name", &self.name)
            .finish()
    }
}

#[test]
fn test_debug(){
    let category = Category{
        id: String::from("1"),
        name: String::from("Elektronik"),
    };

    println!("{:?}", category);
}

#[test]
fn test_closure() {
    let sum: fn(i32, i32) -> i32 = |a:i32, b:i32| -> i32 {
        a + b
    };

    let reault = sum(10, 20);
    println!("{}", reault);
}

fn print_with_filter(name: String, filter: fn(String) -> String) {
    let result = filter(name);
    println!("{}", result);
}

fn to_uppercase(value: String) -> String {
    value.to_uppercase()
}

#[test]
fn test_closure_as_parameter() {
    let name = String::from("Dyaksa Jauharuddin Nour");
    print_with_filter(name, to_uppercase);
}

#[test]
fn test_clsure_scope() {
    let mut counter = 0;

    let mut increment = || {
        counter += 1;
        println!("increment");
    };

    increment();
    increment();
    increment();

    println!("{}", counter);
}

struct Counter {
    counter: i32,
}

impl Counter {
    fn increment(&mut self) {
        self.counter += 1;
    }
}

#[test]
fn test_counter_struct() {
    let mut counter = Counter{counter: 0};
    counter.increment();
    counter.increment();
    counter.increment();

    println!("{}", counter.counter);
}

#[test]
fn test_vector() {
    let mut names = Vec::<String>::new();
    names.push(String::from("Dyaksa"));
    names.push(String::from("Jauharuddin"));
    names.push(String::from("Nour"));

    for name in names {
        println!("{}", name);
    }
}

#[test]
fn test_vector_deque() {
    let mut names = VecDeque::<String>::new();
    names.push_back(String::from("Jauharuddin"));
    names.push_back(String::from("Nour"));
    names.push_front(String::from("Dyaksa"));

    for name in &names {
        println!("{}", name);
    }

    println!("{:?}", names);
}

#[test]
fn linked_list(){
    let mut names  = LinkedList::<String>::new();
    names.push_back(String::from("Dyaksa"));
    names.push_back(String::from("Jauhruddin"));

    for name in names {
        println!("{}", name);
    }
}

#[test]
fn test_hash_map(){
    let mut map : HashMap<String, String> = HashMap::new();
    map.insert(String::from("name"), String::from("Dyaksa"));
    map.insert(String::from("age"), String::from("23"));

    let name = map.get("name");
    println!("{}", name.unwrap());
}

#[test]

fn test_b_tree_map() {
    let mut map : BTreeMap<String, String> = BTreeMap::new();
    map.insert(String::from("name"), String::from("Dyaksa"));
    map.insert(String::from("age"), String::from("23"));

    for entry in map {
        let (name, value) = entry;
        println!("{} {}", name, value);
    }
}

#[test]
fn hash_set(){
    let mut set: HashSet<String> = HashSet::new(); // tidak ada data duplikat tapi tidak berurut
    set.insert(String::from("Dyaksa"));
    set.insert(String::from("Dyaksa"));
    set.insert(String::from("Jauharuddin"));

    for s in set {
        println!("{}", s);
    }
}

#[test]
fn btree_set(){
    let mut set: BTreeSet<String> = BTreeSet::new(); //  tidak ada adata duplikat dan berurut
    set.insert(String::from("Dyaksa")); 
    set.insert(String::from("Dyaksa"));
    set.insert(String::from("Jauharuddin"));

    for s in set {
        println!("{}", s);
    }
}

#[test]
fn test_iterator(){
    let array:[i32;5] = [1, 2, 3, 4, 5];
    let mut iterator = array.iter();

    while let Some(value) = iterator.next() {
        println!("{}", value);
    }

    for value in iterator {
        println!("{}", value);
    }
}

#[test]
fn test_iterator_method(){
    let vector: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("{:?}", vector);
    let sum: i32 = vector.iter().sum();
    println!("{}", sum);
    let usize = vector.iter().count();
    println!("{}", usize);
    let doubled: Vec<i32> = vector.iter().map(|x| x * 2).collect();
    println!("{:?}", doubled);
    let odd: Vec<&i32> = vector.iter().filter(|x| *x % 2 != 0).collect();
    println!("odd {:?}", odd);
}

fn connect_database(host: Option<String>) {
    match host {
        Some(host) => println!("Connecting to database {}", host),
        None => panic!("Cannot connect to database"),
    }
}

#[test]
fn test_connectiing_to_database(){
    connect_database(Some(String::from("localhost")));
    // connect_database(None);
}

fn connect_cache(host: Option<String>) -> Result<String, String> {
    match host {
        Some(host) => Ok(format!("Connecting to cache {}", host)),
        None => Err(String::from("Cannot connect to cache")),
    }
}

fn connect_email(host: Option<String>) -> Result<String, String> {
    match host {
        Some(host) => Ok(format!("Connecting to email {}", host)),
        None => Err(String::from("Cannot connect to email")),
    }
}

fn connect_application(host: Option<String>) -> Result<String, String> {
    connect_cache(host.clone())?;
    // match connect_cache {
    //     Ok(_) => {},
    //     Err(message) => return Err(message),
    // }

    connect_email(host.clone())?;
    // match connect_email {
    //     Ok(_) => {},
    //     Err(message) => return Err(message),
    // }

    Ok("Connected".to_string())
}

#[test]
fn test_connect_application(){
    // let host = None;
    let host = Some(String::from("localhost"));
    let connect_application = connect_application(host);

    match connect_application {
        Ok(message) => println!("{}", message),
        Err(message) => println!("{}", message),
    }
}

#[test]
fn test_connect_cache() {
    let host = String::from("localhost");
    let connect_cache = connect_cache(Some(host));

    match connect_cache {
        Ok(_) => println!("Connected"),
        Err(message) => println!("{}", message),
    }
}

#[test]
fn dangling_reference() {
    let r: &i32;

    {
        let x = 5;
        // r = &x;
    }

    r = &10;

    println!("{}", r);
}

fn longest<'a>(value1: &'a str, value2: &'a str) -> &'a str { // lifetime annotation
    if value1.len() > value2.len() {
        value1
    } else {
        value2
    }
}

#[test]
fn test_longest() {
    let value1 = String::from("Dyaksa");
    let value2 = String::from("Jauharuddin");

    let result = longest(value1.as_str(), value2.as_str());
    println!("{}", result);
}


// lifetime struct
struct  Student<'a> {
    name: &'a str,
}

impl<'a> Student<'a> {
    fn new(name: &'a str) -> Student<'a> {
        Student{name}
    }

    fn longet_student(&self, student: Student<'a>) -> &'a str {
        if self.name.len() > student.name.len() {
            self.name
        } else {
            student.name
        }
    }
}

fn longest_student<'a>(student: Student<'a>, student2: Student<'a>) -> &'a str {
    if student.name.len() > student2.name.len() {
        student.name
    } else {
        student2.name
    }
}

#[test]
fn test_student() {
    let student = Student{name: "Dyaksa"};
    let student2 = Student{name: "Jauharuddin"};

    let result = student.longet_student(student2);
    println!("{}", result);

    // let result = longest_student(student, student2);
    // println!("{}", result);
}

struct Teacher<'a, T> where T: Ord {
    id: T,
    name: &'a str,
}

#[test]
fn test_lifetime_annotation_teacher() {
    let teacher : Teacher<i32> = Teacher{id: 1, name: "Dyaksa"};

    println!("{}", teacher.id);
    println!("{}", teacher.name);
}

#[derive(Debug, PartialEq, PartialOrd)]
struct Company {
    name: String,
    address: String,
    website: String,
}

impl Company {
    fn new(name: String, address: String, website: String) -> Company {
        Company{
            name,
            address,
            website,
        }
    }
}

#[test]
fn attribute_derive() {
    let name = String::from("Dyaksa");
    let address = String::from("bandung");
    let website = String::from("dyaksa.com");

    let company = Company::new(name, address, website);

    let company2 = Company::new(String::from("Dyaksa"), String::from("bandung"), String::from("dyaksa.com"));

    let eq = company == company2;

    println!("{}", eq);

    println!("{:?}", company);
}

fn display_number(value: i32) {
    println!("{}", value);
}

fn display_number_reference(value: &i32) {
    println!("{}", value);
}

#[test]
fn test_box() {
    let value: Box<i32> = Box::new(10);
    println!("{}", value);

    display_number(*value);
}

#[derive(Debug)]
enum ProductCategory {
    Of(String, Box<ProductCategory>),
    End,
}

#[test]
fn test_product_category() {
    let laptop = ProductCategory::Of(
        "Laptop".to_string(), 
        Box::new(ProductCategory::Of("Dell".to_string(), 
        Box::new(ProductCategory::End)))
    );

    println!("{:?}", laptop);
    product_category(laptop);
}

fn product_category(product: ProductCategory) {
    match product {
        ProductCategory::Of(name, product ) => {
            println!("{}", name);
            product_category(*product);
        },
        ProductCategory::End => println!("End"),
    }
}

#[test]
fn test_dereference() {
    let value1 = Box::new(10);
    let value2 = Box::new(20);

    let result: i32 = *value1 * *value2;

    println!("{}", result);
}

struct MyValue<T> {
    value: T,
}

impl<T> Deref for MyValue<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        return &self.value;
    }
}

#[test]
fn test_struct_deref() {
    let value = MyValue{value: 10};
    println!("{}", *value);
}

fn say_holla(name: &String) {
    println!("Holla {}", name);
}

#[test]
fn test_deref() {
    let name = MyValue{
        value: String::from("Dyaksa"),
    };
    say_holla(&name);
}

struct Book {
    title: String,
}

impl Drop for Book {
    fn drop(&mut self) {
        println!("Dropping book {}", self.title);
    }
}

#[test]
fn test_drop() {
    let book = Book{title: String::from("Rust Programming")};
    println!("Book title {}", book.title);
}

#[derive(Debug)]
enum Brand {
    Of(String, Rc<Brand>),
    End
}

#[test]
fn multiple_ownership() {
    let apple: Rc<Brand> = Rc::new(Brand::Of("Apple".to_string(), Rc::new(Brand::End)));
    println!("{}", Rc::strong_count(&apple));

    let laptop = Brand::Of("Laptop".to_string(), Rc::clone(&apple));
    println!("{}", Rc::strong_count(&apple));

    {
        let smartphone = Brand::Of("Smartphone".to_string(), Rc::clone(&apple));
        println!("{}", Rc::strong_count(&apple));
    }

    println!("{}", Rc::strong_count(&apple));

    // let apple = ProductCategory::Of("Apple".to_string(), Box::new(ProductCategory::End));
    // let laptop = ProductCategory::Of("Laptop".to_string(), Box::new(apple));
    // let smartphone = ProductCategory::Of("Smartphone".to_string(), Box::new(apple));
}

#[derive(Debug)]
struct Seller {
    name: RefCell<String>,
    active: RefCell<bool>,
}


#[test]
fn test_ref_cell() {
    let seller = Seller{
        name: RefCell::new("Dyaksa".to_string()),
        active: RefCell::new(true),
    };

    {
        let mut result = seller.name.borrow_mut();
        *result = "Jauharuddin".to_string();
    }

    {
        let mut active = seller.active.borrow_mut();
        *active = false;
    }

    println!("{:?}", seller);

    // seller.name = RefCell::new("Anggiani".to_string());
}

static APPLICATION: &str = "My Application";

#[test]
fn test_static() {
    println!("{}", APPLICATION);
}


static mut COUNTER: i32 = 0;

fn incerement() {
    unsafe {
        COUNTER += 1;
    };
}

#[test]
fn test_unsafe() {
    unsafe {
        incerement();
        COUNTER += 1;
        println!("{}", COUNTER);
    }
}

#[warn(unused_macros)]
macro_rules! say_hello {
    () => {
        println!("Hello");
    };

    ($name: expr) => {
        println!("Hello {}", $name);
    };
}

#[test]
fn test_macro() {
    say_hello!();
    say_hello!("Dyaksa Jauharuddin");
}


#[warn(unused_macros)]
macro_rules! iterate {
    ($array: expr) => {
        for value in $array {
            println!("{}", value);
        }
    };

    ($($item: expr), *) => {
        $(
            println!("{}", $item);
        )*
    }
}

#[test]
fn test_macro_iterate() {
    let array = [1, 2, 3, 4, 5];
    iterate!(array);

    iterate!(1, 2, 3, 4, 5);
}
