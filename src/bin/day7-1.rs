use advent::read_lines;
use petgraph::graphmap::DiGraphMap;
use std::collections::HashSet;

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

fn all_colors() -> HashSet<String> {
    let mut colors: HashSet<String> = HashSet::new();
    let mut lines = read_lines("data/day7input");
    while let Some(Ok(line)) = lines.next() {
        let (target_str, neighbours) = parse(&line);
        {
            colors.insert(target_str.clone());
            for c in neighbours {
                colors.insert(c.clone());
            }
        }
    }
    colors
}

fn get<'a>(colors: &'a HashSet<String>, color: &str) -> &'a str {
    colors.get(color).unwrap().as_str()
}

fn graph<'a>(colors: &'a HashSet<String>) -> DiGraphMap<&'a str, ()> {
    let mut graph = DiGraphMap::new();
    let mut lines = read_lines("data/day7input");
    while let Some(Ok(line)) = lines.next() {
        let (target_str, neighbours) = parse(&line);
        {
            let target = get(colors, &target_str);
            graph.add_node(target);
            for color_str in neighbours {
                let color = get(colors, &color_str);
                graph.add_node(color);
                graph.add_edge(color, target, ());
            }
        }
    }
    graph
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let colors = all_colors();
    let graph = graph(&colors);
    let mut to_visit: Vec<&str> = vec![get(&colors, "shiny gold")];
    let mut visited: HashSet<&str> = HashSet::new();
    while let Some(node) = to_visit.pop() {
        visited.insert(node);
        for n in graph.neighbors(node) {
            if !visited.contains(n) {
                to_visit.push(n);
            }
        }
    }
    println!("{}", visited.len() - 1); // exclude shiny gold
    Ok(())
}
