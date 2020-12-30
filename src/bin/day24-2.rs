use std::cmp;
use std::collections::HashSet;
use std::fs::read_to_string;

// I changed to axial coordinates even though cube coordinates are cooler.
// https://www.redblobgames.com/grids/hexagons/#neighbors
// This is more efficient for storage, plus when iterating through my state space
// I don't have to filter out by x + y + z = 0.
type Coord = (i32, i32);
type State = HashSet<Coord>;
const E: Coord = (1, 0);
const SE: Coord = (0, 1);
const SW: Coord = (-1, 1);
const W: Coord = (-1, 0);
const NW: Coord = (0, -1);
const NE: Coord = (1, -1);
const ALL: [Coord; 6] = [E, SE, SW, W, NW, NE];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = read_to_string("data/day24input")?;
    let mut flipped = State::new();
    let (mut max_x, mut max_y) = (0, 0);
    for s in file.lines() {
        let coord = parse(&s);
        if flipped.contains(&coord) {
            flipped.remove(&coord);
        } else {
            max_x = cmp::max(max_x, coord.0.abs() + 1);
            max_y = cmp::max(max_y, coord.1.abs() + 1);
            flipped.insert(coord);
        }
    }

    for _ in 1..=100 {
        let mut new_flipped = flipped.clone();
        for x in -max_x..=max_x {
            for y in -max_y..=max_y {
                let black = flipped.contains(&(x, y));
                let count = count(&flipped, (x, y));
                if black && (count == 0 || count > 2) {
                    new_flipped.remove(&(x, y));
                } else if !black && count == 2 {
                    new_flipped.insert((x, y));
                    max_x = cmp::max(max_x, x.abs() + 1);
                    max_y = cmp::max(max_y, y.abs() + 1);
                }
            }
        }
        flipped = new_flipped;
    }
    println!("{}", flipped.len());
    Ok(())
}

fn count(state: &State, coord: Coord) -> usize {
    let mut sum = 0;
    for &d in ALL.iter() {
        if state.contains(&p(coord, d)) {
            sum += 1;
        }
    }
    sum
}

fn parse(s: &str) -> Coord {
    let mut ret = (0, 0);
    let c = s.chars().collect::<Vec<_>>();
    let mut i = 0;
    while i < c.len() {
        match c[i] {
            'e' => {
                ret = p(ret, E);
                i += 1;
            }
            'w' => {
                ret = p(ret, W);
                i += 1;
            }
            'n' => match c.get(i + 1) {
                Some('e') => {
                    ret = p(ret, NE);
                    i += 2;
                }
                Some('w') => {
                    ret = p(ret, NW);
                    i += 2;
                }
                _ => {
                    i += 1;
                }
            },
            's' => match c.get(i + 1) {
                Some('e') => {
                    ret = p(ret, SE);
                    i += 2;
                }
                Some('w') => {
                    ret = p(ret, SW);
                    i += 2;
                }
                _ => {
                    i += 1;
                }
            },
            _ => panic!(),
        }
    }
    ret
}

fn p(x: Coord, y: Coord) -> Coord {
    (x.0 + y.0, x.1 + y.1)
}
