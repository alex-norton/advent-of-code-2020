use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mask_re = Regex::new(r"mask = ([01X]{36})")?;
    let mem_re = Regex::new(r"mem\[(\d+)\] = (\d+)")?;

    let mut ones = HashSet::<usize>::new();
    let mut xs = HashSet::<usize>::new();
    let mut mem = HashMap::<usize, usize>::new();
    for line in read_to_string("data/day14input")?.split('\n') {
        if let Some(cap) = mask_re.captures(line) {
            ones = HashSet::<usize>::new();
            xs = HashSet::<usize>::new();
            cap[1].chars().enumerate().for_each(|(i, c)| {
                if c == '1' {
                    ones.insert(i);
                } else if c == 'X' {
                    xs.insert(i);
                }
            });
        } else if let Some(cap) = mem_re.captures(line) {
            let mut adr_bits = to_bits(cap[1].parse::<usize>().unwrap());
            let val = cap[2].parse::<usize>().unwrap();
            ones.iter().for_each(|i| adr_bits[*i] = '1');
            for i in 0..2_usize.pow(xs.len() as u32) {
                let i_bits = to_n_bits(i, xs.len());
                let mut new_adr = adr_bits.clone();
                // assumes that the arbitrary set iterator will use the same order for any fixed value of xs
                xs.iter().enumerate().for_each(|(i, v)| {
                    new_adr[*v] = i_bits[i];
                });
                mem.insert(from_bits(&new_adr), val);
            }
        }
    }

    let sum = mem.iter().map(|(_, v)| v).sum::<usize>();
    println!("{}", sum);
    Ok(())
}

fn to_bits(x: usize) -> Vec<char> {
    to_n_bits(x, 36)
}

fn to_n_bits(x: usize, n: usize) -> Vec<char> {
    let mut bits = format!("{:b}", x).chars().collect::<Vec<char>>();
    let mut full_bits = vec!['0'; n - bits.len()];
    full_bits.append(&mut bits);
    full_bits
}

fn from_bits(b: &Vec<char>) -> usize {
    usize::from_str_radix(&b.iter().collect::<String>(), 2).unwrap()
}
