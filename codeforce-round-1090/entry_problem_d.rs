use std::io::{Read, Write, BufWriter};

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();
    let mut next = || it.next().unwrap();
   let t: usize = next().parse().unwrap();

   let mut out = BufWriter::new(std::io::stdout().lock());

  for _ in 0..t {
    let n: u64 = next().parse().unwrap();

    let a: Vec<u64> = (1..=n) 
    .map(|i| (2 * i - 1) * ( 2 * i + 1))
    .collect();
  
   let s = a.iter()
   .map(|x| x.to_string())
   .collect::<Vec<_>>()
   .join(" ");


  
  
  writeln!(out, "{}", s).unwrap(); // apparently write path goes first in a statement??? never knew that lol.
 

   
    out.flush().unwrap();
  }
    
}

