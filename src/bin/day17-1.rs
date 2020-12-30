use std::cmp;
use std::collections::HashMap;
use std::fs::read_to_string;

// Keep a HashSet of (i64, i64, i64) to boolean as board state, and clone it every time.
// Biggest difference from previous Conway: how to determine how much of space to check?
// Simplest: dimensions can grow by at most one per time step.
// Start with 1 + start dims and check negative to positive on all of them.
// i.e. if starting with a 3x1 grid, we start with 4, 2, 1 and check
// -4..=4, -2..=2, -1..=1 at first evolution, -5..=5, -3..=3, -2..=2 at second.
// to check final active count, iterate through all entries.
type Coord = (isize, isize, isize);
// in hindsight, this should just be a HashSet
type State = HashMap<Coord, bool>;

fn count(state: &State, coord: Coord) -> usize {
    let (x, y, z) = coord;
    let mut sum = 0;
    for dx in -1..=1 {
        for dy in -1..=1 {
            for dz in -1..=1 {
                if (dx, dy, dz) == (0, 0, 0) {
                    continue;
                }
                if *state.get(&(x + dx, y + dy, z + dz)).unwrap_or(&false) {
                    sum += 1;
                }
            }
        }
    }
    sum
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut state = State::new();
    let (mut max_x, mut max_y, mut max_z) = (0_isize, 0_isize, 1_isize);
    for (i, line) in read_to_string("data/day17input")?.lines().enumerate() {
        max_y = (i + 1) as isize;
        for (j, c) in line.chars().enumerate() {
            max_x = (j + 1) as isize;
            if c == '#' {
                state.insert((j as isize, i as isize, 0), true);
            }
        }
    }

    for _ in 0..6 {
        let mut new_state = state.clone();
        for x in -max_x..=max_x {
            for y in -max_y..=max_y {
                for z in -max_z..=max_z {
                    let active = *state.get(&(x, y, z)).unwrap_or(&false);
                    let count = count(&state, (x, y, z));
                    if active && (count < 2 || count > 3) {
                        new_state.insert((x, y, z), false);
                    } else if !active && count == 3 {
                        new_state.insert((x, y, z), true);
                        max_x = cmp::max(max_x, x.abs() + 1);
                        max_y = cmp::max(max_y, y.abs() + 1);
                        max_z = cmp::max(max_z, z.abs() + 1);
                    }
                }
            }
        }
        state = new_state;
    }

    let count = state.values().filter(|x| **x).count();
    println!("{}", count);

    Ok(())
}
