
use std::io::{Read, Write, BufWriter};

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();
    let mut next = || it.next().unwrap();
   let t: usize = next().parse().unwrap();

   let mut out = BufWriter::new(std::io::stdout().lock());



   for _ in 0..t {
  let n: usize = next().parse().unwrap();
  for d in 0..n {
    write!(out, "{} {} {} ", d + 1, n + 1 + 2 * d, n + 2 + 2 * d).unwrap();
  }
   writeln!(out).unwrap();

   }

   
   
   
    
    
}


