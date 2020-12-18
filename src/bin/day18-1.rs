use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sum = read_to_string("data/day18input")?
        .lines()
        .map(|x| calculate(&mut x.chars()))
        .sum::<u64>();
    println!("{}", sum);
    Ok(())
}

fn calculate(cs: &mut std::str::Chars) -> u64 {
    let mut total: u64 = 0;
    let mut op: Option<char> = None;
    while let Some(c) = cs.next() {
        match c {
            ')' => return total,
            ' ' => continue,
            '+' => op = Some('+'),
            '*' => op = Some('*'),
            c => {
                let num = match c {
                    '(' => calculate(cs),
                    c => c.to_digit(10).unwrap() as u64,
                };
                match op {
                    None => total = num,
                    Some('+') => total += num,
                    Some('*') => total *= num,
                    _ => panic!(),
                }
            }
        }
    }
    total
}
