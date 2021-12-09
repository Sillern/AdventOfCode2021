use itertools::Itertools;
use std::char;
use std::collections::HashMap;
use std::env;

fn solve_part1(inputfile: String) -> usize {
    let contents =
        std::fs::read_to_string(inputfile).expect("Something went wrong reading the file");

    let num_easy = contents.lines().fold(0, |total_acc, line| {
        let decoded: usize = line
            .split(" | ")
            .nth(1)
            .unwrap()
            .split(' ')
            .filter_map(|output| match output.len() {
                2 => Some(1),
                3 => Some(1),
                4 => Some(1),
                7 => Some(1),
                _ => Some(0),
            })
            .sum();

        total_acc + decoded
    });
    num_easy
}

fn solve_part2(inputfile: String) -> usize {
    let contents =
        std::fs::read_to_string(inputfile).expect("Something went wrong reading the file");

    //  0000
    // 1    2
    // 1    2
    //  3333
    // 4    5
    // 4    5
    //  6666
    let full_mapping = HashMap::from([
        (0, vec!['a', 'b', 'c', 'd', 'e', 'f', 'g']),
        (1, vec!['a', 'b', 'c', 'd', 'e', 'f', 'g']),
        (2, vec!['a', 'b', 'c', 'd', 'e', 'f', 'g']),
        (3, vec!['a', 'b', 'c', 'd', 'e', 'f', 'g']),
        (4, vec!['a', 'b', 'c', 'd', 'e', 'f', 'g']),
        (5, vec!['a', 'b', 'c', 'd', 'e', 'f', 'g']),
        (6, vec!['a', 'b', 'c', 'd', 'e', 'f', 'g']),
    ]);

    let number_mapping = HashMap::from([
        (0, vec![0, 1, 2, 4, 5, 6]),
        (1, vec![2, 5]),
        (2, vec![0, 2, 3, 4, 6]),
        (3, vec![0, 2, 3, 5, 6]),
        (4, vec![1, 2, 3, 5]),
        (5, vec![0, 1, 3, 5, 6]),
        (6, vec![0, 1, 3, 4, 5, 6]),
        (7, vec![0, 2, 5]),
        (8, vec![0, 1, 2, 3, 4, 5, 6]),
        (9, vec![0, 1, 2, 3, 5, 6]),
    ]);

    let mut reverse_number_mapping = HashMap::new();

    for (key, values) in &number_mapping {
        for value in values {
            reverse_number_mapping
                .entry(value)
                .and_modify(|e: &mut Vec<usize>| (*e).push(*key))
                .or_insert(vec![*key]);
        }
    }

    let mut number_length_mapping = HashMap::new();

    for (key, values) in &number_mapping {
        number_length_mapping
            .entry(values.len())
            .and_modify(|e: &mut Vec<usize>| (*e).push(*key))
            .or_insert(vec![*key]);
    }

    for (tile, numbers) in reverse_number_mapping.iter() {
        println!("tile[{}] in numbers: {:?}", tile, numbers);
    }
    for (number, tiles) in number_mapping.iter() {
        println!("number[{}] : {:?}", number, tiles);
    }
    for (length, numbers) in number_length_mapping.iter() {
        println!("length[{}] : {:?}", length, numbers);
    }

    contents.lines().fold(0, |acc, line| {
        let mut mapping = full_mapping.clone();
        let decoded = line
            .split(" | ")
            .map(|signals| signals.split(' ').collect::<Vec<&str>>())
            .collect::<Vec<Vec<&str>>>();

        let inputs = decoded.iter().nth(0).unwrap();
        let outputs = decoded
            .iter()
            .nth(1)
            .unwrap()
            .iter()
            .map(|pattern| pattern.chars().sorted().collect::<String>())
            .collect::<Vec<String>>();

        let sorted_inputs = inputs
            .iter()
            .map(|pattern| pattern.chars().sorted().collect::<String>())
            .sorted_by(|a, b| Ord::cmp(&b.len(), &a.len()))
            .collect::<Vec<String>>();

        let mut deduced_number_mapping = HashMap::new();

        for pattern in &sorted_inputs {
            let signals = pattern.chars().collect::<Vec<char>>();
            match pattern.len() {
                2 => {
                    deduced_number_mapping.insert(1, pattern.clone());
                    for (key, possibilities) in mapping.iter_mut() {
                        let filtered_possibilities: Vec<char> = possibilities
                            .iter()
                            .filter(|signal| !signals.contains(signal)).copied()
                            .collect();

                        *possibilities = match *key {
                            2 => signals.clone(),
                            5 => signals.clone(),
                            _ => filtered_possibilities.clone(),
                        }
                    }
                }
                3 => {
                    deduced_number_mapping.insert(7, pattern.clone());
                    for (key, possibilities) in mapping.iter_mut() {
                        let filtered_possibilities: Vec<char> = possibilities
                            .iter()
                            .filter(|signal| !signals.contains(signal)).copied()
                            .collect();

                        *possibilities = match *key {
                            0 => signals.clone(),
                            2 => signals.clone(),
                            5 => signals.clone(),
                            _ => filtered_possibilities.clone(),
                        }
                    }
                }
                4 => {
                    deduced_number_mapping.insert(4, pattern.clone());
                    for (key, possibilities) in mapping.iter_mut() {
                        let filtered_possibilities: Vec<char> = possibilities
                            .iter()
                            .filter(|signal| !signals.contains(signal)).copied()
                            .collect();

                        *possibilities = match *key {
                            1 => signals.clone(),
                            2 => signals.clone(),
                            3 => signals.clone(),
                            5 => signals.clone(),
                            _ => filtered_possibilities.clone(),
                        }
                    }
                }
                5 => {
                    for place in [0, 3, 6] {
                        mapping.entry(place).and_modify(|possibilities| {
                            *possibilities = possibilities
                                .iter()
                                .filter(|signal| signals.contains(signal)).copied()
                                .collect::<Vec<char>>()
                        });
                    }
                }
                6 => {
                    for place in [0, 1, 5, 6] {
                        mapping.entry(place).and_modify(|possibilities| {
                            *possibilities = possibilities
                                .iter()
                                .filter(|signal| signals.contains(signal)).copied()
                                .collect::<Vec<char>>()
                        });
                    }
                }
                7 => {
                    deduced_number_mapping.insert(8, pattern.clone());
                }
                _ => (),
            };

            let completed_mapping = mapping
                .iter()
                .filter_map(|(_, values)| {
                    if values.len() == 1 {
                        Some(values[0])
                    } else {
                        None
                    }
                })
                .collect::<Vec<char>>();

            for possibility in completed_mapping {
                for (_, possibilities) in mapping.iter_mut().filter(|(_, values)| values.len() != 1)
                {
                    let filtered_possibilities = possibilities
                        .iter()
                        .filter(|signal| &&possibility != signal).copied()
                        .collect::<Vec<char>>();

                    *possibilities = filtered_possibilities;
                }
            }
        }
        let mut reverse_mapping = HashMap::new();

        for (key, values) in &mapping {
            for value in values {
                reverse_mapping
                    .entry(value)
                    .and_modify(|e: &mut Vec<usize>| (*e).push(*key))
                    .or_insert(vec![*key]);
            }
        }

        for pattern in sorted_inputs {
            if deduced_number_mapping.values().contains(&&pattern) {
                continue;
            }

            let signals = pattern.chars().collect::<Vec<char>>();

            let first_candidates = number_length_mapping[&pattern.len()]
                .iter()
                .filter_map(|candidate| {
                    if !deduced_number_mapping.keys().contains(candidate) {
                        let has_invalid_tiles =
                            number_mapping[candidate].iter().fold(0, |acc, number| {
                                if mapping[number].len() != 1 {
                                    return acc;
                                }

                                acc + mapping[number].iter().fold(
                                    0,
                                    |inner_acc, required_character| {
                                        inner_acc
                                            + if !pattern.chars().contains(required_character) {
                                                1
                                            } else {
                                                0
                                            }
                                    },
                                )
                            });
                        if has_invalid_tiles == 0 {
                            Some(*candidate)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .collect::<Vec<usize>>();

            let mut required: Vec<usize> = vec![];
            for signal in &signals {
                let possibilities = reverse_mapping.get(&signal).unwrap();
                match possibilities.len() {
                    1 => required.extend(possibilities),
                    2 => {
                        let has_both = possibilities.iter().all(|&possibility| {
                            mapping[&possibility]
                                .iter()
                                .all(|character| signals.contains(character))
                        });

                        if has_both {
                            required.extend(possibilities);
                        }
                    }
                    _ => println!("Something broken"),
                };
            }
            required.sort_unstable();
            required.dedup();

            let mut candidates = vec![first_candidates
                .iter()
                .filter_map(|candidate| {
                    if required
                        .iter()
                        .all(|x| number_mapping[candidate].contains(x))
                    {
                        Some(*candidate)
                    } else {
                        None
                    }
                })
                .collect::<Vec<usize>>()];

            for signal in &signals {
                let possibilities = reverse_mapping.get(&signal).unwrap();

                match possibilities.len() {
                    1 => {
                        let candidate = possibilities[0];
                        for candidate_list in &mut candidates {
                            *candidate_list = candidate_list
                                .iter()
                                .filter(|e| reverse_number_mapping[&candidate].contains(e)).copied()
                                .collect();
                        }
                    }
                    2 => {
                        let has_both = possibilities.iter().all(|&possibility| {
                            mapping[&possibility]
                                .iter()
                                .all(|character| signals.contains(character))
                        });

                        if has_both {
                            for candidate_list in &mut candidates {
                                for candidate in possibilities {
                                    let filtered_list = candidate_list
                                        .iter()
                                        .filter_map(|e| {
                                            if reverse_number_mapping[&candidate].contains(e) {
                                                Some(*e)
                                            } else {
                                                None
                                            }
                                        })
                                        .collect::<Vec<usize>>();
                                    if !filtered_list.is_empty() {
                                        *candidate_list = filtered_list;
                                    }
                                }
                            }
                        } else {
                            let mut candidates_list = [candidates.clone(), candidates.clone()];

                            for (index, candidate) in possibilities.iter().enumerate() {
                                for candidate_list in &mut candidates_list[index] {
                                    let filtered_list = candidate_list
                                        .iter()
                                        .filter_map(|e| {
                                            if reverse_number_mapping[&candidate].contains(e) {
                                                Some(*e)
                                            } else {
                                                None
                                            }
                                        })
                                        .collect::<Vec<usize>>();
                                    if !filtered_list.is_empty() {
                                        *candidate_list = filtered_list;
                                    }
                                }
                            }
                            candidates = candidates_list[0].clone();
                            candidates.extend(candidates_list[1].clone());
                        }
                    }
                    _ => println!("Something broken"),
                }
            }
            candidates = candidates.clone().into_iter().unique().collect();
            if candidates.len() == 1 && candidates[0].len() == 1 {
                let single_match = candidates[0][0];
                deduced_number_mapping.insert(single_match, pattern);
            }
        }

        let reversed_deduced = deduced_number_mapping
            .iter()
            .map(|(key, value)| (value.clone(), *key))
            .collect::<HashMap<String, usize>>();
        acc + outputs
            .iter()
            .map(|raw| char::from_digit(reversed_deduced[raw] as u32, 10).unwrap())
            .join("")
            .parse::<usize>()
            .unwrap()
    })
}
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Part1: {}", solve_part1(args[1].to_string()));
    println!("Part2: {}", solve_part2(args[1].to_string()));
}
