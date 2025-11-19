use rand::{prelude::*, random, rng};

fn main() {
    println!("Hello, world!");

    let mut rng = rand::rng();

    let mut numbers: Vec<i32> = (1..100).collect();
    let secret_number = numbers.choose(&mut rng);

    println!("{}", secret_number);
    
}
