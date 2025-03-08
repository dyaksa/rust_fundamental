pub fn say_hello(){
    println!("hello first");
}

pub mod second {
    pub mod third {
        pub fn say_hello(){
            super::super::say_hello();
        }
    }
}