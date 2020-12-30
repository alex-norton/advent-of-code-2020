use std::collections::HashSet;
use std::fs::read_to_string;

type Coord = (i32, i32, i32);
const E: Coord = (1, 0, -1);
const SE: Coord = (0, 1, -1);
const SW: Coord = (-1, 1, 0);
const W: Coord = (-1, 0, 1);
const NW: Coord = (0, -1, 1);
const NE: Coord = (1, -1, 0);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = read_to_string("data/day24input")?;
    let mut flipped = HashSet::<Coord>::new();
    for s in file.lines() {
        let coord = parse(&s);
        if flipped.contains(&coord) {
            flipped.remove(&coord);
        } else {
            flipped.insert(coord);
        }
    }
    println!("{}", flipped.len());
    Ok(())
}

fn parse(s: &str) -> Coord {
    let mut ret = (0, 0, 0);
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
    (x.0 + y.0, x.1 + y.1, x.2 + y.2)
}
