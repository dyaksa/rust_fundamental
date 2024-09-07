pub mod outer_mod {

    pub mod inner_mod {
        // pub(in crate::outer::outer_mod) fn say_hello() {
        //     println!("Hello Rust pub in crate::outer::outer_mod");
        // }

        pub(super) fn say_hello() {
            println!("Hello Rust pub(super)");
        }
    }

    pub fn greetings() {
        inner_mod::say_hello();
    }
}