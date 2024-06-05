use rand::Rng;

fn main() {
    for i in 0..=10 {
        println!("random number {}: {}", i, generate_random_number());
    }
}

fn generate_random_number() -> i32 {
    rand::thread_rng().gen_range(1..100)
}