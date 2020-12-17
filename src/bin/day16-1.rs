use regex::Regex;
use std::collections::HashSet;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let range_re = Regex::new(r"([0-9]+)-([0-9]+)")?;
    let input = read_to_string("data/day16input")?;
    let mut sections = input.split("\n\n");

    let mut ranges = HashSet::<(usize, usize)>::new();

    // fields
    for line in sections.next().unwrap().split('\n') {
        for cap in range_re.captures_iter(line) {
            ranges.insert((
                cap[1].parse::<usize>().unwrap(),
                cap[2].parse::<usize>().unwrap(),
            ));
        }
    }

    // Skip your input
    sections.next();

    let sum = sections
        .next()
        .unwrap()
        .split(&['\n', ','][..])
        .skip(1)
        .map(|s| s.parse::<usize>().unwrap())
        .filter(|n| ranges.iter().all(|(l, u)| n < l || n > u))
        .sum::<usize>();
    println!("{}", sum);

    Ok(())
}
