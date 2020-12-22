use std::collections::HashMap;

enum Rule {
    Atom(&'static str),
    Branch(Vec<Vec<u32>>),
}

type Rules = HashMap<u32, Rule>;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // static string
    let file = include_str!("../../data/day19input");
    let mut sections = file.split("\n\n");

    let mut rules = Rules::new();
    for line in sections.next().unwrap().split('\n') {
        let mut halves = line.split(':');
        let no = halves.next().unwrap().parse::<u32>()?;
        let rule_str = halves.next().unwrap();
        if let Some(start) = rule_str.find('"') {
            let end = rule_str[start + 1..].find('"').unwrap();
            rules.insert(no, Rule::Atom(&rule_str[start + 1..start + 1 + end]));
        } else {
            let rule: Vec<Vec<u32>> = rule_str
                .split('|')
                .map(|s| s.split(' ').filter_map(|n| n.parse::<u32>().ok()).collect())
                .collect();
            rules.insert(no, Rule::Branch(rule));
        }
    }

    let count = sections
        .next()
        .unwrap()
        .split('\n')
        .filter(|s| match check(&rules, 0, s) {
            Some("") => true,
            _ => false,
        })
        .count();

    println!("{}", count);
    Ok(())
}

// On successful match, returns the remaining unmatched portion of the str.
fn check(rules: &Rules, index: u32, target: &'static str) -> Option<&'static str> {
    let rule = rules.get(&index).unwrap();
    match rule {
        Rule::Atom(s) => match target.starts_with(s) {
            true => Some(&target[s.len()..]),
            false => None,
        },
        Rule::Branch(groups) => {
            for group in groups {
                // Assume that they are kind, and that if a string matches many rule groups,
                // it will always have the same length match, and so we don't have to backtrack.
                let mut rem = Some(target);
                for i in group {
                    if let Some(r) = rem {
                        rem = check(rules, *i, r);
                    }
                }
                match rem {
                    None => continue,
                    Some(_) => return rem,
                }
            }
            None
        }
    }
}
