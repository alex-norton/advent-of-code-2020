use std::fs::File;
use std::io::{self, BufRead};

// Belated note: I thought I'd be accumulating useful snippets as I went.
// That was not the case! Even this was unnecessary once I found the right
// standard lib function.

pub fn read_lines(filename: &str) -> io::Lines<io::BufReader<File>> {
  let file = File::open(filename).unwrap();
  io::BufReader::new(file).lines()
}
