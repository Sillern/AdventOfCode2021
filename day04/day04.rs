use itertools::Itertools;
use std::env;
use std::fmt;

#[derive(Debug)]
struct Board {
    numbers: Vec<(u32, bool)>,
    pub valid_score: u32,
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.numbers
                .iter()
                .chunks(5)
                .into_iter()
                .map(|row| {
                    row.map(|(number, marked)| {
                        if *marked {
                            " * ".to_string()
                        } else {
                            format!("{:<3}", number)
                        }
                    })
                    .join(" ")
                })
                .join("\n")
        )
    }
}

impl Board {
    pub fn new(numbers: &str) -> Self {
        Self {
            numbers: numbers
                .split(&['\n', ' '][..])
                .filter_map(|maybe_number| maybe_number.parse::<u32>().ok().map(|number| (number, false)))
                .collect::<Vec<(u32, bool)>>(),
            valid_score: 0,
        }
    }

    pub fn check_drawn_number(&mut self, number: u32) -> Option<u32> {
        if self.valid_score > 0 {
            return None;
        }

        match self.numbers.iter().position(|&(value, _)| number == value) {
            Some(index) => {
                self.numbers[index] = (number, true);
                if self.has_winning_criteria() {
                    self.valid_score = self.score(number);
                    Some(self.valid_score)
                } else {
                    None
                }
            }
            None => None,
        }
    }

    fn has_winning_criteria(&self) -> bool {
        let row_size = 5;
        let column_size = 5;

        let rows = (0..column_size)
            .map(|offset| {
                self.numbers
                    .iter()
                    .skip(offset * row_size)
                    .take(row_size)
                    .all(|(_, marked)| *marked)
            })
            .any(|has_won| has_won);

        let columns = (0..row_size)
            .map(|offset| {
                self.numbers
                    .iter()
                    .skip(offset)
                    .step_by(row_size)
                    .all(|(_, marked)| *marked)
            })
            .any(|has_won| has_won);

        rows || columns
    }

    fn score(&self, last_drawn_number: u32) -> u32 {
        last_drawn_number
            * self.numbers.iter().fold(0, |acc, &(number, marked)| {
                acc + if !marked { number } else { 0 }
            })
    }
}

fn solve_part1(inputfile: String) -> usize {
    let contents =
        std::fs::read_to_string(inputfile).expect("Something went wrong reading the file");

    let drawn_numbers = contents
        .lines().next()
        .unwrap()
        .split(',')
        .map(|number| number.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let mut boards = contents
        .split("\n\n")
        .skip(1)
        .map(Board::new)
        .collect::<Vec<Board>>();

    let winning_score = drawn_numbers
        .iter()
        .find_map(|&drawn_number| {
            boards
                .iter_mut()
                .filter_map(|board| board.check_drawn_number(drawn_number))
                .next()
        })
        .unwrap();

    winning_score as usize
}

fn solve_part2(inputfile: String) -> usize {
    let contents =
        std::fs::read_to_string(inputfile).expect("Something went wrong reading the file");

    let drawn_numbers = contents
        .lines().next()
        .unwrap()
        .split(',')
        .map(|number| number.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let mut boards = contents
        .split("\n\n")
        .skip(1)
        .map(Board::new)
        .collect::<Vec<Board>>();

    let winning_scores = drawn_numbers
        .iter()
        .filter_map(|&drawn_number| {
            let boards_won = boards
                .iter_mut()
                .map(|board| board.check_drawn_number(drawn_number))
                .collect::<Vec<Option<u32>>>();

            boards_won.iter().filter_map(|&score| score).next()
        })
        .collect::<Vec<u32>>();

    *winning_scores.last().unwrap() as usize
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Part1: {}", solve_part1(args[1].to_string()));
    println!("Part2: {}", solve_part2(args[1].to_string()));
}
