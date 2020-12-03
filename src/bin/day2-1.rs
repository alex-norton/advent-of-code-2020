use advent::read_lines;
use regex::Regex;

fn main() {
    let re = Regex::new(r"(\d+)-(\d+) ([a-z]): ([a-z]+)").unwrap();
    let mut count = 0;

    for line in read_lines("data/day2input") {
        if let Ok(l) = line {
            let cap = re.captures(&l).unwrap();
            let (min, max, letter, word) = (
                (&cap[1]).parse().unwrap(),
                (&cap[2]).parse().unwrap(),
                &cap[3],
                &cap[4],
            );
            let num = word.chars().filter(|c| c.to_string() == letter).count();
            if num >= min && num <= max {
                count += 1;
            }
        }
    }
    println!("{}", count)
}
