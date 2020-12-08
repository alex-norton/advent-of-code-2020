use advent::read_lines;
use std::collections::HashSet;

type Line = (String, isize);

fn next(lines: &Vec<Line>, acc: &mut isize, cur_line: usize) -> usize {
    let (op, num) = lines.get(cur_line).unwrap();
    match op.as_str() {
        "nop" => cur_line + 1,
        "jmp" => (*num + (cur_line as isize)) as usize,
        "acc" => {
            *acc += num;
            cur_line + 1
        }
        _ => panic!(),
    }
}

fn main() {
    let lines: Vec<Line> = read_lines("data/day8input")
        .filter_map(Result::ok)
        .map(|l| {
            let mut parts = l.split(' ');
            (
                String::from(parts.next().unwrap()),
                parts.next().unwrap().parse::<isize>().unwrap(),
            )
        })
        .collect();
    println!("{:?}", lines);

    let mut visited = HashSet::<usize>::new();
    let mut next_line = 0;
    let mut acc = 0;
    while !visited.contains(&next_line) {
        visited.insert(next_line);
        next_line = next(&lines, &mut acc, next_line);
        println!("{}, {}", next_line, acc);
    }
}
