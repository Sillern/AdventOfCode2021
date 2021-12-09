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

            visited.len()
        })
        .sorted()
        .rev()
        .take(3)
        .product()
}

use image::ImageBuffer;
type Color = (u8, u8, u8);

fn draw_pixel(pixels: &mut Vec<(Coordinate, Color)>, position: Coordinate, color_index: usize) {
    let palette = [
        (23, 37, 23),
        (12, 57, 83),
        (9, 76, 114),
        (5, 90, 140),
        (2, 106, 167),
        (0, 121, 191),
        (41, 143, 202),
        (91, 164, 207),
        (139, 189, 217),
        (188, 217, 234),
        (228, 240, 246),
    ];

    let default_color = palette[0];

    let color = match palette.get(color_index) {
        Some(valid_color) => *valid_color,
        None => default_color,
    };

    pixels.push((position, color));
}

fn draw_height_map(inputfile: String) {
    let height_map = parse_input(inputfile);

    let x_min = height_map.iter().map(|(pos, _)| pos.0).min().unwrap();
    let x_max = height_map.iter().map(|(pos, _)| pos.0).max().unwrap();
    let y_min = height_map.iter().map(|(pos, _)| pos.1).min().unwrap();
    let y_max = height_map.iter().map(|(pos, _)| pos.1).max().unwrap();
    let x_range = (x_max - x_min) as u32;
    let y_range = (y_max - y_min) as u32;
    let dimensions: Coordinate = (1 + x_range as i32, 1 + y_range as i32);

    let border = 2;
    let scale = 2;
    let real_size = (
        (scale * (dimensions.0 + border * 2) as u32),
        (scale * (dimensions.1 + border * 2) as u32),
    );

    let mut pixels = Vec::<(Coordinate, Color)>::new();

    for y in 0..y_range {
        for x in 0..x_range {
            let block_pos = (border + x as i32, border + y as i32);
            let pos = (x as i32, y as i32);

            match height_map.get(&pos) {
                Some(&height) => draw_pixel(&mut pixels, block_pos, height as usize),
                None => {
                    println!("Didn't find {:?}", pos);
                }
            }
        }
    }

    let mut img = ImageBuffer::from_fn(real_size.0, real_size.1, |_x, _y| {
        image::Rgb([255, 255, 255])
    });

    for ((x, y), color) in pixels {
        let pixel = image::Rgb([color.0, color.1, color.2]);
        if x >= 0 && y >= 0 && x < real_size.0 as i32 && y < real_size.1 as i32 {
            img.put_pixel(scale * x as u32 + 0, scale * y as u32 + 0, pixel);
            img.put_pixel(scale * x as u32 + 1, scale * y as u32 + 0, pixel);
            img.put_pixel(scale * x as u32 + 0, scale * y as u32 + 1, pixel);
            img.put_pixel(scale * x as u32 + 1, scale * y as u32 + 1, pixel);
        }
    }

    img.save("frames/day09.png".to_string()).unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Part1: {}", solve_part1(args[1].to_string()));
    println!("Part2: {}", solve_part2(args[1].to_string()));

    draw_height_map(args[1].to_string());
}
