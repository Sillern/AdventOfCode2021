use itertools::Itertools;
use std::collections::HashMap;
use std::env;

type Caves = HashMap<String, Vec<String>>;

fn is_lowercase(value: &str) -> bool {
    match value.find(char::is_lowercase) {
        Some(_) => true,
        None => false,
    }
}

fn parse_input(inputfile: String) -> Caves {
    let contents =
        std::fs::read_to_string(inputfile).expect("Something went wrong reading the file");

    let mut caves = HashMap::new();

    contents.lines().for_each(|line| {
        let mut tokens = line.split('-');
        let name = tokens.next().unwrap();
        let connection = tokens.next().unwrap();

        caves
            .entry(name.to_string())
            .and_modify(|e: &mut Vec<String>| e.push(connection.to_string()))
            .or_insert(vec![connection.to_string()]);

        caves
            .entry(connection.to_string())
            .and_modify(|e: &mut Vec<String>| e.push(name.to_string()))
            .or_insert(vec![name.to_string()]);
    });
    caves
}

fn find_paths(caves: &Caves, start: &str, end: &str) -> usize {
    let mut queue: Vec<(String, Vec<String>)> = vec![(start.to_string(), vec![])];
    let mut num_paths = 0;

    let mut paths: Vec<Vec<String>> = vec![];

    fn could_revisit_node(previous_nodes: &Vec<String>, next_node: &str) -> bool {
        if previous_nodes.iter().any(|e| e == next_node) {
            !is_lowercase(next_node)
        } else {
            true
        }
    }

    while !queue.is_empty() {
        match queue.pop() {
            Some((position, mut path)) => {
                let previous_node = if let Some(node) = path.last() {
                    Some(node.clone())
                } else {
                    None
                };

                path.push(position.clone());

                if position == end {
                    paths.push(path);
                } else {
                    if let Some(backtrack_node) = &previous_node {
                        if could_revisit_node(&path, &backtrack_node) {
                            queue.push((backtrack_node.clone(), path.clone()));
                        }
                    }

                    if let Some(connections) = caves.get(&position) {
                        for connection in connections {
                            if could_revisit_node(&path, &connection) {
                                queue.push((connection.clone(), path.clone()));
                            }
                        }
                    }
                }
            }
            None => (),
        }
    }

    paths.sort();
    paths.dedup();

    for path in paths.iter() {
        println!("path: {:?}", path);
    }
    paths.len()
}

fn find_paths_small_node_twice(caves: &Caves, start: &str, end: &str) -> usize {
    let mut queue: Vec<(String, Vec<String>)> = vec![(start.to_string(), vec![])];
    let mut num_paths = 0;

    let mut paths: Vec<Vec<String>> = vec![];

    fn could_revisit_node(previous_nodes: &Vec<String>, next_node: &str) -> bool {
        let already_has_visited_a_lowercase_node_twice = previous_nodes
            .iter()
            .filter(|node| is_lowercase(node))
            .sorted()
            .counts()
            .iter()
            .any(|(_, count)| count > &1);

        if previous_nodes.iter().any(|e| e == next_node) {
            if is_lowercase(next_node) {
                if next_node == "start" {
                    false
                } else {
                    !already_has_visited_a_lowercase_node_twice
                }
            } else {
                true
            }
        } else {
            true
        }
    }

    while !queue.is_empty() {
        match queue.pop() {
            Some((position, mut path)) => {
                let previous_node = if let Some(node) = path.last() {
                    Some(node.clone())
                } else {
                    None
                };

                path.push(position.clone());

                if position == end {
                    paths.push(path);
                } else {
                    if let Some(backtrack_node) = &previous_node {
                        if could_revisit_node(&path, &backtrack_node) {
                            queue.push((backtrack_node.clone(), path.clone()));
                        }
                    }

                    if let Some(connections) = caves.get(&position) {
                        for connection in connections {
                            if could_revisit_node(&path, &connection) {
                                queue.push((connection.clone(), path.clone()));
                            }
                        }
                    }
                }
            }
            None => (),
        }
    }

    paths.sort();
    paths.dedup();

    paths.len()
}

fn solve_part1(inputfile: String) -> usize {
    let caves = parse_input(inputfile);

    println!("caves: {:?}", caves);
    find_paths(&caves, "start", "end")
}

fn solve_part2(inputfile: String) -> usize {
    let caves = parse_input(inputfile);

    println!("caves: {:?}", caves);
    find_paths_small_node_twice(&caves, "start", "end")
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Part1: {}", solve_part1(args[1].to_string()));
    println!("Part2: {}", solve_part2(args[1].to_string()));
}
