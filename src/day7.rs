use std::{time::Instant, fs};

pub fn part_1() {
  let before = Instant::now();
  let contents = fs::read_to_string("inputs/d7_input.txt")
  .unwrap();

  println!("Executed in {:?}", before.elapsed());
}