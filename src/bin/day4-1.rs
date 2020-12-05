use std::collections::HashSet;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let count = read_to_string("data/day4input")?
        .split("\n\n")
        .filter(|passport| {
            let parts = passport.split(|c| ['\n', ' '].contains(&c));
            let keys: HashSet<&str> = parts.map(|kv| kv.split(':').next().unwrap()).collect();
            let required: HashSet<&str> = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
                .iter()
                .cloned()
                .collect();
            required.is_subset(&keys)
        })
        .count();
    println!("{}", count);
    Ok(())
}
