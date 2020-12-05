use std::cmp;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut max = isize::MIN;
    let mut min = isize::MAX;
    let mut total: isize = 0;
    for line in read_to_string("data/day5input")?.split('\n') {
        let line_str: String = line
            .chars()
            .map(|x| if x == 'F' || x == 'L' { '0' } else { '1' })
            .collect();
        let line_num = isize::from_str_radix(&line_str, 2)?;
        total += line_num;
        max = cmp::max(max, line_num);
        min = cmp::min(min, line_num);
    }
    let expected = (min + max) * (max - min + 1) / 2;
    println!("{}", expected - total);
    Ok(())
}
