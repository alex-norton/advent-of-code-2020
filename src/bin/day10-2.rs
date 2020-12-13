use petgraph::graphmap::DiGraphMap;
use std::collections::HashMap;
use std::fs::read_to_string;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut graph = DiGraphMap::<u64, ()>::new();
    read_to_string("data/day10input")?
        .split('\n')
        .for_each(|s| {
            graph.add_node(s.parse::<u64>().unwrap());
        });
    graph.add_node(0);
    // Clone 'cause you can't modify the graph
    for n in graph.nodes().collect::<Vec<_>>().clone() {
        if graph.contains_node(n + 1) {
            graph.add_edge(n, n + 1, ());
        }
        if graph.contains_node(n + 2) {
            graph.add_edge(n, n + 2, ());
        }
        if graph.contains_node(n + 3) {
            graph.add_edge(n, n + 3, ());
        }
    }

    let mut memo = HashMap::<u64, u64>::new();
    let count = num_paths(&graph, &mut memo, 0);
    println!("{}", count);
    Ok(())
}
fn num_paths(graph: &DiGraphMap<u64, ()>, memo: &mut HashMap<u64, u64>, num: u64) -> u64 {
    let mut neighbors = graph.neighbors(num).peekable();
    if let None = neighbors.peek() {
        return 1;
    }
    return neighbors
        .map(|n| match memo.get(&n) {
            None => {
                let count = num_paths(graph, memo, n);
                memo.insert(n, count);
                println!("{}, {}", n, count);
                count
            }
            Some(n) => *n,
        })
        .sum();
}
