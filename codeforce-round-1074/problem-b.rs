
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
  let mut counter = 0;
  let mut container: Vec<u64> = Vec::new();

  for i in 0..n {
  let x: u64 = next().parse().unwrap();
  container.push(x);
  }
  container.sort();
  container.reverse();
  counter = container[0] * container.len() as u64;


  writeln!(out, "{}", counter).unwrap();

  out.flush().unwrap();

 }

}
