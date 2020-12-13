use std::fs::read_to_string;
type Board = Vec<Vec<char>>;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut cur: Board = read_to_string("data/day11input")?
        .split('\n')
        .map(|s| s.chars().collect())
        .collect();
    loop {
        print(&cur);
        let next = evolve(&cur);
        if next == cur {
            println!(
                "{}",
                cur.iter()
                    .map(|i| i.iter().filter(|j| **j == '#').count())
                    .sum::<usize>()
            );
            break;
        }
        cur = next;
    }
    Ok(())
}

fn evolve(b: &Board) -> Board {
    let mut new = b.clone();
    for i in 0..b.len() {
        for j in 0..b[0].len() {
            let count = count(b, i, j);
            if b[i][j] == 'L' && count == 0 {
                new[i][j] = '#';
            } else if b[i][j] == '#' && count >= 4 {
                new[i][j] = 'L';
            }
        }
    }
    new
}

fn count(b: &Board, i: usize, j: usize) -> usize {
    let mut count = 0;
    let max_i: isize = b.len() as isize;
    let max_j: isize = b[0].len() as isize;
    for d_i in -1..=1 {
        for d_j in -1..=1 {
            if d_i == 0 && d_j == 0 {
                continue;
            }
            let n_i = (i as isize) + d_i;
            let n_j = (j as isize) + d_j;
            if n_i < 0 || n_i >= max_i || n_j < 0 || n_j >= max_j {
                continue;
            }
            if b[n_i as usize][n_j as usize] == '#' {
                count += 1;
            }
        }
    }
    count
}

fn print(b: &Board) {
    b.iter().for_each(|i| {
        i.iter().for_each(|j| print!("{}", j));
        print!("\n");
    });
    print!("\n");
}
