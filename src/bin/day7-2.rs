use advent::read_lines;
use petgraph::graphmap::DiGraphMap;
use std::collections::HashSet;

fn parse(s: &str) -> (String, Vec<(u32, String)>) {
    let mut neighbors = Vec::new();

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
            let num = remain[num_index..num_index + 1].parse::<u32>().unwrap();
            remain = &remain[num_index + 2..]; // skip space after number, assumes single digit
            let color_end = remain.find(" bag").unwrap();
            let color = remain[..color_end].to_string();
            neighbors.push((num, color));
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
            for (_, c) in neighbours {
                colors.insert(c.clone());
            }
        }
    }
    colors
}

fn get<'a>(colors: &'a HashSet<String>, color: &str) -> &'a str {
    colors.get(color).unwrap().as_str()
}

fn graph<'a>(colors: &'a HashSet<String>) -> DiGraphMap<&'a str, u32> {
    let mut graph = DiGraphMap::new();
    let mut lines = read_lines("data/day7input");
    while let Some(Ok(line)) = lines.next() {
        let (target_str, neighbours) = parse(&line);
        {
            let target = get(colors, &target_str);
            graph.add_node(target);
            for (num, color_str) in neighbours {
                let color = get(colors, &color_str);
                graph.add_node(color);
                graph.add_edge(target, color, num);
            }
        }
    }
    graph
}

fn num_bags(g: &DiGraphMap<&str, u32>, node: &str) -> u32 {
    let edges: Vec<(&str, u32)> = g.edges(node).map(|(_, t, i)| (t, *i)).collect();
    if edges.is_empty() {
        return 1;
    }
    edges.iter().map(|(n, i)| i * num_bags(&g, n)).sum::<u32>() + 1
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let colors = all_colors();
    let graph = graph(&colors);
    let shiny_gold: &str = get(&colors, "shiny gold");
    println!("{}", num_bags(&graph, shiny_gold) - 1);
    Ok(())
}
