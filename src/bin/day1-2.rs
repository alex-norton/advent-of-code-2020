use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut nums: Vec<i32> = Vec::new();
    if let Ok(lines) = read_lines("data/day1input") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                let num = ip.parse().unwrap();
                nums.push(num);
            }
        }
    }
    for num1 in &nums {
        for num2 in &nums {
            for num3 in &nums {
                if num1 + num2 + num3 == 2020 {
                    println!("{}, {}, {}", num1, num2, num3)
                    // found 780, 542, 698
                }
            }
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
