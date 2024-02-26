extern crate rand;
use rand::{rngs::ThreadRng, thread_rng, Rng};

fn main() {
    for number in (1..=3).rev() {
        print!("{number}, ");
    }
    println!("LIFTOFF!!!");
    let mut thr: ThreadRng = thread_rng();
    let randnum: u32 = thr.gen_range(1..11);
    println!("Random number: {}", randnum);
    let fubar = if randnum < 5 {
        Some("less than 5")
    } else {
        None
    };
    match fubar {
        Some(val) => println!("foo is {}", val),
        None => println!("foo is None"),
    }
}
