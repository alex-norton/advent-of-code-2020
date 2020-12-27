use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut candidates = HashMap::<&str, HashSet<&str>>::new();
    let file = include_str!("../../data/day21input");
    for line in file.lines() {
        // remove final )
        let line = &line[..line.len() - 1];
        let mut sections = line.split(" (contains ");
        let mut ingredients = HashSet::new();
        for ingredient in sections.next().unwrap().split(' ') {
            ingredients.insert(ingredient);
        }
        for allergen in sections.next().unwrap().split(", ") {
            if let Some(s) = candidates.get(allergen) {
                let intersection = s.intersection(&ingredients).copied().collect();
                candidates.insert(allergen, intersection);
            } else {
                candidates.insert(allergen, ingredients.clone());
            }
        }
    }

    let mut mapping: Vec<(&str, &str)> = Vec::new();
    while candidates.iter().any(|(_, v)| !v.is_empty()) {
        let (allergen, set) = candidates.iter().find(|(_, v)| v.len() == 1).unwrap();
        let ingredient = set.iter().next().unwrap().clone();
        mapping.push((allergen, ingredient));
        candidates.iter_mut().for_each(|(_, set)| {
            set.remove(ingredient);
        });
    }

    mapping.sort();
    let list = mapping.iter().map(|(_, v)| v).join(",");

    println!("{:?}", list);

    Ok(())
}
