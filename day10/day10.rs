use itertools::Itertools;
use std::env;

fn get_closing_counterpart(open: char) -> char {
    let open_lut = vec!['[', '(', '{', '<'];
    let close_lut = vec![']', ')', '}', '>'];

    let (open_index, _) = open_lut
        .iter()
        .enumerate()
        .find(|(index, x)| **x == open)
        .unwrap();

    *close_lut.get(open_index).unwrap()
}

fn is_counterpart(open: char, close: char) -> bool {
    let open_lut = vec!['[', '(', '{', '<'];
    let close_lut = vec![']', ')', '}', '>'];

    match close_lut.iter().enumerate().find(|(index, x)| **x == close) {
        Some((close_index, _)) => match open_lut.get(close_index) {
            Some(open_ch) => *open_ch == open,
            None => false,
        },
        None => false,
    }
}

fn solve_part1(inputfile: String) -> usize {
    let contents =
        std::fs::read_to_string(inputfile).expect("Something went wrong reading the file");

    let mut syntax_error_score = 0;

    let positions = contents.lines().for_each(|line| {
        let mut stack: Vec<char> = vec![];
        for ch in line.chars() {
            match ch {
                '[' | '(' | '{' | '<' => {
                    stack.push(ch);
                }
                ']' | ')' | '}' | '>' => match stack.pop() {
                    Some(matching) => {
                        if is_counterpart(matching, ch) {
                            //println!("popping {}", ch);
                        } else {
                            syntax_error_score += match (ch) {
                                ']' => 57,
                                ')' => 3,
                                '}' => 1197,
                                '>' => 25137,
                                _ => 0,
                            };
                        }
                    }
                    None => {
                        println!("missing {}", ch);
                    }
                },
                _ => {
                    println!("unknown character: {}", ch);
                }
            }
        }
    });
    syntax_error_score
}

fn solve_part2(inputfile: String) -> usize {
    let contents =
        std::fs::read_to_string(inputfile).expect("Something went wrong reading the file");

    let mut completion_error_score = 0;

    let completion_error_scores = contents
        .lines()
        .filter_map(|line| {
            let mut stack: Vec<char> = vec![];
            let mut is_corrupted = false;
            for ch in line.chars() {
                match ch {
                    '[' | '(' | '{' | '<' => {
                        stack.push(ch);
                    }
                    ']' | ')' | '}' | '>' => match stack.pop() {
                        Some(matching) => {
                            if !is_counterpart(matching, ch) {
                                is_corrupted = true
                            }
                        }
                        None => {
                            println!("missing {}", ch);
                        }
                    },
                    _ => {
                        println!("unknown character: {}", ch);
                    }
                }
            }

            if !is_corrupted {
                Some(stack.iter().rev().fold(0, |acc, open_ch| {
                    5 * acc
                        + match get_closing_counterpart(*open_ch) {
                            ')' => 1,
                            ']' => 2,
                            '}' => 3,
                            '>' => 4,
                            _ => 0,
                        }
                }))
            } else {
                None
            }
        })
        .sorted()
        .collect::<Vec<usize>>();

    completion_error_scores[completion_error_scores.len() / 2]
}
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Part1: {}", solve_part1(args[1].to_string()));
    println!("Part2: {}", solve_part2(args[1].to_string()));
}
