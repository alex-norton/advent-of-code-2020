use std::collections::HashMap;
use std::fs::read_to_string;

// https://brilliant.org/wiki/recursive-backtracking/ to structure backtracking
// total tiles = 144 -> 12 by 12 solution
// also, no tile has the same edge on multiple places (thankfully)
type Edge = Vec<char>;
#[derive(Debug)]
struct Tile {
    id: usize,
    top: Edge,
    right: Edge,
    bottom: Edge,
    left: Edge,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // stores all tile variants for each id
    let mut tiles = HashMap::<usize, Vec<Tile>>::new();
    read_to_string("data/test")?.split("\n\n").for_each(|s| {
        let mut lines = s.lines();
        let id_line = lines.next().unwrap();
        let start = id_line.find(' ').unwrap();
        let end = id_line.find(':').unwrap();
        let id = id_line[start + 1..end].parse::<usize>().unwrap();

        let data = lines
            .map(|l| l.chars().collect::<Vec<char>>())
            .collect::<Vec<_>>();
        let height = data.len();
        let width = data[0].len();

        let top: Edge = (0..width).map(|i| data[0][i]).collect();
        let top_rev: Edge = (0..width).rev().map(|i| data[0][i]).collect();
        let right: Edge = (0..height).map(|i| data[i][width - 1]).collect();
        let right_rev: Edge = (0..height).rev().map(|i| data[i][width - 1]).collect();
        let bottom: Edge = (0..width).map(|i| data[height - 1][i]).collect();
        let bottom_rev: Edge = (0..width).rev().map(|i| data[height - 1][i]).collect();
        let left: Edge = (0..height).map(|i| data[i][0]).collect();
        let left_rev: Edge = (0..height).rev().map(|i| data[i][0]).collect();
        for (t, r, b, l) in &[
            (&top, &right, &bottom, &left),
            (&left_rev, &top, &right_rev, &bottom),
            (&bottom_rev, &left_rev, &top_rev, &right_rev),
            (&right, &bottom_rev, &left, &top_rev),
            (&top_rev, &left, &bottom_rev, &right),
            (&left, &bottom, &right, &top),
            (&bottom, &right_rev, &top, &left_rev),
            (&right_rev, &top_rev, &left_rev, &bottom_rev),
        ] {
            let tile = Tile {
                id: id,
                top: (*t).clone(),
                right: (*r).clone(),
                bottom: (*b).clone(),
                left: (*l).clone(),
            };
            let entry = tiles.entry(id);
            entry.or_insert_with(Vec::new).push(tile);
        }
    });
    // we fill puzzle left to right, top to bottom
    // so at any given insertion, we only need to match with
    // new tiles left edge and top edge
    let mut lefts = HashMap::<Edge, Vec<&Tile>>::new();
    let mut tops = HashMap::<Edge, Vec<&Tile>>::new();
    for (_, v) in &tiles {
        for t in v {
            match lefts.get_mut(&t.left) {
                Some(v) => v.push(t),
                None => {
                    lefts.insert(t.left.clone(), vec![t]);
                }
            }
            match tops.get_mut(&t.top) {
                Some(v) => v.push(t),
                None => {
                    tops.insert(t.top.clone(), vec![t]);
                }
            }
        }
    }
    Ok(())
}
