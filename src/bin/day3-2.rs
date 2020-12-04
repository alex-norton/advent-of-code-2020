use advent::read_lines;

fn main() {
    let grid: Vec<Vec<char>> = read_lines("data/day3input")
        .filter_map(Result::ok)
        .map(|x| x.chars().collect())
        .collect();
    let width = grid[0].len();
    let height = grid.len();

    let slopes = [(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)];
    let product: usize = slopes
        .iter()
        .map(|(x, y)| {
            (0..height)
                .map(|s| (s * x, s * y % width))
                .filter(|(i, _)| i < &height)
                .filter(|(i, j)| grid[*i][*j] == '#')
                .count()
        })
        .product();
    println!("{}", product)
}
