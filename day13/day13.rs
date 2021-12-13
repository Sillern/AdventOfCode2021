use itertools::Itertools;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use std::collections::HashMap;
use std::env;

type Coordinate = (i32, i32);
type OptionalCoordinate = (Option<i32>, Option<i32>);

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
                None => {}
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

    img.save(format!("frames/day13.frame{:05}.png", frame));
}

fn parse_input(inputfile: String) -> (Vec<Coordinate>, Vec<OptionalCoordinate>) {
    let mut coordinates: Vec<Coordinate> = vec![];
    let mut fold_along: Vec<OptionalCoordinate> = vec![];

    let contents = std::fs::read_to_string(inputfile)
        .expect("Something went wrong reading the file")
        .lines()
        .for_each(|line| {
            if line.contains(",") {
                let tokens = line
                    .split(",")
                    .map(|token| token.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>();
                coordinates.push((tokens[0], tokens[1]));
            } else if line.contains("=") {
                line.split(" ")
                    .filter(|token| token.contains("="))
                    .for_each(|token| {
                        let mut parts = token.split("=");
                        let axis = parts.next().unwrap();
                        let value = parts.next().unwrap().parse::<i32>().unwrap();

                        fold_along.push(match axis {
                            "x" => (Some(value), None),
                            "y" => (None, Some(value)),
                            _ => panic!(),
                        });
                    });
            }
        });

    (coordinates, fold_along)
}

fn fold_paper(paper: &Vec<Coordinate>, axis: &OptionalCoordinate) -> Vec<Coordinate> {
    if let Some(fold_x) = axis.0 {
        println!("x: {:?}", fold_x);
        paper
            .iter()
            .map(|&(x, y)| {
                if x > fold_x {
                    (fold_x - (x - fold_x), y)
                } else {
                    (x, y)
                }
            })
            .sorted()
            .unique()
            .collect()
    } else if let Some(fold_y) = axis.1 {
        println!("y: {:?}", fold_y);
        paper
            .iter()
            .map(|&(x, y)| {
                if y > fold_y {
                    println!(
                        "new {:?} -> {:?}, diff: {:?}",
                        (x, y),
                        (x, fold_y - (y - fold_y)),
                        (y - fold_y)
                    );
                    (x, fold_y - (y - fold_y))
                } else {
                    (x, y)
                }
            })
            .sorted()
            .unique()
            .collect()
    } else {
        panic!()
    }
}

fn solve_part1(inputfile: String) -> usize {
    let (mut coordinates, fold_along) = parse_input(inputfile);

    let mut num_dots = 0;
    for axis in &fold_along {
        coordinates = fold_paper(&coordinates, axis);
        num_dots = coordinates.len();
        break;
    }

    num_dots
}

fn solve_part2(inputfile: String) -> usize {
    let (mut coordinates, fold_along) = parse_input(inputfile);
    let mut frame = 0;

    println!(
        "coordinates: {:?}, fold_along: {:?}",
        coordinates, fold_along
    );
    draw_image(
        &coordinates
            .iter()
            .map(|coord| (coord.clone(), 1))
            .collect::<HashMap<Coordinate, u32>>(),
        frame,
    );

    frame += 1;
    for axis in &fold_along {
        coordinates = fold_paper(&coordinates, axis);
        draw_image(
            &coordinates
                .iter()
                .map(|coord| (coord.clone(), 1))
                .collect::<HashMap<Coordinate, u32>>(),
            frame,
        );
        frame += 1;
        println!(
            "coordinates: {:?}, fold_along: {:?}",
            coordinates, fold_along
        );
    }

    0
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Part1: {}", solve_part1(args[1].to_string()));
    println!("Part2: {}", solve_part2(args[1].to_string()));
}
