use advent::read_lines;
use std::collections::HashMap;

struct Node<'a> {
    name: String,
    neighbours: HashMap<String, &'a Node<'a>>,
}

impl<'a> Node<'a> {
    fn new(name: String) -> Node<'a> {
        Node {
            name: name,
            neighbours: HashMap::new(),
        }
    }

    fn add_neighbour(&mut self, name: String, neighbour: &'a Node) {
        self.neighbours.insert(name, neighbour);
    }
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
    // This owns all the nodes.
    let mut nodes: HashMap<String, Node> = HashMap::new();

    let mut lines = read_lines("data/test");
    while let Some(Ok(line)) = lines.next() {
        let (target_str, neighbours) = parse(&line);
        println!("{}: {:?}", target_str, neighbours);
        let mut target = nodes
            .entry(target_str.clone())
            .or_insert(Node::new(target_str.clone()));
        for node_str in neighbours {
            let mut node = nodes
                .entry(node_str.clone())
                .or_insert(Node::new(node_str.clone()));
            node.add_neighbour(node_str.clone(), target);
        }
    }
    Ok(())
}
