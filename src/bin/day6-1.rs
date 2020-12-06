use std::collections::HashSet;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let count = read_to_string("data/day6input")?
        .split("\n\n")
        .map(|group| {
            group
                .chars()
                .filter(|&c| c != '\n')
                .collect::<HashSet<char>>()
                .len()
        })
        .sum::<usize>();
    println!("{}", count);
    Ok(())
}
