use std::fs::File;
use std::io::{self, BufRead};

pub fn read_lines(filename: &str) -> io::Lines<io::BufReader<File>> {
  let file = File::open(filename).unwrap();
  io::BufReader::new(file).lines()
}
