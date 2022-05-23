use std::env;

use generand::{consts::*, generate::*};

fn main() {
    //println!("Hello, world!");

    let args: Vec<String> = env::args().collect();
    let n : usize = args[1].parse().unwrap();

    let s = generate(CATS, n);

    println!("{s}");
}
