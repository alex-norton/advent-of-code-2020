use std::collections::HashSet;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    //let count = read_to_string("data/day4input")?
    let contents = include_str!("day4input");
    println!("{}", contents);
    let count = contents
        .split("\n\n")
        .filter(|passport| {
            let parts = passport.split(|c| ['\n', ' '].contains(&c));
            let keys: HashSet<&str> = parts.map(|kv| kv.split(':').next().unwrap()).collect();
            let required: HashSet<&str> = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
                .iter()
                .cloned()
                .collect();
            println!("{} : {:?}", required.is_subset(&keys), keys);
            required.is_subset(&keys)
        })
        .count();
    println!("{}", count);
    //.map(|x| x.split(|c: char| ['\n', ' '].contains(&c)).collect::<Vec<&str>>())
    Ok(())
}
