use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut pos = (0, 0);
    let mut dir = (0, 1);
    read_to_string("data/day12input")?
        .split('\n')
        .map(|s| (s.chars().next().unwrap(), s[1..].parse::<i32>().unwrap()))
        .for_each(|(c, i)| match (c, i) {
            ('N', d) => pos.0 += d,
            ('S', d) => pos.0 -= d,
            ('E', d) => pos.1 += d,
            ('W', d) => pos.1 -= d,
            ('L', d) => dir = turn(&dir, d),
            ('R', d) => dir = turn(&dir, 360 - d),
            ('F', d) => pos = (pos.0 + dir.0 * d, pos.1 + dir.1 * d),
            _ => panic!(),
        });
    println!(
        "({}, {}): {}",
        pos.0,
        pos.1,
        i32::abs(pos.0) + i32::abs(pos.1)
    );
    Ok(())
}

fn turn(x: &(i32, i32), i: i32) -> (i32, i32) {
    let ang = match x {
        (0, 1) => 0,
        (1, 0) => 90,
        (0, -1) => 180,
        (-1, 0) => 270,
        _ => panic!(),
    };
    let new_ang = (ang + i) % 360;
    match new_ang {
        0 => (0, 1),
        90 => (1, 0),
        180 => (0, -1),
        270 => (-1, 0),
        _ => panic!(),
    }
}
