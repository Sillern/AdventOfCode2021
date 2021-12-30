use itertools::Itertools;


use std::collections::HashMap;
use std::env;

fn parse_input(inputfile: String) -> (Vec<char>, HashMap<(char, char), char>) {
    let mut template: Vec<char> = vec![];
    let mut rules: HashMap<(char, char), char> = HashMap::new();

    let _contents = std::fs::read_to_string(inputfile)
        .expect("Something went wrong reading the file")
        .lines()
        .for_each(|line| {
            if line.is_empty() {
            } else if line.contains(" -> ") {
                let mut tokens = line.split(" -> ");

                let key = tokens
                    .next()
                    .unwrap()
                    .chars()
                    .collect_tuple::<(char, char)>()
                    .unwrap();
                let value = tokens.next().unwrap().chars().next().unwrap();

                rules.entry(key).or_insert(value);
            } else {
                template = line.chars().collect();
            }
        });

    (template, rules)
}

fn solve_part1(inputfile: String) -> usize {
    let (mut template, rules) = parse_input(inputfile);

    for _step in 0..10 {
        template = template
            .iter()
            .peekable()
            .batching(|it| match it.next() {
                Some(a) => match it.peek() {
                    Some(&b) => {
                        if let Some(injection) = rules.get(&(*a, *b)) {
                            let mut it_copy = it.clone();
                            it_copy.next();

                            if let Some(_c) = it_copy.peek() {
                                Some(vec![*a, *injection])
                            } else {
                                Some(vec![*a, *injection, *b])
                            }
                        } else {
                            panic!()
                        }
                    }
                    None => None,
                },
                None => None,
            })
            .flatten()
            .collect::<Vec<char>>();
    }
    let counts = template
        .iter()
        .counts()
        .iter()
        .map(|(_, count)| *count)
        .sorted()
        .collect::<Vec<usize>>();

    counts.last().unwrap() - counts.first().unwrap()
}

fn solve_part2(inputfile: String) -> usize {
    let (template, rules) = parse_input(inputfile);

    let mut frequencies: HashMap<Vec<char>, usize> = template
        .iter()
        .tuple_windows()
        .map(|(a, b)| vec![*a, *b])
        .counts();

    for character in template {
        frequencies
            .entry(vec![character])
            .and_modify(|e| *e += 1)
            .or_insert(1);
    }

    for _step in 0..40 {
        let keys = frequencies
            .iter()
            .filter_map(|(key, count)| {
                if let Some((a, b)) = key.iter().collect_tuple() {
                    Some(((*a, *b), *count))
                } else {
                    None
                }
            })
            .collect::<Vec<((char, char), usize)>>();

        for ((a, b), count) in keys {
            if let Some(injection) = rules.get(&(a, b)) {
                for key in [vec![a, *injection], vec![*injection, b], vec![*injection]] {
                    frequencies
                        .entry(key)
                        .and_modify(|e| *e += count)
                        .or_insert(count);
                }
                frequencies.entry(vec![a, b]).and_modify(|e| *e -= count);
            }
        }
    }

    let counts = frequencies
        .iter()
        .filter_map(
            |(key, count)| {
                if key.len() == 1 {
                    Some(*count)
                } else {
                    None
                }
            },
        )
        .sorted()
        .collect::<Vec<usize>>();

    counts.last().unwrap() - counts.first().unwrap()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Part1: {}", solve_part1(args[1].to_string()));
    println!("Part2: {}", solve_part2(args[1].to_string()));
}
