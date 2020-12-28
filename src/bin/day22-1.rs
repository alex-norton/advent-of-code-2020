use std::collections::VecDeque;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = read_to_string("data/day22input")?;
    let mut lines = file.lines();
    lines.next();
    let mut p1 = VecDeque::<usize>::new();
    while let Ok(i) = lines.next().unwrap().parse::<usize>() {
        p1.push_back(i);
    }
    lines.next();
    let mut p2 = lines
        .map(|l| l.parse::<usize>().unwrap())
        .collect::<VecDeque<usize>>();
    while !p1.is_empty() && !p2.is_empty() {
        let one = p1.pop_front().unwrap();
        let two = p2.pop_front().unwrap();
        if one > two {
            p1.push_back(one);
            p1.push_back(two);
        } else {
            p2.push_back(two);
            p2.push_back(one);
        }
    }
    let winner = if p1.is_empty() { p2 } else { p1 };
    let score = winner
        .iter()
        .rev()
        .enumerate()
        .map(|(i, v)| (i + 1) * v)
        .sum::<usize>();
    println!("{}", score);

    Ok(())
}
