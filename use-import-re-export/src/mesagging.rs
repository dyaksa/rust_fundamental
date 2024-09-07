// pub use self::sub_module::say_hello_world as say_hello;
pub use self::sub_module::say_hello_world;

mod sub_module {
    pub fn say_hello_world() {
        println!("Hello, world!");
    }
}