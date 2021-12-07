use std::env;

fn solve_part1(inputfile: String) -> usize {
    let contents =
        std::fs::read_to_string(inputfile).expect("Something went wrong reading the file");

    let positions = contents
        .lines()
        .next()
        .unwrap()
        .split(",")
        .map(|position| position.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let max_position = positions.iter().max().unwrap();

    let mut min_fuel_cost = 0;
    for aligned_position in 0..*max_position {
        let fuel_cost = positions.iter().fold(0, |total_fuel, position| {
            total_fuel + (*position as i64 - aligned_position as i64).abs()
        });
        if fuel_cost < min_fuel_cost || min_fuel_cost == 0 {
            min_fuel_cost = fuel_cost;
        }
    }

    min_fuel_cost as usize
}

fn solve_part2(inputfile: String) -> usize {
    let contents =
        std::fs::read_to_string(inputfile).expect("Something went wrong reading the file");

    let positions = contents
        .lines()
        .next()
        .unwrap()
        .split(",")
        .map(|position| position.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    let max_position = positions.iter().max().unwrap();

    let mut min_fuel_cost = 0;
    for aligned_position in 0..*max_position {
        let fuel_cost = positions.iter().fold(0, |total_fuel, position| {
            let distance = (*position as i64 - aligned_position as i64).abs();
            total_fuel + (distance * (distance + 1) / 2)
        });
        if fuel_cost < min_fuel_cost || min_fuel_cost == 0 {
            min_fuel_cost = fuel_cost;
        }
    }

    min_fuel_cost as usize
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Part1: {}", solve_part1(args[1].to_string()));
    println!("Part2: {}", solve_part2(args[1].to_string()));
}
