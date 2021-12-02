use regex::Regex;
use std::env;

fn solve_part1(inputfile: String) -> usize {
    let contents =
        std::fs::read_to_string(inputfile).expect("Something went wrong reading the file");

    let re = Regex::new(r"(?P<command>(forward|down|up))\s(?P<units>\d+)").unwrap();

    let (horizontal, depth) = contents.lines().fold((0, 0), |(horizontal, depth), line| {
        let parsed = re.captures(line).unwrap();
        let units = parsed["units"].parse::<usize>().unwrap();
        match &parsed["command"] {
            "forward" => (horizontal + units, depth),
            "down" => (horizontal, depth + units),
            "up" => (horizontal, depth - units),
            _ => (horizontal, depth),
        }
    });
    horizontal * depth
}

fn solve_part2(inputfile: String) -> usize {
    let contents =
        std::fs::read_to_string(inputfile).expect("Something went wrong reading the file");

    let re = Regex::new(r"(?P<command>(forward|down|up))\s(?P<units>\d+)").unwrap();

    let (horizontal, depth, _) =
        contents
            .lines()
            .fold((0, 0, 0), |(horizontal, depth, aim), line| {
                let parsed = re.captures(line).unwrap();
                let units = parsed["units"].parse::<usize>().unwrap();
                match &parsed["command"] {
                    "forward" => (horizontal + units, depth + aim * units, aim),
                    "down" => (horizontal, depth, aim + units),
                    "up" => (horizontal, depth, aim - units),
                    _ => (horizontal, depth, aim),
                }
            });
    horizontal * depth
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Part1: {}", solve_part1(args[1].to_string()));
    println!("Part2: {}", solve_part2(args[1].to_string()));
}
