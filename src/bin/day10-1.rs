use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut nums: Vec<i64> = read_to_string("data/day10input")?
        .split('\n')
        .map(|s| s.parse::<i64>().unwrap())
        .collect();
    nums.sort_unstable();

    let mut num_ones = 0;
    let mut num_threes = 0;
    let mut last = 0;
    for i in nums {
        if i - last == 1 {
            num_ones += 1;
        } else if i - last == 3 {
            num_threes += 1;
        } else if i - last != 2 {
            panic!("{}, {}", i, last);
        }
        last = i;
    }
    num_threes += 1;
    println!("{}", num_ones * num_threes);
    Ok(())
}
