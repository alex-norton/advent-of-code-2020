use std::fs::read_to_string;

fn main() {
    let max = read_to_string("../../data/day5input")
        .unwrap()
        .split('\n')
        .map(|line| {
            let line_str: String = line
                .chars()
                .map(|x| if x == 'F' || x == 'L' { '0' } else { '1' })
                .collect();
            usize::from_str_radix(&line_str, 2).unwrap()
        })
        .max()
        .unwrap();
    println!("{}", max);
}
