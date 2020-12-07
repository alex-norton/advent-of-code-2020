use advent::read_lines;
use std::collections::HashMap;

struct Node {
    name: String,
    neighbours: HashMap<String, Node>,
}

fn parse(s: &str) -> (String, Vec<String>) {
    let mut neighbors = Vec::<String>::new();

    let target_end = s.find(" bag").unwrap();
    let target = s[..target_end].to_string();
    let mut remain: &str = &s[target_end..];

    if remain.find("no other bags").is_some() {
        return (target, neighbors);
    }

    let mut more = true;
    while more {
        let num_found = remain.find(char::is_numeric);
        if let Some(num_index) = num_found {
            remain = &remain[num_index + 2..]; // skip space after number
            let color_end = remain.find(" bag").unwrap();
            let color = remain[..color_end].to_string();
            neighbors.push(color);
            remain = &remain[color_end..];
        } else {
            more = false;
        }
    }
    (target, neighbors)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut lines = read_lines("data/test");
    while let Some(Ok(line)) = lines.next() {
        let (target, neighbours) = parse(&line);
        println!("{}: {:?}", target, neighbours);
    }
    Ok(())
}
