use image::ImageBuffer;
use itertools::Itertools;
use std::collections::HashMap;
use std::env;

type Coordinate = (i32, i32);

struct CoordinateRange {
    start: Coordinate,
    current: Option<Coordinate>,
    end: Coordinate,
}
impl CoordinateRange {
    fn new(start_pos: Coordinate, end_pos: Coordinate) -> CoordinateRange {
        CoordinateRange {
            start: start_pos,
            current: None,
            end: end_pos,
        }
    }
}

impl Iterator for CoordinateRange {
    type Item = Coordinate;
    fn next(&mut self) -> Option<Coordinate> {
        match &mut self.current {
            None => {
                self.current = Some(self.start);
                self.current
            }
            Some(current) => {
                if current.0 == self.end.0 && current.1 == self.end.1 {
                    return None;
                }

                if current.0 != self.end.0 {
                    if current.0 > self.end.0 {
                        current.0 -= 1;
                    } else {
                        current.0 += 1;
                    }
                }

                if current.1 != self.end.1 {
                    if current.1 > self.end.1 {
                        current.1 -= 1;
                    } else {
                        current.1 += 1;
                    }
                }

                self.current
            }
        }
    }
}

fn parse_input(inputfile: String) -> Vec<(Coordinate, Coordinate)> {
    let contents =
        std::fs::read_to_string(inputfile).expect("Something went wrong reading the file");

    contents
        .lines()
        .map(|line| {
            let (start, stop) = line
                .split(" -> ")
                .map(|pos| {
                    pos.split(",")
                        .map(|value| value.parse::<i32>().unwrap())
                        .tuples()
                        .next()
                        .unwrap()
                })
                .tuples()
                .next()
                .unwrap();

            (start, stop)
        })
        .collect::<Vec<(Coordinate, Coordinate)>>()
}

fn solve_part1(inputfile: String) -> usize {
    let input = parse_input(inputfile);

    let mut vent_map = HashMap::<Coordinate, i32>::new();

    for (start, stop) in input {
        if (start.0 == stop.0) || (start.1 == stop.1) {
            for coord in CoordinateRange::new(start, stop) {
                vent_map
                    .entry(coord)
                    .and_modify(|e| *e = *e + 1)
                    .or_insert(1);
            }
        }
    }

    vent_map
        .iter()
        .fold(0, |acc, (_, &value)| if value > 1 { acc + 1 } else { acc })
}

fn solve_part2(inputfile: String) -> usize {
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
        } else {
            println!(
                " strange line {:?}, {:?}",
                (start.0 - stop.0).abs(),
                (start.1 - stop.1).abs()
            );
        }
    }

    vent_map
        .iter()
        .fold(0, |acc, (_, &value)| if value > 1 { acc + 1 } else { acc })
}

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
