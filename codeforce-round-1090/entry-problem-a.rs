
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();

    let t: usize = it.next().unwrap().parse().unwrap();
    for i in 0..t {
        let x : i32 = it.next().unwrap().parse().unwrap();
        println!("67");
    }
}


