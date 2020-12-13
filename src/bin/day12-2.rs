use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut s_pos = (0, 0);
    let mut w_pos = (10, 1);
    read_to_string("data/day12input")?
        .split('\n')
        .map(|s| (s.chars().next().unwrap(), s[1..].parse::<i32>().unwrap()))
        .for_each(|(c, i)| match (c, i) {
            ('E', d) => w_pos = add(&w_pos, &(d, 0)),
            ('N', d) => w_pos = add(&w_pos, &(0, d)),
            ('W', d) => w_pos = add(&w_pos, &(-d, 0)),
            ('S', d) => w_pos = add(&w_pos, &(0, -d)),
            ('L', d) => w_pos = turn(&w_pos, d),
            ('R', d) => w_pos = turn(&w_pos, 360 - d),
            ('F', d) => s_pos = add_times(&s_pos, &w_pos, d),
            _ => panic!(),
        });
    println!(
        "({}, {}): {}",
        s_pos.0,
        s_pos.1,
        i32::abs(s_pos.0) + i32::abs(s_pos.1)
    );
    Ok(())
}

fn add(x: &(i32, i32), y: &(i32, i32)) -> (i32, i32) {
    add_times(x, y, 1)
}

fn add_times(x: &(i32, i32), y: &(i32, i32), m: i32) -> (i32, i32) {
    (x.0 + m * y.0, x.1 + m * y.1)
}

fn turn(w_pos: &(i32, i32), i: i32) -> (i32, i32) {
    match i {
        0 => (w_pos.0, w_pos.1),
        90 => (-w_pos.1, w_pos.0),
        180 => (-w_pos.0, -w_pos.1),
        270 => (w_pos.1, -w_pos.0),
        _ => panic!(),
    }
}
