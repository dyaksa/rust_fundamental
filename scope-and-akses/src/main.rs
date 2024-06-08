fn my_func() {
    println!("calling `my_func`");
}

mod module_a {
    pub fn my_func() {
        println!("calling `module_a::my_func`");
    }
}

mod module_b {
    mod submodule_b_one {
        pub fn my_func() {
            println!("calling `module_b::submodule_b_one::my_func`");
        }
    }

    mod submodule_b_two {
        pub fn my_func() {
            println!("calling `module_b::submodule_b_two::my_func`");

            super::submodule_b_one::my_func();
        }
    }
}

pub fn run_all_func() {
    self::my_func();

    my_func();
}

mod my_mod {
    pub fn my_func() {
        println!("calling `my_mod::my_func`");
    }

    pub mod my_submod {
        pub fn my_func() {
            println!("calling `my_mod::my_submod::my_func`");
        }

        pub fn run_the_app() {
            println!("calling `my_mod::run_the_app` with note");
            crate::my_func();
            super::my_func();
            self::my_func();
        }
    }


}

fn main() {
    my_mod::my_submod::run_the_app();
}