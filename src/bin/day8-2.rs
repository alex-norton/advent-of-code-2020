use advent::read_lines;
use partitions::partition_vec::PartitionVec;
use std::collections::HashSet;

type Line = (String, isize);

fn main() {
    // parse input into instruction, number pairs
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

    // build all connected components using union find / partition vector
    // find index of any line that leads to the exit
    let total = lines.len();
    let mut to_visit: HashSet<usize> = (0..total).collect();
    let mut conn_comps: PartitionVec<usize> = (0..total).collect();
    let mut exit = usize::MAX;

    while !to_visit.is_empty() {
        let mut cur_line = *to_visit.iter().next().unwrap();
        while to_visit.contains(&cur_line) {
            to_visit.remove(&cur_line);
            let (op, num) = lines.get(cur_line).unwrap();
            let next_line = match op.as_str() {
                "nop" | "acc" => cur_line + 1,
                "jmp" => (*num + (cur_line as isize)) as usize,
                _ => panic!(),
            };
            if next_line == total {
                exit = cur_line;
            } else {
                conn_comps.union(cur_line, next_line);
            }
            cur_line = next_line;
        }
    }

    // execute program. if swapping the instruction would put us on the path to the exit
    // (that is, would intersect with "exit"'s connected component) then do so
    let mut acc = 0;
    let mut changed = false;
    let mut cur_line = 0;
    while cur_line != total {
        let (op, num) = lines.get(cur_line).unwrap();
        let next_line = cur_line + 1;
        let jump_line = (*num + (cur_line as isize)) as usize;
        if op == "acc" {
            acc += num;
            cur_line = next_line;
            continue;
        }
        if !changed {
            if op == "jmp" && conn_comps.same_set(exit, next_line) {
                changed = true;
                cur_line = next_line;
                continue;
            }
            if op == "nop" && conn_comps.same_set(exit, jump_line) {
                changed = true;
                cur_line = jump_line;
                continue;
            }
        }
        if op == "jmp" {
            cur_line = jump_line;
        } else if op == "nop" {
            cur_line = next_line;
        } else {
            panic!()
        }
    }
    println!("{}", acc);
}
