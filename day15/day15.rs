use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};

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
            (-1, 0),
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

fn draw_pixel(
    pixels: &mut Vec<(Coordinate, Color)>,
    position: Coordinate,
    block_size: u32,
    has_border: bool,
    color_index: usize,
) {
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
        (228, 90, 120),
    ];

    let default_color = palette[0];
    let border_color = palette[11];

    let color = match palette.get(color_index) {
        Some(valid_color) => *valid_color,
        None => default_color,
    };

    for offset_y in 0..block_size {
        for offset_x in 0..block_size {
            if has_border
                && ((offset_y == 0 || offset_y == block_size - 1)
                    || (offset_x == 0 || offset_x == block_size - 1))
            {
                pixels.push((
                    (
                        (block_size * position.0 as u32 + offset_x) as i32,
                        (block_size * position.1 as u32 + offset_y) as i32,
                    ),
                    border_color,
                ));
            } else {
                pixels.push((
                    (
                        (block_size * position.0 as u32 + offset_x) as i32,
                        (block_size * position.1 as u32 + offset_y) as i32,
                    ),
                    color,
                ));
            }
        }
    }
}

fn draw_image(image_data: &HashMap<Coordinate, u32>, path: &Vec<Coordinate>, frame: u32) {
    let x_min = image_data.iter().map(|(pos, _)| pos.0).min().unwrap();
    let x_max = image_data.iter().map(|(pos, _)| pos.0).max().unwrap();
    let y_min = image_data.iter().map(|(pos, _)| pos.1).min().unwrap();
    let y_max = image_data.iter().map(|(pos, _)| pos.1).max().unwrap();
    let x_range = (1 + x_max - x_min) as u32;
    let y_range = (1 + y_max - y_min) as u32;
    let dimensions: Coordinate = (x_range as i32, y_range as i32);

    let border = 2;
    let block_size = 8;
    let scale = 1;
    let real_size = (
        (scale * block_size * (dimensions.0 + border * 2) as u32),
        (scale * block_size * (dimensions.1 + border * 2) as u32),
    );

    // Translate value to a color from a palette
    let mut pixels = Vec::<(Coordinate, Color)>::new();

    for y in 0..y_range {
        for x in 0..x_range {
            let block_pos = (border + x as i32, border + y as i32);
            let pos = (x as i32, y as i32);

            match image_data.get(&pos) {
                Some(&value) => draw_pixel(
                    &mut pixels,
                    block_pos,
                    block_size,
                    path.contains(&pos),
                    value as usize,
                ),
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

    img.save(format!("frames/day15.frame{:05}.png", frame));
}

fn get_wrapped_position(position: &Coordinate, tile_size: &(i32, i32)) -> Coordinate {
    (position.0 % tile_size.0, position.1 % tile_size.1)
}

fn get_extra_cost(position: &Coordinate, tile_size: &(i32, i32)) -> u32 {
    (position.0 / tile_size.0 + position.1 / tile_size.1) as u32
}

fn get_cost(cost: &u32, position: &Coordinate, tile_size: &(i32, i32)) -> u32 {
    1 + ((cost - 1) + get_extra_cost(position, tile_size)) % 9
}

fn draw_image_all_tiles(
    image_data: &HashMap<Coordinate, u32>,
    path: &Vec<Coordinate>,
    tiles: &(i32, i32),
    frame: u32,
) {
    let x_min = image_data.iter().map(|(pos, _)| pos.0).min().unwrap();
    let x_max = image_data.iter().map(|(pos, _)| pos.0).max().unwrap();
    let y_min = image_data.iter().map(|(pos, _)| pos.1).min().unwrap();
    let y_max = image_data.iter().map(|(pos, _)| pos.1).max().unwrap();
    let tile_x_range = (1 + x_max - x_min);
    let tile_y_range = (1 + y_max - y_min);
    let tile_size = (tile_x_range as i32, tile_y_range as i32);
    let x_range = (tiles.0 * tile_x_range) as u32;
    let y_range = (tiles.1 * tile_x_range) as u32;

    let dimensions: Coordinate = (x_range as i32, y_range as i32);

    let border = 2;
    let block_size = 4;
    let scale = 1;
    let real_size = (
        (scale * block_size * (dimensions.0 + border * 2) as u32),
        (scale * block_size * (dimensions.1 + border * 2) as u32),
    );

    // Translate value to a color from a palette
    let mut pixels = Vec::<(Coordinate, Color)>::new();

    for y in 0..y_range {
        for x in 0..x_range {
            let block_pos = (border + x as i32, border + y as i32);
            let pos = (x as i32, y as i32);
            let wrapped_pos = get_wrapped_position(&pos, &tile_size);

            match image_data.get(&wrapped_pos) {
                Some(&value) => draw_pixel(
                    &mut pixels,
                    block_pos,
                    block_size,
                    path.contains(&pos),
                    get_cost(&value, &pos, &tile_size) as usize,
                ),
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

    img.save(format!("frames/day15.frame{:05}.png", frame));
}

fn get_shortest_path(map: &HashMap<Coordinate, u32>, start: Coordinate) -> u32 {
    let x_max = map.iter().map(|(pos, _)| pos.0).max().unwrap();
    let y_max = map.iter().map(|(pos, _)| pos.1).max().unwrap();

    let end: Coordinate = (x_max, y_max);

    let mut queue: Vec<(Coordinate, u32)> = vec![(start, 0)];
    let mut came_from: HashMap<Coordinate, Coordinate> = HashMap::new();
    let mut costs: HashMap<Coordinate, u32> = HashMap::from([(start, 0)]);

    let mut lowest_cost = 0;
    let mut frame = 0;
    draw_image(map, &vec![], frame);
    while !queue.is_empty() {
        queue.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        match queue.pop() {
            Some((position, priority)) => {
                if position == end {
                    println!("found end: {:?}", (position, priority));
                    if priority < lowest_cost || lowest_cost == 0 {
                        lowest_cost = priority;
                        println!("found better path {}", priority);
                    }

                    {
                        frame += 1;
                        let mut path = vec![position];
                        while path.last().unwrap() != &start {
                            path.push(*came_from.get(path.last().unwrap()).unwrap());
                        }
                        draw_image(map, &path, frame);
                    }
                }

                for neighbour in AdjacentRange::new(position) {
                    match map.get(&neighbour) {
                        Some(tile_cost) => {
                            let next_cost = costs.get(&position).unwrap_or(&0) + *tile_cost;
                            let should_add_to_queue = match costs.get(&neighbour) {
                                Some(neighbour_cost) => next_cost < *neighbour_cost,
                                None => true,
                            };

                            if should_add_to_queue {
                                costs
                                    .entry(neighbour)
                                    .and_modify(|e| *e = next_cost)
                                    .or_insert(next_cost);
                                came_from
                                    .entry(neighbour)
                                    .and_modify(|e| *e = position)
                                    .or_insert(position);

                                let manhattan_cost = (neighbour.1..end.1)
                                    .zip(neighbour.0..end.0)
                                    .fold(0, |acc, pos| {
                                        acc + match map.get(&pos) {
                                            Some(cost) => *cost,
                                            None => 0,
                                        }
                                    });
                                queue.push((neighbour, next_cost + manhattan_cost));
                            }
                        }
                        None => (),
                    }
                }
            }
            None => (),
        }
    }

    lowest_cost
}

fn get_shortest_path_all_tiles(
    map: &HashMap<Coordinate, u32>,
    start: Coordinate,
    tiles: (i32, i32),
) -> u32 {
    let x_max = map.iter().map(|(pos, _)| pos.0).max().unwrap();
    let y_max = map.iter().map(|(pos, _)| pos.1).max().unwrap();

    let tile_size = (x_max + 1, y_max + 1);

    let end: Coordinate = (
        tile_size.0 * (tiles.0 - 1) + x_max,
        (tile_size.1 * (tiles.1 - 1) + x_max),
    );

    let mut queue: Vec<(Coordinate, u32)> = vec![(start, 0)];
    let mut came_from: HashMap<Coordinate, Coordinate> = HashMap::new();
    let mut costs: HashMap<Coordinate, u32> = HashMap::from([(start, 0)]);

    let mut frame = 0;
    draw_image_all_tiles(map, &vec![], &tiles, frame);
    println!("Searching");

    let mut lowest_cost = 0;
    while !queue.is_empty() {
        queue.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        match queue.pop() {
            Some((position, priority)) => {
                if position == end {
                    println!("found end: {:?}", (position, priority));
                    if priority < lowest_cost || lowest_cost == 0 {
                        lowest_cost = priority;
                        println!("found better path {}", priority);
                    }
                    {
                        let mut path = vec![position];
                        while path.last().unwrap() != &start {
                            path.push(*came_from.get(path.last().unwrap()).unwrap());
                        }
                        frame += 1;
                        draw_image_all_tiles(map, &path, &tiles, frame);
                    }
                }

                let wrapped_position = get_wrapped_position(&position, &tile_size);
                for neighbour in AdjacentRange::new(position) {
                    if neighbour.0 > end.0 || neighbour.1 > end.1 {
                        continue;
                    }
                    let wrapped_neighbour = get_wrapped_position(&neighbour, &tile_size);

                    match map.get(&wrapped_neighbour) {
                        Some(tile_cost) => {
                            let unwrapped_tile_cost = get_cost(tile_cost, &neighbour, &tile_size);
                            let next_cost = costs.get(&position).unwrap() + unwrapped_tile_cost;
                            let should_add_to_queue = match costs.get(&neighbour) {
                                Some(neighbour_cost) => next_cost < *neighbour_cost,
                                None => true,
                            };

                            if should_add_to_queue {
                                costs
                                    .entry(neighbour)
                                    .and_modify(|e| *e = next_cost)
                                    .or_insert(next_cost);
                                came_from
                                    .entry(neighbour)
                                    .and_modify(|e| *e = position)
                                    .or_insert(position);

                                let manhattan_cost = (neighbour.1..end.1)
                                    .zip(neighbour.0..end.0)
                                    .fold(0, |acc, pos| {
                                        let wrapped_position =
                                            get_wrapped_position(&pos, &tile_size);
                                        acc + match map.get(&wrapped_position) {
                                            Some(risk_level) => {
                                                get_cost(risk_level, &pos, &tile_size)
                                            }
                                            None => 0,
                                        }
                                    });
                                queue.push((neighbour, next_cost + manhattan_cost));
                            }
                        }
                        None => (),
                    }
                }
            }
            None => (),
        }
    }

    lowest_cost
}

fn solve_part1(inputfile: String) -> u32 {
    let mut map = parse_input(inputfile);
    get_shortest_path(&map, (0, 0))
}

fn solve_part2(inputfile: String) -> u32 {
    let mut map = parse_input(inputfile);
    get_shortest_path_all_tiles(&map, (0, 0), (5, 5))
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Part1: {}", solve_part1(args[1].to_string()));
    println!("Part2: {}", solve_part2(args[1].to_string()));
}
