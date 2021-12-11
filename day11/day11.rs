use rand::seq::SliceRandom;

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
        let adjacents = [
            // (0, 0),
            (0, 1),
            (0, -1),
            (1, 0),
            (1, 1),
            (1, -1),
            (-1, 0),
            (-1, 1),
            (-1, -1),
        ];

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

    let mut grid = HashMap::new();

    contents.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            let position: Coordinate = (x as i32, y as i32);
            grid.entry(position).or_insert(c.to_digit(10).unwrap());
        })
    });
    grid
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

fn draw_image(image_data: &HashMap<Coordinate, u32>, frame: u32) {
    let x_min = image_data.iter().map(|(pos, _)| pos.0).min().unwrap();
    let x_max = image_data.iter().map(|(pos, _)| pos.0).max().unwrap();
    let y_min = image_data.iter().map(|(pos, _)| pos.1).min().unwrap();
    let y_max = image_data.iter().map(|(pos, _)| pos.1).max().unwrap();
    let x_range = (1 + x_max - x_min) as u32;
    let y_range = (1 + y_max - y_min) as u32;
    let dimensions: Coordinate = (x_range as i32, y_range as i32);

    let border = 2;
    let scale = 4;
    let real_size = (
        (scale * (dimensions.0 + border * 2) as u32),
        (scale * (dimensions.1 + border * 2) as u32),
    );

    // Translate value to a color from a palette
    let mut pixels = Vec::<(Coordinate, Color)>::new();

    for y in 0..y_range {
        for x in 0..x_range {
            let block_pos = (border + x as i32, border + y as i32);
            let pos = (x as i32, y as i32);

            match image_data.get(&pos) {
                Some(&value) => draw_pixel(&mut pixels, block_pos, value as usize),
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
            for offset_y in 0..scale {
                for offset_x in 0..scale {
                    img.put_pixel(
                        scale * x as u32 + offset_x,
                        scale * y as u32 + offset_y,
                        pixel,
                    );
                }
            }
        }
    }

    img.save(format!("frames/day11.frame{:05}.png", frame));
}

fn flash(octopus_grid: &mut HashMap<Coordinate, u32>) -> u32 {
    let mut queue = octopus_grid
        .iter()
        .filter_map(|(position, energy_level)| {
            if *energy_level > 9 {
                Some(*position)
            } else {
                None
            }
        })
        .collect::<Vec<Coordinate>>();

    let mut visited: Vec<Coordinate> = vec![];

    while !queue.is_empty() {
        match queue.pop() {
            Some(position) => {
                if !visited.contains(&position) {
                    visited.push(position);

                    for neighbour in AdjacentRange::new(position) {
                        match octopus_grid.get_mut(&neighbour) {
                            Some(value) => {
                                *value += 1;
                                if *value > 9 && !visited.contains(&neighbour) {
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
    let mut num_flashes = 0;
    octopus_grid.iter_mut().for_each(|(_, energy_level)| {
        if *energy_level > 9 {
            num_flashes += 1;
            *energy_level = 0;
        }
    });
    num_flashes
}

fn solve_part1(inputfile: String) -> usize {
    let mut octopus_grid = parse_input(inputfile);

    let mut total_flashes = 0;
    for _ in 0..100 {
        octopus_grid.iter_mut().for_each(|(_, energy_level)| {
            *energy_level += 1;
        });

        total_flashes += flash(&mut octopus_grid);
    }
    total_flashes as usize
}

fn solve_part2(inputfile: String) -> usize {
    let mut octopus_grid = parse_input(inputfile);
    let num_octopus = octopus_grid.len() as u32;

    let mut frame = 0;
    let mut is_synchronized = false;
    while !is_synchronized {
        frame += 1;

        octopus_grid.iter_mut().for_each(|(_, energy_level)| {
            *energy_level += 1;
        });

        is_synchronized = flash(&mut octopus_grid) == num_octopus;
    }
    frame
}

fn draw_large_image(width: u32, height: u32) {
    let mut rng = rand::thread_rng();

    let energy_levels = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    let mut grid = HashMap::new();
    for y in 0..height {
        for x in 0..width {
            let position: Coordinate = (x as i32, y as i32);
            grid.entry(position)
                .or_insert(*energy_levels.choose(&mut rng).unwrap());
        }
    }

    let mut frame = 0;
    draw_image(&grid, frame);
    for _ in 0..100 {
        println!("frame[{}]", frame);
        frame += 1;

        grid.iter_mut().for_each(|(_, energy_level)| {
            *energy_level += 1;
        });

        flash(&mut grid);

        draw_image(&grid, frame);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Part1: {}", solve_part1(args[1].to_string()));
    println!("Part2: {}", solve_part2(args[1].to_string()));

    draw_large_image(100, 100);
}
