use std::collections::HashSet;
use std::collections::VecDeque;
use std::fs::read_to_string;

type Deck = VecDeque<usize>;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = read_to_string("data/day22input")?;
    let mut lines = file.lines();
    lines.next();
    let mut p1 = Deck::new();
    while let Ok(i) = lines.next().unwrap().parse() {
        p1.push_back(i);
    }
    lines.next();
    let mut p2 = lines.map(|l| l.parse().unwrap()).collect::<Deck>();

    let winner = play(&mut p1, &mut p2);

    let winning_deck = if winner { p1 } else { p2 };
    let score = winning_deck
        .iter()
        .rev()
        .enumerate()
        .map(|(i, v)| (i + 1) * v)
        .sum::<usize>();
    println!("{}", score);

    Ok(())
}

// True iff p1 wins
fn play(p1: &mut Deck, p2: &mut Deck) -> bool {
    let mut seen = HashSet::<(Deck, Deck)>::new();
    seen.insert((p1.clone(), p2.clone()));

    while !p1.is_empty() && !p2.is_empty() {
        let one = p1.pop_front().unwrap();
        let two = p2.pop_front().unwrap();
        let decks = (p1.clone(), p2.clone());
        if seen.contains(&decks) {
            return true;
        } else {
            seen.insert(decks);
        }
        let winner;
        if p1.len() >= one && p2.len() >= two {
            let mut new_p1 = p1.iter().take(one).cloned().collect::<Deck>();
            let mut new_p2 = p2.iter().take(two).cloned().collect::<Deck>();
            winner = play(&mut new_p1, &mut new_p2);
        } else if one > two {
            winner = true;
        } else {
            winner = false;
        }
        if winner {
            p1.push_back(one);
            p1.push_back(two);
        } else {
            p2.push_back(two);
            p2.push_back(one);
        }
    }
    p2.is_empty()
}
