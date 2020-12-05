#[macro_use]
extern crate lazy_static;
use regex::Regex;
use std::collections::HashSet;
use std::fs::read_to_string;

// This accepts one more passport than it's supposed to!
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let count = read_to_string("data/day4input")?
        .split("\n\n")
        .filter(|passport| {
            let parts = passport.split(|c| ['\n', ' '].contains(&c));
            let kvs: Vec<(&str, &str)> = parts
                .map(|kv| {
                    let mut split = kv.split(':');
                    (split.next().unwrap(), split.next().unwrap())
                })
                .collect();
            let keys: HashSet<&str> = kvs.iter().map(|kv| kv.0).collect();
            lazy_static! {
                static ref REQUIRED: HashSet<&'static str> =
                    ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
                        .iter()
                        .cloned()
                        .collect();
            }
            if !REQUIRED.is_subset(&keys) {
                return false;
            }
            kvs.iter().all(|(k, v)| match *k {
                "byr" => match v.parse::<usize>() {
                    Ok(i) => 1920 <= i && i <= 2002,
                    Err(_) => false,
                },
                "iyr" => match v.parse::<usize>() {
                    Ok(i) => 2010 <= i && i <= 2020,
                    Err(_) => false,
                },
                "eyr" => match v.parse::<usize>() {
                    Ok(i) => 2020 <= i && i <= 2030,
                    Err(_) => false,
                },
                "hgt" => {
                    lazy_static! {
                        static ref HGT: Regex = Regex::new(r"(\d+)(cm|in)").unwrap();
                    }
                    match HGT.captures(v) {
                        Some(cap) => {
                            let i = *&cap[1].parse::<usize>().unwrap();
                            match &cap[2] {
                                "cm" => 150 <= i && i <= 193,
                                _ => 59 <= i && i <= 76,
                            }
                        }
                        None => false,
                    }
                }
                "hcl" => {
                    lazy_static! {
                        static ref HCL: Regex = Regex::new("#[0-9a-f]{6}").unwrap();
                    }
                    HCL.is_match(v)
                }
                "ecl" => {
                    lazy_static! {
                        static ref ECL: Regex = Regex::new("amb|blu|brn|gry|grn|hzl|oth").unwrap();
                    }
                    ECL.is_match(v)
                }
                "pid" => {
                    lazy_static! {
                        static ref PID: Regex = Regex::new("[0-9]{9}").unwrap();
                    }
                    PID.is_match(v)
                }
                _ => true,
            })
        })
        .count();
    println!("{}", count);
    Ok(())
}
