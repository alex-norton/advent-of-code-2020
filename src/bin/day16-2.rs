use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let field_re = Regex::new(r"([\w ]*):")?;
    let range_re = Regex::new(r"([0-9]+)-([0-9]+)")?;
    let input = read_to_string("data/day16input")?;
    let mut sections = input.split("\n\n");

    let mut ranges = HashSet::<(usize, usize)>::new();
    let mut fields = HashMap::<String, HashSet<(usize, usize)>>::new();
    for line in sections.next().unwrap().split('\n') {
        let field = field_re.captures(line).unwrap()[1].to_string();
        let mut field_ranges = HashSet::<(usize, usize)>::new();

        for cap in range_re.captures_iter(line) {
            let low = cap[1].parse::<usize>().unwrap();
            let high = cap[2].parse::<usize>().unwrap();
            ranges.insert((low, high));
            field_ranges.insert((low, high));
        }
        fields.insert(field, field_ranges);
    }

    // lol
    let my_ticket = sections
        .next()
        .unwrap()
        .split('\n')
        .skip(1)
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect::<Vec<usize>>();

    // Possible fields for each position
    let mut candidates = vec![fields.keys().collect::<HashSet<&String>>(); my_ticket.len()];
    sections
        .next()
        .unwrap()
        .split('\n')
        .skip(1)
        .map(|l| {
            l.split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .filter(|v| {
            v.iter()
                .all(|n| ranges.iter().any(|(l, u)| l <= n && n <= u))
        })
        .for_each(|v| {
            for (i, n) in v.iter().enumerate() {
                candidates[i].retain(|c| {
                    fields
                        .get(&c.to_string())
                        .unwrap()
                        .iter()
                        .any(|(l, u)| l <= n && n <= u)
                });
            }
        });

    // At this point, there are some positions that only have a single candidate remaining.
    // Extract those into a final pos -> field mapping, and remove from other candidate lists.
    let mut mapping = vec![String::new(); my_ticket.len()];
    while candidates.iter().any(|c| !c.is_empty()) {
        let (i, c) = candidates
            .iter()
            .enumerate()
            .find(|(_, c)| c.len() == 1)
            .unwrap();
        mapping[i] = c.iter().next().unwrap().to_string();
        candidates.iter_mut().for_each(|c| {
            c.remove(&mapping[i]);
        });
    }

    let product = mapping
        .iter()
        .enumerate()
        .filter(|(_, s)| s.starts_with("departure"))
        .map(|(i, _)| my_ticket[i])
        .product::<usize>();
    println!("{}", product);

    Ok(())
}
