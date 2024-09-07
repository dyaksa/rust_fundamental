pub mod outer_mod {
    pub mod inner_mod {
        pub(self) fn say_hello() {
            println!("Hello Rust pub(self)");
        }

        pub fn greet() {
            say_hello();
        }
    }

    pub fn greetings() {
        inner_mod::greet();
    }
}