use itertools::Itertools;
use std::collections::HashMap;
use std::env;
use std::fmt;

type Coordinate = (i32, i32);

#[derive(Debug)]
enum AmphipodType {
    Amber,
    Bronze,
    Copper,
    Desert,
}

#[derive(Debug)]
struct Amphipod {
    amphipod_type: AmphipodType,
    position: Coordinate,
    home_cave: Vec<Coordinate>,
}

impl Amphipod {
    pub fn new(amphipod_type: AmphipodType, position: Coordinate) -> Self {
        Self {
            home_cave: match &amphipod_type {
                AmphipodType::Amber => vec![(3, 2), (3, 3)],
                AmphipodType::Bronze => vec![(5, 2), (5, 3)],
                AmphipodType::Copper => vec![(7, 2), (7, 3)],
                AmphipodType::Desert => vec![(9, 2), (9, 3)],
                _ => panic!(),
            },
            amphipod_type: amphipod_type,
            position: position,
        }
    }

    pub fn is_home(&self) -> bool {
        self.home_cave.contains(&self.position)
    }
}

#[derive(Debug)]
struct Game {
    map: HashMap<Coordinate, bool>,
    amphipods: Vec<Amphipod>,
}

impl Game {
    pub fn from_file(inputfile: &str) -> Self {
        Self::from_string(
            &std::fs::read_to_string(inputfile).expect("Something went wrong reading the file"),
        )
    }
    pub fn from_string(input: &str) -> Self {
        let mut amphipods = vec![];
        let mut map: HashMap<Coordinate, bool> = HashMap::new();

        input.lines().enumerate().for_each(|(y, row)| {
            row.chars().enumerate().for_each(|(x, c)| {
                let position: Coordinate = (x as i32, y as i32);
                match c {
                    '#' => {
                        map.insert(position, true);
                    }
                    '.' => {
                        map.insert(position, false);
                    }
                    ' ' => {}
                    _ => {
                        let amphipod_type = match c {
                            'A' => AmphipodType::Amber,
                            'B' => AmphipodType::Bronze,
                            'C' => AmphipodType::Copper,
                            'D' => AmphipodType::Desert,
                            _ => panic!(),
                        };

                        amphipods.push(Amphipod::new(amphipod_type, position));
                        map.insert(position, false);
                    }
                };
            })
        });

        Self { map, amphipods }
    }

    pub fn step(&mut self) {
        println!("do stuff");
    }

    pub fn num_amphipods(&self) -> usize {
        self.amphipods.len()
    }

    pub fn num_organized_amphipods(&self) -> usize {
        self.amphipods
            .iter()
            .filter(|&amphipod| amphipod.is_home())
            .count()
    }
}

impl fmt::Display for Amphipod {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self.amphipod_type {
                AmphipodType::Amber => "A",
                AmphipodType::Bronze => "B",
                AmphipodType::Copper => "C",
                AmphipodType::Desert => "D",
                _ => panic!(),
            }
        )
    }
}
impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let x_min = self.map.iter().map(|(pos, _)| pos.0).min().unwrap();
        let x_max = self.map.iter().map(|(pos, _)| pos.0).max().unwrap();
        let y_min = self.map.iter().map(|(pos, _)| pos.1).min().unwrap();
        let y_max = self.map.iter().map(|(pos, _)| pos.1).max().unwrap();

        write!(
            f,
            "{}",
            (y_min..y_max + 1)
                .map(|y| {
                    (x_min..x_max + 1)
                        .map(|x| {
                            let position = (x, y);
                            if let Some(amphipod) = self
                                .amphipods
                                .iter()
                                .find(|&amphipod| amphipod.position == position)
                            {
                                format!("{}", amphipod)
                            } else {
                                match self.map.get(&position) {
                                    Some(true) => "#",
                                    Some(false) => ".",
                                    None => " ",
                                }
                                .to_string()
                            }
                        })
                        .join("")
                })
                .join("\n")
        )
    }
}
fn solve_part1() -> usize {
    let mut game = Game::from_string(
        "#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########",
    );
    println!("{}", game);
    game.step();
    println!("{}", game);
    0
}

fn solve_part2() -> usize {
    0
}

fn main() {
    println!("Part1: {}", solve_part1());
    println!("Part2: {}", solve_part2());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_initial_game() {
        let mut game = Game::from_string(
            "#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########",
        );

        println!("game: {}", game);
        assert_eq!(game.num_amphipods(), 8);
        assert_eq!(game.num_organized_amphipods(), 3);
    }
}
