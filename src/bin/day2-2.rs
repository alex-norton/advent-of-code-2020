use advent::read_lines;
use regex::Regex;

fn main() {
    let re = Regex::new(r"(\d+)-(\d+) ([a-z]): ([a-z]+)").unwrap();
    let mut count = 0;

    for line in read_lines("data/day2input") {
        if let Ok(l) = line {
            let cap = re.captures(&l).unwrap();
            let (left, right, letter, word) = (
                (&cap[1]).parse::<usize>().unwrap(),
                (&cap[2]).parse::<usize>().unwrap(),
                &cap[3],
                &cap[4],
            );
            let chars: Vec<String> = word.chars().map(|c| c.to_string()).collect();
            let left_found = match chars.get(left - 1) {
                Some(c) => c == letter,
                None => false,
            };
            let right_found = match chars.get(right - 1) {
                Some(c) => c == letter,
                None => false,
            };
            if left_found ^ right_found {
                count += 1
            }
        }
    }
    println!("{}", count)
}
