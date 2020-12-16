use std::collections::HashMap;
use std::fs::read_to_string;
use std::time::Instant;

// Why is this so slow?
fn main() {
    let now = Instant::now();
    let seed: Vec<usize> = read_to_string("data/day15input")
        .unwrap()
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    let mut seen = HashMap::<usize, usize>::new();
    let mut num = seed[0];
    for i in 1..=2020 {
        if i == 2020 {
            println!("{}", num);
        }
        if i < seed.len() {
            seen.insert(num, i);
            num = seed[i];
        } else if !seen.contains_key(&num) {
            seen.insert(num, i);
            num = 0;
        } else {
            let new_num = i - seen.get(&num).unwrap();
            seen.insert(num, i);
            num = new_num;
        }
    }
    println!("{}", now.elapsed().as_secs());
}
