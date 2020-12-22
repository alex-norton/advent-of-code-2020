use std::collections::HashMap;
use std::collections::HashSet;

/*
Hardcode:
8: 42 | 42 8
11: 42 31 | 42 11 31
with special handling

note that 42 and 31 always match to length 8
rule 42: length 8
rule 31: length 8
*/
enum Rule {
    Atom(&'static str),
    Branch(Vec<Vec<u32>>),
    Eight,
    Eleven,
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

    // Override input with custom handling
    rules.insert(8, Rule::Eight);
    // rules.insert(11, Rule::Eleven);

    let count = sections
        .next()
        .unwrap()
        .split('\n')
        .filter(|s| {
            check(
                &rules,
                0,
                vec![*s].into_iter().collect::<HashSet<&'static str>>(),
            )
            .iter()
            .any(|t| *t == "")
        })
        .count();

    println!("{}", count);
    Ok(())
}

// On successful match, returns the remaining unmatched portion of the str.
fn check(rules: &Rules, index: u32, targets: HashSet<&'static str>) -> HashSet<&'static str> {
    let rule = rules.get(&index).unwrap();
    let mut ret = HashSet::<&'static str>::new();
    for target in targets {
        match rule {
            Rule::Atom(s) => {
                if target.starts_with(s) {
                    ret.insert(&target[s.len()..]);
                }
            }
            Rule::Branch(groups) => {
                for group in groups {
                    let mut rem = vec![target].into_iter().collect::<HashSet<&'static str>>();
                    for i in group {
                        rem = check(rules, *i, rem);
                    }
                    rem.iter().for_each(|r| {
                        ret.insert(r);
                    });
                }
            }
            Rule::Eight => {
                let mut rem = vec![target].into_iter().collect::<HashSet<&'static str>>();
                while rem.len() > 0 {
                    rem = check(rules, 42, rem);
                    rem.iter().for_each(|r| {
                        ret.insert(r);
                    })
                }
            }
            Rule::Eleven => {}
        }
    }
    ret
}
