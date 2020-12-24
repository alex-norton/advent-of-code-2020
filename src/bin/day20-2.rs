use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::read_to_string;

/**
 * We could probably do some clever variant of KMP to speed this up, but let's
 * just brute force every sea monster orientation.
 */
type Edge = Vec<char>;
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Tile {
    id: usize,
    top: Edge,
    right: Edge,
    bottom: Edge,
    left: Edge,
    image: Vec<Vec<char>>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // stores all tile variants for each id
    let mut tiles = HashSet::<Tile>::new();
    let mut num_tiles = 0;

    read_to_string("data/day20input")?
        .split("\n\n")
        .for_each(|s| {
            num_tiles += 1;
            let mut lines = s.lines();
            let id_line = lines.next().unwrap();
            let start = id_line.find(' ').unwrap();
            let end = id_line.find(':').unwrap();
            let id = id_line[start + 1..end].parse::<usize>().unwrap();

            let data = lines
                .map(|l| l.chars().collect::<Vec<char>>())
                .collect::<Vec<_>>();

            for chars in all_variations(data) {
                let height = chars.len();
                let width = chars[0].len();
                let top: Edge = (0..width).map(|i| chars[0][i]).collect();
                let right: Edge = (0..height).map(|i| chars[i][width - 1]).collect();
                let bottom: Edge = (0..width).map(|i| chars[height - 1][i]).collect();
                let left: Edge = (0..height).map(|i| chars[i][0]).collect();
                let image = (1..height - 1)
                    .map(|i| (1..width - 1).map(|j| chars[i][j]).collect())
                    .collect();
                let tile = Tile {
                    id,
                    top,
                    right,
                    bottom,
                    left,
                    image,
                };
                tiles.insert(tile);
            }
        });
    // we fill puzzle left to right, top to bottom
    // so at any given insertion, we only need to match with
    // new tiles left edge and top edge
    let mut lefts = HashMap::<Edge, HashSet<&Tile>>::new();
    let mut tops = HashMap::<Edge, HashSet<&Tile>>::new();
    for t in &tiles {
        match lefts.get_mut(&t.left) {
            Some(s) => {
                s.insert(t);
            }
            None => {
                let mut s = HashSet::new();
                s.insert(t);
                lefts.insert(t.left.clone(), s);
            }
        }
        match tops.get_mut(&t.top) {
            Some(s) => {
                s.insert(t);
            }
            None => {
                let mut s = HashSet::new();
                s.insert(t);
                tops.insert(t.top.clone(), s);
            }
        }
    }
    let dims = (num_tiles as f64).sqrt() as usize;
    let mut puzzle: Vec<Vec<Option<&Tile>>> = vec![vec![None; dims]; dims];
    let mut used: HashSet<usize> = HashSet::new();
    if !solve(&mut puzzle, &mut used, &tiles, &lefts, &tops, dims, 0, 0) {
        panic!("No solution found.");
    }
    let top_left = puzzle[0][0].unwrap().id;
    let top_right = puzzle[0][dims - 1].unwrap().id;
    let bottom_left = puzzle[dims - 1][0].unwrap().id;
    let bottom_right = puzzle[dims - 1][dims - 1].unwrap().id;
    println!("{}", top_left * top_right * bottom_left * bottom_right);
    Ok(())
}

fn all_variations(chars: Vec<Vec<char>>) -> Vec<Vec<Vec<char>>> {
    let mut variations = Vec::new();
    // Untouched
    variations.push(chars);
    variations.push(rotate_cw(&variations[0]));
    variations.push(rotate_cw(&variations[1]));
    variations.push(rotate_cw(&variations[2]));
    variations.push(flip_horiz(&variations[0]));
    variations.push(flip_horiz(&variations[1]));
    variations.push(flip_horiz(&variations[2]));
    variations.push(flip_horiz(&variations[3]));
    variations
}

fn rotate_cw(chars: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let height = chars.len();
    let width = chars[0].len();
    (0..width)
        .map(|i| (0..height).rev().map(|j| chars[j][i]).collect())
        .collect()
}

fn flip_horiz(chars: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let height = chars.len();
    let width = chars[0].len();
    (0..height)
        .map(|i| (0..width).rev().map(|j| chars[i][j]).collect())
        .collect()
}

fn solve<'a>(
    puzzle: &mut Vec<Vec<Option<&'a Tile>>>,
    used: &mut HashSet<usize>,
    tiles: &'a HashSet<Tile>,
    lefts: &HashMap<Edge, HashSet<&'a Tile>>,
    tops: &HashMap<Edge, HashSet<&'a Tile>>,
    dims: usize,
    h: usize,
    w: usize,
) -> bool {
    if h == dims {
        return true;
    }
    if h == 0 && w == 0 {
        return try_tiles(tiles, puzzle, used, tiles, lefts, tops, dims, h, w);
    } else if h == 0 {
        let right_edge = &puzzle[h][w - 1].unwrap().right;
        let candidates = get_unused(lefts, right_edge, used);
        return try_tiles(candidates, puzzle, used, tiles, lefts, tops, dims, h, w);
    } else if w == 0 {
        let bottom_edge = &puzzle[h - 1][w].unwrap().bottom;
        let candidates = get_unused(tops, bottom_edge, used);
        return try_tiles(candidates, puzzle, used, tiles, lefts, tops, dims, h, w);
    } else {
        let right_edge = &puzzle[h][w - 1].unwrap().right;
        let right_candidates = get_unused(lefts, right_edge, used)
            .into_iter()
            .collect::<HashSet<&Tile>>();
        let bottom_edge = &puzzle[h - 1][w].unwrap().bottom;
        let bottom_candidates = get_unused(tops, bottom_edge, used)
            .into_iter()
            .collect::<HashSet<&Tile>>();
        return try_tiles(
            right_candidates
                .intersection(&bottom_candidates)
                .map(|x| *x),
            puzzle,
            used,
            tiles,
            lefts,
            tops,
            dims,
            h,
            w,
        );
    }
}

fn try_tiles<'a, I>(
    candidates: I,
    puzzle: &mut Vec<Vec<Option<&'a Tile>>>,
    used: &mut HashSet<usize>,
    tiles: &'a HashSet<Tile>,
    lefts: &HashMap<Edge, HashSet<&'a Tile>>,
    tops: &HashMap<Edge, HashSet<&'a Tile>>,
    dims: usize,
    h: usize,
    w: usize,
) -> bool
where
    I: IntoIterator<Item = &'a Tile>,
{
    for tile in candidates {
        let solved = try_tile(tile, puzzle, used, tiles, lefts, tops, dims, h, w);
        if solved {
            return true;
        }
    }
    return false;
}

fn try_tile<'a>(
    tile: &'a Tile,
    puzzle: &mut Vec<Vec<Option<&'a Tile>>>,
    used: &mut HashSet<usize>,
    tiles: &'a HashSet<Tile>,
    lefts: &HashMap<Edge, HashSet<&'a Tile>>,
    tops: &HashMap<Edge, HashSet<&'a Tile>>,
    dims: usize,
    h: usize,
    w: usize,
) -> bool {
    puzzle[h][w] = Some(tile);
    used.insert(tile.id);

    let next_h;
    let next_w;
    if w == dims - 1 {
        next_h = h + 1;
        next_w = 0;
    } else {
        next_h = h;
        next_w = w + 1;
    }

    let solved = solve(puzzle, used, tiles, lefts, tops, dims, next_h, next_w);
    if solved {
        return true;
    }
    puzzle[h][w] = None;
    used.remove(&tile.id);
    return false;
}

fn get_unused<'a>(
    candidates: &HashMap<Edge, HashSet<&'a Tile>>,
    edge: &Edge,
    used: &HashSet<usize>,
) -> Vec<&'a Tile> {
    candidates
        .get(edge)
        .unwrap()
        .iter()
        .filter(|t| !used.contains(&t.id))
        .map(|t| *t)
        .collect()
}
