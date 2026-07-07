use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();
    let mut next = || it.next().unwrap();
   let t: usize = next().parse().unwrap();

    for i in 0..t {
    let mut best: i64 = i64::MIN;
   let mut sum: i64 = 0;
  let mut container: Vec<i64> = Vec::new();
   for _ in 0..7 {
    let a: i64 = next().parse().unwrap();
    container.push(a);


   }
  container.sort();
  let rest: i64 = container[0..=5].iter().sum();
  println!("{}", container[6] - rest);
  
    }
    
}
