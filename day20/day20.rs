
use std::collections::HashMap;
use std::env;

type Coordinate = (i32, i32);

#[derive(Debug)]
struct ImageEnhancer {
    enhance: HashMap<usize, bool>,
}

impl ImageEnhancer {
    pub fn from_string(input: &str) -> Self {
        Self {
            enhance: input
                .chars()
                .enumerate()
                .map(|(index, c)| (index, c == '#'))
                .collect(),
        }
    }

    pub fn has_detail(&self, image: &Image, pixel: Coordinate, flipped_boundary: bool) -> bool {
        let x_min = image.pixels.iter().map(|(pos, _)| pos.0).min().unwrap() as i32;
        let x_max = image.pixels.iter().map(|(pos, _)| pos.0).max().unwrap() as i32;
        let y_min = image.pixels.iter().map(|(pos, _)| pos.1).min().unwrap() as i32;
        let y_max = image.pixels.iter().map(|(pos, _)| pos.1).max().unwrap() as i32;

        let lookup_index =
            AdjacentPixels::new(pixel)
                .enumerate()
                .fold(0, |acc, (index, position)| {
                    let (x, y) = position;
                    let value = if x < x_min || x > x_max || y < y_min || y > y_max {
                        if flipped_boundary {
                            1
                        } else {
                            0
                        }
                    } else if let Some(is_lit) = image.pixels.get(&position) {
                        if *is_lit {
                            1
                        } else {
                            0
                        }
                    } else {
                        0
                    };

                    acc + (value << (8 - index))
                });

        if let Some(has_detail) = self.enhance.get(&lookup_index) {
            *has_detail
        } else {
            false
        }
    }
}

#[derive(Debug)]
struct Image {
    pixels: HashMap<Coordinate, bool>,
}

impl Image {
    pub fn from_string(input: &str) -> Self {
        Self {
            pixels: input
                .lines()
                .enumerate()
                .map(|(y, row)| {
                    row.chars()
                        .enumerate()
                        .filter_map(|(x, c)| {
                            if c == '#' {
                                Some(((x as i32, y as i32), true))
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<(Coordinate, bool)>>()
                })
                .flatten()
                .collect(),
        }
    }

    pub fn enhance(&mut self, enhancer: &ImageEnhancer, flip_boundary: bool) {
        let x_min = self.pixels.iter().map(|(pos, _)| pos.0).min().unwrap() as i32;
        let x_max = self.pixels.iter().map(|(pos, _)| pos.0).max().unwrap() as i32;
        let y_min = self.pixels.iter().map(|(pos, _)| pos.1).min().unwrap() as i32;
        let y_max = self.pixels.iter().map(|(pos, _)| pos.1).max().unwrap() as i32;

        self.pixels = ((y_min - 3)..(y_max + 4))
            .map(|y| {
                ((x_min - 3)..(x_max + 4))
                    .map(|x| {
                        let pos = (x, y);
                        (pos, enhancer.has_detail(self, pos, flip_boundary))
                    })
                    .collect::<Vec<(Coordinate, bool)>>()
            })
            .flatten()
            .filter(|(_pos, has_value)| *has_value)
            .collect::<HashMap<Coordinate, bool>>();
    }
}

struct AdjacentPixels {
    start: Coordinate,
    index: usize,
}
impl AdjacentPixels {
    fn new(position: Coordinate) -> Self {
        Self {
            start: position,
            index: 0,
        }
    }
}

impl Iterator for AdjacentPixels {
    type Item = Coordinate;
    fn next(&mut self) -> Option<Coordinate> {
        let adjacents = [
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            (0, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
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

fn solve_part1(inputfile: String) -> usize {
    let text_parts = std::fs::read_to_string(inputfile)
        .expect("Something went wrong reading the file")
        .split("\n\n")
        .map(|blob| blob.to_string())
        .collect::<Vec<String>>();

    let image_enhancer = ImageEnhancer::from_string(&text_parts[0]);
    let mut image = Image::from_string(&text_parts[1]);

    println!(
        "next_pixel_len: {}",
        image.pixels.iter().filter(|&(_, pixel)| *pixel).count()
    );
    draw_image(&image, 0);
    for iteration in 1..3 {
        image.enhance(&image_enhancer, iteration % 2 == 0);
        draw_image(&image, iteration);
    }
    image.pixels.iter().filter(|&(_, pixel)| *pixel).count()
}

fn solve_part2(inputfile: String) -> usize {
    let text_parts = std::fs::read_to_string(inputfile)
        .expect("Something went wrong reading the file")
        .split("\n\n")
        .map(|blob| blob.to_string())
        .collect::<Vec<String>>();

    let image_enhancer = ImageEnhancer::from_string(&text_parts[0]);
    let mut image = Image::from_string(&text_parts[1]);

    println!(
        "next_pixel_len: {}",
        image.pixels.iter().filter(|&(_, pixel)| *pixel).count()
    );
    for iteration in 1..51 {
        image.enhance(&image_enhancer, iteration % 2 == 0);
        draw_image(&image, iteration);
        println!(
            "next_pixel_len[{}]: {}",
            iteration,
            image.pixels.iter().filter(|&(_, pixel)| *pixel).count()
        );
    }
    image.pixels.iter().filter(|&(_, pixel)| *pixel).count()
}

use image::ImageBuffer;
type Color = (u8, u8, u8);

fn draw_pixel(
    pixels: &mut Vec<(Coordinate, Color)>,
    position: Coordinate,
    block_size: i32,
    index: usize,
) {
    let color: Color = (
        ((30 + 2 * index) % 256) as u8,
        ((10 + 2 * index) % 256) as u8,
        ((22 + 1 * index) % 256) as u8,
    );

    for offset_y in 0..block_size {
        for offset_x in 0..block_size {
            pixels.push((
                (
                    (block_size * position.0 + offset_x),
                    (block_size * position.1 + offset_y),
                ),
                color,
            ));
        }
    }
}

fn draw_image(image: &Image, frame: u32) {
    let x_min = image.pixels.iter().map(|(pos, _)| pos.0).min().unwrap();
    let x_max = image.pixels.iter().map(|(pos, _)| pos.0).max().unwrap();
    let y_min = image.pixels.iter().map(|(pos, _)| pos.1).min().unwrap();
    let y_max = image.pixels.iter().map(|(pos, _)| pos.1).max().unwrap();
    let x_range = 1 + x_max - x_min;
    let y_range = 1 + y_max - y_min;
    let dimensions: Coordinate = (x_range, y_range);

    let border = 2;
    let block_size: i32 = 3;
    let scale = 4;
    let virtual_size = (
        (block_size * (dimensions.0 + border * 2)),
        (block_size * (dimensions.1 + border * 2)),
    );
    let real_size = ((scale * virtual_size.0), (scale * virtual_size.1));

    // Translate value to a color from a palette
    let mut pixels = Vec::<(Coordinate, Color)>::new();

    for (pos, has_value) in image.pixels.iter() {
        if *has_value {
            draw_pixel(&mut pixels, *pos, block_size, 40);
        }
    }

    let mut img = ImageBuffer::from_fn(real_size.0 as u32, real_size.1 as u32, |_x, _y| {
        image::Rgb([255, 255, 255])
    });

    for ((x_, y_), color) in pixels {
        let pixel = image::Rgb([color.0, color.1, color.2]);
        let (x, y) = (
            x_ - ((x_min - border) * block_size),
            (y_ - y_min * block_size) + border * block_size,
        );

        if x >= 0 && y >= 0 && x < real_size.0 && y < real_size.1 {
            for offset_y in 0..scale {
                for offset_x in 0..scale {
                    img.put_pixel(
                        (scale * x + offset_x) as u32,
                        (scale * y + offset_y) as u32,
                        pixel,
                    );
                }
            }
        } else {
            println!(
                "out of boundary {:?} in {:?} {:?}",
                ((x, y), (x_, (y_ - (y_min - border)))),
                (x_range, y_range, virtual_size),
                (x_min, x_max, y_min, y_max)
            );
            panic!();
        }
    }

    img.save(format!("frames/day20.frame{:05}.png", frame));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Part1: {}", solve_part1(args[1].to_string()));
    println!("Part2: {}", solve_part2(args[1].to_string()));
}

#[cfg(test)]
mod tests {
    
    #[test]
    fn test_small_example() {}
}
