// Rang is called crate in Rust, which is a package of Rust code. which is library in python.
use rand::Rng;

fn main() {
    println!("Getting Random number from 1 to 100");
    let random_number: u32 = rand::rng().random_range(1..=100);
    print!("Number is {}", random_number);
}
