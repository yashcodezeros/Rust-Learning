extern crate rand;
extern crate phrases;
use rand::Rng;
use phrases::greetings::french;

pub fn run(){
    let mut rng = rand::thread_rng();
    let b:bool = rng.gen();

    //run the own crate/dependency fn with "cargo build && cargo run" in main project

    println!("English: {} {}" ,phrases::greetings::english::hello(),phrases::greetings::english::good_bye());
    println!("French: {} {}" ,french::hello(),french::good_bye());
}