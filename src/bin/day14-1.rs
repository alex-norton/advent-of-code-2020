use regex::Regex;
use std::collections::HashMap;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mask_re = Regex::new(r"mask = ([01X]{36})")?;
    let mem_re = Regex::new(r"mem\[(\d+)\] = (\d+)")?;

    let mut mask = HashMap::<usize, char>::new();
    let mut mem = HashMap::<usize, Vec<char>>::new();
    for line in read_to_string("data/day14input")?.split('\n') {
        if let Some(cap) = mask_re.captures(line) {
            mask = cap[1]
                .chars()
                .enumerate()
                .filter(|(_, c)| *c != 'X')
                .collect();
        } else if let Some(cap) = mem_re.captures(line) {
            let adr = cap[1].parse::<usize>().unwrap();
            let mut bits = format!("{:036b}", cap[2].parse::<usize>().unwrap())
                .chars()
                .collect::<Vec<char>>();
            mask.iter().for_each(|(k, v)| bits[*k] = *v);
            mem.insert(adr, bits);
        }
    }

    let sum = mem
        .iter()
        .map(|(_, v)| usize::from_str_radix(&v.iter().collect::<String>(), 2).unwrap())
        .sum::<usize>();
    println!("{}", sum);
    Ok(())
}
