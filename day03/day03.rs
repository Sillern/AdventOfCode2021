use std::env;
use std::i64;

fn solve_part1(inputfile: String) -> usize {
    let contents =
        std::fs::read_to_string(inputfile).expect("Something went wrong reading the file");

    let mut num_lines = 1;
    let accumulated_bits = contents
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .reduce(|acc, bits| {
            num_lines += 1;
            acc.into_iter()
                .zip(bits)
                .map(|(acc, bit)| acc + bit)
                .collect::<Vec<u32>>()
        })
        .unwrap()
        .into_iter()
        .map(|x| (2 * x) / num_lines)
        .collect::<Vec<u32>>();

    let num_bits = accumulated_bits.len() - 1;
    let (gamma, epsilon) =
        accumulated_bits
            .into_iter()
            .enumerate()
            .fold((0, 0), |(gamma, epsilon), (index, bit)| {
                (
                    gamma + 2usize.pow((num_bits - index) as u32) * bit as usize,
                    epsilon + 2usize.pow((num_bits - index) as u32) * (1 - bit) as usize,
                )
            });

    println!("gamma, epsilon: {}, {}", gamma, epsilon);

    gamma * epsilon
}

fn solve_part2(inputfile: String) -> usize {
    let contents =
        std::fs::read_to_string(inputfile).expect("Something went wrong reading the file");

    let mut oxygen_generator_rating = contents.lines().collect::<Vec<&str>>();
    let mut split_index = 0;

    while oxygen_generator_rating.len() != 1 {
        let (ones, zeroes): (Vec<&str>, Vec<&str>) = oxygen_generator_rating
            .into_iter()
            .partition(|&line| line.chars().nth(split_index) == Some('1'));

        oxygen_generator_rating = if ones.len() >= zeroes.len() {
            ones
        } else {
            zeroes
        };
        split_index += 1;
    }

    let o2_rating = i64::from_str_radix(oxygen_generator_rating[0], 2).unwrap();

    let mut co2_scrubber_rating = contents.lines().collect::<Vec<&str>>();
    split_index = 0;

    while co2_scrubber_rating.len() != 1 {
        let (ones, zeroes): (Vec<&str>, Vec<&str>) = co2_scrubber_rating
            .into_iter()
            .partition(|&line| line.chars().nth(split_index) == Some('1'));

        co2_scrubber_rating = if ones.len() < zeroes.len() {
            ones
        } else {
            zeroes
        };
        split_index += 1;
    }

    let co2_rating = i64::from_str_radix(co2_scrubber_rating[0], 2).unwrap();

    (o2_rating * co2_rating) as usize
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Part1: {}", solve_part1(args[1].to_string()));
    println!("Part2: {}", solve_part2(args[1].to_string()));
}
