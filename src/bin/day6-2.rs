use std::collections::HashSet;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let count = read_to_string("data/day6input")?
        .split("\n\n")
        .map(|group| {
            let mut sets = group
                .split('\n')
                .map(|person| person.chars().collect::<HashSet<char>>());
            let first = sets.next().unwrap();
            sets.fold(first, |a, x| a.intersection(&x).cloned().collect())
                .len()
        })
        .sum::<usize>();
    println!("{}", count);
    Ok(())
}
