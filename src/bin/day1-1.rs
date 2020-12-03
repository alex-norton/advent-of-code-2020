use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut nums: Vec<i32> = Vec::new();
    let mut targets = HashSet::new();
    if let Ok(lines) = read_lines("data/day1input") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                let num = ip.parse().unwrap();
                nums.push(num);
                targets.insert(2020 - num);
            }
        }
    }
    for num in nums {
        if targets.contains(&num) {
            println!("{}, {}", num, 2020 - num)
            // found 1014, 1006 for output 1020084
        }
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
