use itertools::Itertools;
use regex::Regex;
use std::env;

fn solve_part1(inputfile: String) -> usize {
    let contents =
        std::fs::read_to_string(inputfile).expect("Something went wrong reading the file");

    let re = Regex::new(r"(?P<command>(forward|down|up))\s(?P<units>\d+)").unwrap();

    let mut horizontal = 0;
    let mut depth = 0;
    contents.lines().for_each(|line| {
        let parsed = re.captures(line).unwrap();
        let units = parsed["units"].parse::<usize>().unwrap();
        match &parsed["command"] {
            "forward" => {
                horizontal += units;
            }
            "down" => {
                depth += units;
            }
            "up" => {
                depth -= units;
            }
            _ => {
                println!("unknown command");
            }
        };
    });
    horizontal * depth
}

fn solve_part2(inputfile: String) -> usize {
    let contents =
        std::fs::read_to_string(inputfile).expect("Something went wrong reading the file");

    let re = Regex::new(r"(?P<command>(forward|down|up))\s(?P<units>\d+)").unwrap();

    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;
    contents.lines().for_each(|line| {
        let parsed = re.captures(line).unwrap();
        let units = parsed["units"].parse::<usize>().unwrap();
        match &parsed["command"] {
            "forward" => {
                horizontal += units;
                depth += aim * units;
            }
            "down" => {
                aim += units;
            }
            "up" => {
                aim -= units;
            }
            _ => {
                println!("unknown command");
            }
        };
    });
    horizontal * depth
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Part1: {}", solve_part1(args[1].to_string()));
    println!("Part2: {}", solve_part2(args[1].to_string()));
}
