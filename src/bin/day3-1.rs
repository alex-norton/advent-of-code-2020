use advent::read_lines;

fn main() {
    let grid: Vec<Vec<char>> = read_lines("data/day3input")
        .filter_map(Result::ok)
        .map(|x| x.chars().collect())
        .collect();
    let width = grid[0].len();
    let height = grid.len();
    let trees = (0..height)
        .map(|x| (x, 3 * x % width))
        .filter(|(i, j)| grid[*i][*j] == '#')
        .count();
    println!("{}", trees)
}
