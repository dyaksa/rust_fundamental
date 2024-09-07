mod messaging;
mod outer;
mod sprs;

/* keyword pub(create) */

pub mod outer_mod_one {
    pub mod inner_mod_one {
        pub(crate) fn say_hello() {
            println!("Hello Rust pub(crate)");
        }
    }

    pub fn greetings() {
        inner_mod_one::say_hello();
    }
}

pub mod outer_mod_two {
    pub fn do_something() {
        crate::outer_mod_one::inner_mod_one::say_hello();
    }
}

fn main() {
    messaging::say_hello();
    messaging::some_black_magic();
    outer::outer_mod::greetings();

    outer_mod_one::greetings();
    outer_mod_two::do_something();

    sprs::outer_mod::greetings();
}
