use std::collections::HashMap;

/*
rule 42: length 8
rule 31: length 8

rule 8: length 8
rule 11: length 16
*/
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

    for i in 8..9 {
        for j in 0..2_usize.pow(i as u32) {
            let s = to_n_bits(j, i);
            if let Some("") = check(&rules, 31, &s) {
                println!("{}", s);
            }
        }
    }

    Ok(())
}

// On successful match, returns the remaining unmatched portion of the str.
fn check<'a>(rules: &Rules, index: u32, target: &'a str) -> Option<&'a str> {
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
fn to_n_bits(x: usize, n: usize) -> String {
    let mut bits = format!("{:b}", x).chars().collect::<Vec<char>>();
    let mut full_bits = vec!['0'; n - bits.len()];
    full_bits.append(&mut bits);
    full_bits
        .iter()
        .map(|c| match c {
            '0' => 'a',
            _ => 'b',
        })
        .collect()
}
