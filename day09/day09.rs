use itertools::Itertools;
use std::collections::HashMap;
use std::env;

type Coordinate = (i32, i32);

struct AdjacentRange {
    start: Coordinate,
    index: usize,
}
impl AdjacentRange {
    fn new(position: Coordinate) -> Self {
        Self {
            start: position,
            index: 0,
        }
    }
}

impl Iterator for AdjacentRange {
    type Item = Coordinate;
    fn next(&mut self) -> Option<Coordinate> {
        let adjacents = [(-1, 0), (1, 0), (0, 1), (0, -1)];

        if self.index == adjacents.len() {
            return None;
        }

        let next = (
            self.start.0 + adjacents[self.index].0,
            self.start.1 + adjacents[self.index].1,
        );

        self.index += 1;
        Some(next)
    }
}

fn parse_input(inputfile: String) -> HashMap<Coordinate, u32> {
    let contents =
        std::fs::read_to_string(inputfile).expect("Something went wrong reading the file");

    let mut height_map = HashMap::new();

    contents.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            let position: Coordinate = (x as i32, y as i32);
            height_map
                .entry(position)
                .or_insert(c.to_digit(10).unwrap());
        })
    });
    height_map
}

fn solve_part1(inputfile: String) -> usize {
    let height_map = parse_input(inputfile);

    height_map.iter().fold(0, |acc, (position, height)| {
        let is_lowest =
            AdjacentRange::new(*position).all(|neighbour| match height_map.get(&neighbour) {
                Some(neighbour_height) => neighbour_height > height,
                None => true,
            });

        acc + (if is_lowest { height + 1 } else { 0 }) as usize
    })
}

fn solve_part2(inputfile: String) -> usize {
    let height_map = parse_input(inputfile);

    let lowest_points = height_map
        .iter()
        .filter_map(|(position, height)| {
            let is_lowest =
                AdjacentRange::new(*position).all(|neighbour| match height_map.get(&neighbour) {
                    Some(neighbour_height) => neighbour_height > height,
                    None => true,
                });

            if is_lowest {
                Some((*position, *height))
            } else {
                None
            }
        })
        .collect::<Vec<(Coordinate, u32)>>();

    println!("lowest points: {:?}", lowest_points);

    lowest_points
        .iter()
        .map(|(lowest_point, height)| {
            let mut queue = vec![*lowest_point];
            let mut visited: Vec<Coordinate> = vec![];

            while !queue.is_empty() {
                match queue.pop() {
                    Some(position) => {
                        if !visited.contains(&position) {
                            visited.push(position);

                            for neighbour in AdjacentRange::new(position) {
                                match height_map.get(&neighbour) {
                                    Some(neighbour_height) => {
                                        if *neighbour_height > *height
                                            && *neighbour_height != 9
                                            && !visited.contains(&neighbour)
                                        {
                                            queue.push(neighbour);
                                        }
                                    }
                                    None => (),
                                }
                            }
                        }
                    }
                    None => (),
                }
            }

            println!(
                "lowest point: {:?}, basin[{}]: {:?}",
                lowest_point,
                visited.len(),
                visited
            );

            visited.len()
        })
        .sorted()
        .rev()
        .take(3)
        .product()
}

/*

use image::ImageBuffer;
type Color = (u8, u8, u8);

fn draw_pixel(pixels: &mut Vec<(Coordinate, Color)>, position: Coordinate, color_index: i32) {
    let color = match color_index {
        1 => (166, 145, 80),
        2 => (177, 157, 94),
        3 => (186, 168, 111),
        4 => (194, 178, 128),
        5 => (202, 188, 145),
        6 => (211, 199, 162),
        7 => (219, 209, 180),
        _ => (219, 209, 180),
    };

    pixels.push((position, color));
}

fn draw_vent_map(inputfile: String) {
    let input = parse_input(inputfile);

    let mut vent_map = HashMap::<Coordinate, i32>::new();

    for (start, stop) in input {
        if (start.0 == stop.0)
            || (start.1 == stop.1)
            || ((start.0 - stop.0).abs() == (start.1 - stop.1).abs())
        {
            for coord in CoordinateRange::new(start, stop) {
                vent_map
                    .entry(coord)
                    .and_modify(|e| *e = *e + 1)
                    .or_insert(1);
            }
        }
    }

    let x_min = vent_map.iter().map(|(pos, _)| pos.0).min().unwrap();
    let x_max = vent_map.iter().map(|(pos, _)| pos.0).max().unwrap();
    let y_min = vent_map.iter().map(|(pos, _)| pos.1).min().unwrap();
    let y_max = vent_map.iter().map(|(pos, _)| pos.1).max().unwrap();
    let x_range = (x_max - x_min) as u32;
    let y_range = (y_max - y_min) as u32;
    let dimensions: Coordinate = (1 + x_range as i32, 1 + y_range as i32);

    let border = 2;
    let real_size = (
        ((dimensions.0 + border * 2) as u32),
        ((dimensions.1 + border * 2) as u32),
    );

    let offset = (x_min + border, y_min + border);

    let mut pixels = Vec::<(Coordinate, Color)>::new();

    for y in 0..y_range {
        for x in 0..x_range {
            let block_pos = (border + x as i32, border + y as i32);
            let vent_pos = (offset.0 + block_pos.0, offset.1 + block_pos.1);

            match vent_map.get(&vent_pos) {
                Some(&height) => draw_pixel(&mut pixels, block_pos, height),
                None => draw_pixel(&mut pixels, block_pos, 0),
            }
        }
    }

    let mut img = ImageBuffer::from_fn(real_size.0, real_size.1, |_x, _y| {
        image::Rgb([255, 255, 255])
    });

    for ((x, y), color) in pixels {
        let pixel = image::Rgb([color.0, color.1, color.2]);
        if x >= 0 && y >= 0 && x < real_size.0 as i32 && y < real_size.1 as i32 {
            img.put_pixel(x as u32, y as u32, pixel);
        }
    }

    img.save(format!("frames/day05.png")).unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Part1: {}", solve_part1(args[1].to_string()));
    println!("Part2: {}", solve_part2(args[1].to_string()));

    draw_vent_map(args[1].to_string());
}

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
*/

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Part1: {}", solve_part1(args[1].to_string()));
    println!("Part2: {}", solve_part2(args[1].to_string()));
}
