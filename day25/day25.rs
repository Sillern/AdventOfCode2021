use itertools::Itertools;
use regex::Regex;
use std::cmp::max;
use std::cmp::min;
use std::collections::HashMap;
use std::env;

type Coordinate = (i32, i32);
type Map = HashMap<Coordinate, SeacucumberDirection>;

#[derive(Debug)]
enum SeacucumberDirection {
    East,
    South,
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
                '>' => {
                    map.insert(position, SeacucumberDirection::East);
                }
                'v' => {
                    map.insert(position, SeacucumberDirection::South);
                }
                _ => {}
            };
        })
    });

    map
}

fn solve_part1(inputfile: String) -> usize {
    let mut map = map_from_string(
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
    0
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
