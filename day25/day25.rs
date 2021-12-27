use itertools::Itertools;
use regex::Regex;
use std::cmp::max;
use std::cmp::min;
use std::collections::HashMap;
use std::env;

type Coordinate = (i32, i32);
type Map = HashMap<Coordinate, MapType>;

#[derive(Debug, PartialEq, Clone)]
enum MapType {
    Empty,
    MovingEast,
    MovingSouth,
}

fn map_from_file(inputfile: &str) -> Map {
    map_from_string(
        &std::fs::read_to_string(inputfile).expect("Something went wrong reading the file"),
    )
}

fn map_from_string(input: &str) -> Map {
    let mut map: Map = HashMap::new();

    input.lines().enumerate().for_each(|(y, row)| {
        row.chars().enumerate().for_each(|(x, c)| {
            let position: Coordinate = (x as i32, y as i32);
            match c {
                '.' => {
                    map.insert(position, MapType::Empty);
                }
                '>' => {
                    map.insert(position, MapType::MovingEast);
                }
                'v' => {
                    map.insert(position, MapType::MovingSouth);
                }
                _ => {}
            };
        })
    });

    map
}

fn print_map(map: &HashMap<Coordinate, MapType>) {
    let x_min = map.iter().map(|(pos, _)| pos.0).min().unwrap();
    let x_max = map.iter().map(|(pos, _)| pos.0).max().unwrap();
    let y_min = map.iter().map(|(pos, _)| pos.1).min().unwrap();
    let y_max = map.iter().map(|(pos, _)| pos.1).max().unwrap();

    println!(
        "{}",
        (y_min..y_max + 1)
            .map(|y| {
                (x_min..x_max + 1)
                    .map(|x| {
                        let position = (x, y);
                        match map.get(&position) {
                            Some(MapType::Empty) => ".",
                            Some(MapType::MovingEast) => ">",
                            Some(MapType::MovingSouth) => "v",
                            None => ".",
                        }
                    })
                    .join("")
            })
            .join("\n")
    );
}

fn step(map: &mut Map) -> usize {
    let x_max = map.iter().map(|(pos, _)| pos.0).max().unwrap() + 1;
    let y_max = map.iter().map(|(pos, _)| pos.1).max().unwrap() + 1;

    let mut num_moves = 0;

    for (check_direction, offset) in [
        (MapType::MovingEast, (1, 0)),
        (MapType::MovingSouth, (0, 1)),
    ] {
        let mut moves: Vec<(Coordinate, Coordinate)> = vec![];
        for (position, direction) in map
            .iter()
            .filter(|&(position, direction)| *direction == check_direction)
        {
            let neighbour = (
                (position.0 + offset.0) % x_max,
                (position.1 + offset.1) % y_max,
            );

            if let Some(MapType::Empty) = map.get(&neighbour) {
                moves.push((*position, neighbour));
            }
        }

        num_moves += moves.len();

        for (from, to) in moves {
            if let Some(direction) = map.get(&from) {
                map.insert(to, direction.clone());
                map.entry(from).and_modify(|e| *e = MapType::Empty);
            }
        }
    }

    num_moves
}

fn solve_part1(inputfile: String) -> usize {
    let mut map = map_from_file(&inputfile);

    println!();
    let mut iteration = 0;
    while step(&mut map) != 0 {
        println!("steps: {}", iteration);
        iteration += 1;
    }
    iteration + 1
}

fn solve_part2(inputfile: String) -> usize {
    0
}
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Part1: {}", solve_part1(args[1].to_string()));
    println!("Part2: {}", solve_part2(args[1].to_string()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_example() {
        let mut map = Map::from_string(
            "v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>",
        );

        println!("{:?}", map);
    }
}
