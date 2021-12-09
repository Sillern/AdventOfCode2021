use itertools::Itertools;
use std::collections::HashMap;
use std::env;

fn solve_part1(inputfile: String) -> usize {
    let contents =
        std::fs::read_to_string(inputfile).expect("Something went wrong reading the file");

    let mut fishes = contents
        .lines()
        .next()
        .unwrap()
        .split(",")
        .map(|cycletime| cycletime.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    for _ in 0..80 {
        let new_fishes = fishes
            .iter_mut()
            .filter_map(|cycle_time| {
                if *cycle_time == 0 {
                    *cycle_time = 6;
                    Some(8)
                } else {
                    *cycle_time -= 1;
                    None
                }
            })
            .collect::<Vec<usize>>();
        fishes.extend(new_fishes);
    }

    fishes.len()
}

fn solve_part2(inputfile: String) -> usize {
    let contents =
        std::fs::read_to_string(inputfile).expect("Something went wrong reading the file");

    let mut fish_buckets = contents
        .lines()
        .next()
        .unwrap()
        .split(",")
        .map(|cycletime| cycletime.parse::<usize>().unwrap())
        .sorted()
        .into_iter()
        .dedup_with_count()
        .map(|(num_fish, cycle_time)| (cycle_time, num_fish))
        .collect::<HashMap<usize, usize>>();

    for _ in 0..256 {
        let new_fish = match fish_buckets.get(&0) {
            Some(&count) => count,
            None => 0,
        };

        for cycle_time in 0..8 {
            let next_fish_count = match fish_buckets.get(&(cycle_time + 1)) {
                Some(&count) => count,
                None => 0,
            };

            fish_buckets
                .entry(cycle_time)
                .and_modify(|e| *e = next_fish_count)
                .or_insert(next_fish_count);
        }

        fish_buckets
            .entry(6)
            .and_modify(|e| *e += new_fish)
            .or_insert(0);
        fish_buckets
            .entry(8)
            .and_modify(|e| *e = new_fish)
            .or_insert(0);
    }

    fish_buckets.values().fold(0, |acc, bucket| acc + bucket)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Part1: {}", solve_part1(args[1].to_string()));
    println!("Part2: {}", solve_part2(args[1].to_string()));
}
