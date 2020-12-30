use itertools::Itertools;
use std::collections::HashMap;
use std::fs::read_to_string;

const MS: usize = 1000000;
const MI: u32 = 1000000;
const I: usize = 10000000;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = vec![0; MS];
    read_to_string("data/day23input")?
        .chars()
        .enumerate()
        .for_each(|(i, c)| input[i] = c.to_digit(10).unwrap());
    for i in 10..=MS {
        input[i - 1] = i as u32;
    }
    let mut map = HashMap::<u32, u32>::with_capacity(MS);
    for (a, b) in input.iter().tuple_windows() {
        map.insert(*a, *b);
    }
    map.insert(input[MS - 1], input[0]);

    let mut cur = input[0];

    for _ in 0..I {
        let one = map[&cur];
        let two = map[&one];
        let three = map[&two];
        let target = find_target(cur, &[one, two, three]);
        let after_three = map[&three];
        let after_target = map[&target];

        map.insert(cur, after_three);
        map.insert(target, one);
        map.insert(three, after_target);

        cur = map[&cur];
    }

    let first = map[&1];
    let second = map[&first];
    println!("{}", (first as usize) * (second as usize));

    Ok(())
}

fn find_target(start: u32, avoid: &[u32]) -> u32 {
    let mut ret = start;
    if ret == 1 {
        ret = MI;
    } else {
        ret -= 1;
    }
    while let Some(_) = avoid.iter().find(|&&x| x == ret) {
        if ret == 1 {
            ret = MI;
        } else {
            ret -= 1;
        }
    }
    ret
}
