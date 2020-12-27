use std::collections::HashMap;
use std::collections::HashSet;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut all_ingredients = HashSet::<&str>::new();
    let mut ingredient_lines = Vec::<Vec<&str>>::new();
    let mut candidates = HashMap::<&str, HashSet<&str>>::new();
    let file = include_str!("../../data/day21input");
    for line in file.lines() {
        // remove final )
        let line = &line[..line.len() - 1];
        let mut sections = line.split(" (contains ");
        let mut ingredients = HashSet::new();
        let mut ingredient_line = Vec::new();
        for ingredient in sections.next().unwrap().split(' ') {
            all_ingredients.insert(ingredient);
            ingredients.insert(ingredient);
            ingredient_line.push(ingredient);
        }
        ingredient_lines.push(ingredient_line);
        for allergen in sections.next().unwrap().split(", ") {
            if let Some(s) = candidates.get(allergen) {
                let intersection = s.intersection(&ingredients).copied().collect();
                candidates.insert(allergen, intersection);
            } else {
                candidates.insert(allergen, ingredients.clone());
            }
        }
    }

    for (_, v) in &candidates {
        all_ingredients = all_ingredients.difference(v).copied().collect();
    }

    let count: usize = ingredient_lines
        .iter()
        .map(|v| v.iter().filter(|&x| all_ingredients.contains(x)).count())
        .sum();

    println!("{}", count);

    Ok(())
}
