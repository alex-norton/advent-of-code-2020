use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let contents = read_to_string("data/day13input")?;
    let mut lines = contents.split('\n');
    let target = lines.next().unwrap().parse::<i32>()?;
    let minutes: Vec<i32> = lines
        .next()
        .unwrap()
        .split(',')
        .filter(|x| x != &"x")
        .map(|x| x.parse().unwrap())
        .collect();
    let min = minutes
        .iter()
        .map(|m| {
            let x = target / m;
            let r = target % m;
            match r {
                0 => (0, m),
                _ => (m * x + m - target, m),
            }
        })
        .min()
        .unwrap();
    println!("{}, {}: {}", min.0, min.1, min.0 * min.1);
    Ok(())
}
