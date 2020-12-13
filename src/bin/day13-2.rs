use ring_algorithm::chinese_remainder_theorem;
use std::fs::read_to_string;
fn main() {
    let contents = read_to_string("data/day13input").unwrap();
    let mut lines = contents.split('\n');
    lines.next(); // skip line
    let mut r = Vec::<i64>::new();
    let mut m = Vec::<i64>::new();
    lines
        .next()
        .unwrap()
        .split(',')
        .enumerate()
        .filter(|(_, x)| x != &"x")
        .for_each(|(i, x)| {
            r.push(i as i64);
            m.push(x.parse::<i64>().unwrap());
        });

    println!("{:?}", r);
    println!("{:?}", m);

    let a = chinese_remainder_theorem::<i64>(&r, &m).unwrap();
    println!("{}", a);
}
