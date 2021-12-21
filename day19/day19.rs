use itertools::Itertools;
use std::collections::HashMap;
use std::env;
use std::fmt;
use std::ops;

type Coordinate = (i32, i32, i32);
#[derive(Debug)]
struct Scanner {
    position: Coordinate,
    id: i32,
    detections: Vec<Coordinate>,
}

impl Scanner {
    pub fn from_string(input: &str) -> Self {
        let mut it = input.lines();
        let name = it.next().unwrap();
        let detections = it
            .map(|line| {
                let values = line
                    .split(",")
                    .map(|value| value.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>();

                (
                    values[0],
                    values[1],
                    if values.len() == 3 { values[2] } else { 0 },
                )
            })
            .collect::<Vec<_>>();
        Self {
            position: (0, 0, 0),
            id: name
                .split(" ")
                .filter_map(|token| {
                    if let std::result::Result::Ok(result) = token.parse::<i32>() {
                        Some(result)
                    } else {
                        None
                    }
                })
                .next()
                .unwrap(),
            detections: detections,
        }
    }

    pub fn translate(&mut self, translation: Coordinate) {
        self.position = (
            self.position.0 + translation.0,
            self.position.1 + translation.1,
            self.position.2 + translation.2,
        );

        self.detections = self
            .detections
            .iter()
            .map(|pos| {
                (
                    pos.0 + translation.0,
                    pos.1 + translation.1,
                    pos.2 + translation.2,
                )
            })
            .collect();
    }

    pub fn flip_axis(&mut self, axis: Coordinate) {
        self.detections = self
            .detections
            .iter()
            .map(|pos| (axis.0 * pos.0, axis.1 * pos.1, axis.2 * pos.2))
            .collect::<Vec<Coordinate>>();
    }

    pub fn rotate(&mut self, rotation: &Vec<Vec<i32>>) {
        self.detections = self
            .detections
            .iter()
            .map(|pos| {
                rotation
                    .iter()
                    .map(|row| {
                        row.iter()
                            .zip(vec![pos.0, pos.1, pos.2])
                            .fold(0, |sum, (a, b)| sum + a * b)
                    })
                    .collect_tuple::<Coordinate>()
                    .unwrap()
            })
            .collect::<Vec<Coordinate>>();
        println!("rotated: {:?}", &self.detections);
    }

    pub fn add_to_map(
        &self,
        map: &mut HashMap<Coordinate, usize>,
        translation: Coordinate,
    ) -> usize {
        map.entry(translation).and_modify(|e| *e = 0).or_insert(0);

        self.detections.iter().fold(0, |mut acc, pos| {
            let translated_pos = (
                pos.0 + translation.0,
                pos.1 + translation.1,
                pos.2 + translation.2,
            );
            map.entry(translated_pos)
                .and_modify(|e| {
                    if *e == 1 {
                        acc += 1;
                    }
                    *e = 1;
                })
                .or_insert(1);
            acc
        })
    }

    pub fn print(&self) {
        let mut map = HashMap::new();
        let no_translation = (0, 0, 0);
        self.add_to_map(&mut map, no_translation);

        print_map(&map);
    }

    pub fn find_maximum_matching_points(&self, other: &Self) -> Coordinate {
        let mut max_matching_points = 0;
        let mut best_translation = (0, 0, 0);

        for self_offset in self.detections.iter() {
            for other_offset in other.detections.iter() {
                let offset = (
                    self_offset.0 - other_offset.0,
                    self_offset.1 - other_offset.1,
                    self_offset.2 - other_offset.2,
                );
                let matching_points = self.num_matching_points(other, offset);
                if matching_points > max_matching_points {
                    best_translation = offset;
                    max_matching_points = matching_points;
                }
            }
        }
        best_translation
    }

    pub fn find_match(&mut self, other: &Self, min_points: usize) -> Option<Coordinate> {
        let mut map = HashMap::new();
        map.entry(self.position).and_modify(|e| *e = 0).or_insert(0);

        self.detections.iter().for_each(|&pos| {
            map.entry(pos).and_modify(|e| *e = 1).or_insert(1);
        });

        let mut translation = (0, 0, 0);

        // for every rotation
        if ScannerOrientation::new(&other.detections).any(|other_detections| {
            let offsets = self
                .detections
                .iter()
                .map(|self_offset| {
                    other_detections
                        .iter()
                        .map(|other_offset| {
                            (
                                self_offset.0 - other_offset.0,
                                self_offset.1 - other_offset.1,
                                self_offset.2 - other_offset.2,
                            )
                        })
                        .collect::<Vec<Coordinate>>()
                })
                .flatten()
                .sorted()
                .unique()
                .collect::<Vec<Coordinate>>();

            offsets.iter().any(|offset| {
                let num_points = other_detections.iter().fold(0, |acc, pos| {
                    acc + match map.get(&(pos.0 + offset.0, pos.1 + offset.1, pos.2 + offset.2)) {
                        Some(0) => 0,
                        Some(1) => 1,
                        Some(_) => panic!(),
                        None => 0,
                    }
                });
                if num_points >= min_points {
                    println!("self[{}] to {}, offset: {:?}", self.id, other.id, offset);
                    translation = *offset;

                    self.detections = self
                        .detections
                        .iter()
                        .chain(
                            other_detections
                                .iter()
                                .map(|pos| (pos.0 + offset.0, pos.1 + offset.1, pos.2 + offset.2))
                                .collect::<Vec<Coordinate>>()
                                .iter(),
                        )
                        .sorted()
                        .unique()
                        .map(|pos| *pos)
                        .collect();

                    true
                } else {
                    false
                }
            })
        }) {
            Some(translation)
        } else {
            None
        }
    }

    pub fn num_matching_points(&self, other: &Self, translation: Coordinate) -> usize {
        let mut map = HashMap::new();
        let no_translation = (0, 0, 0);

        self.add_to_map(&mut map, no_translation) + other.add_to_map(&mut map, translation)
    }

    pub fn is_matching(&self, other: &Self, num_matching_points: usize) -> bool {
        let mut map = HashMap::new();
        let no_translation = (0, 0, 0);
        num_matching_points
            == self.add_to_map(&mut map, no_translation)
                + other.add_to_map(&mut map, no_translation)
    }
}

struct ScannerOrientation {
    start: Vec<Coordinate>,
    index: usize,
}

impl ScannerOrientation {
    fn new(detections: &Vec<Coordinate>) -> Self {
        Self {
            start: detections.clone(),
            index: 0,
        }
    }
}

impl Iterator for ScannerOrientation {
    type Item = Vec<Coordinate>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index == 48 {
            return None;
        }
        let next = self
            .start
            .iter()
            .map(|&(x, y, z)| match self.index {
                0 => (x, y, z),
                1 => (z, x, y),
                2 => (y, z, x),
                3 => (x, z, y),
                4 => (y, x, z),
                5 => (z, y, x),
                6 => (-x, y, z),
                7 => (-z, x, y),
                8 => (-y, z, x),
                9 => (-x, z, y),
                10 => (-y, x, z),
                11 => (-z, y, x),
                12 => (x, -y, z),
                13 => (z, -x, y),
                14 => (y, -z, x),
                15 => (x, -z, y),
                16 => (y, -x, z),
                17 => (z, -y, x),
                18 => (x, y, -z),
                19 => (z, x, -y),
                20 => (y, z, -x),
                21 => (x, z, -y),
                22 => (y, x, -z),
                23 => (z, y, -x),
                24 => (-x, -y, z),
                25 => (-z, -x, y),
                26 => (-y, -z, x),
                27 => (-x, -z, y),
                28 => (-y, -x, z),
                29 => (-z, -y, x),
                30 => (x, -y, -z),
                31 => (z, -x, -y),
                32 => (y, -z, -x),
                33 => (x, -z, -y),
                34 => (y, -x, -z),
                35 => (z, -y, -x),
                36 => (-x, y, -z),
                37 => (-z, x, -y),
                38 => (-y, z, -x),
                39 => (-x, z, -y),
                40 => (-y, x, -z),
                41 => (-z, y, -x),
                42 => (-x, -y, -z),
                43 => (-z, -x, -y),
                44 => (-y, -z, -x),
                45 => (-x, -z, -y),
                46 => (-y, -x, -z),
                47 => (-z, -y, -x),
                _ => panic!("out of bounds: {}", self.index),
            })
            .collect::<Self::Item>();

        self.index += 1;
        Some(next)
    }
}
fn print_map(map: &HashMap<Coordinate, usize>) {
    let x_min = map.iter().map(|(pos, _)| pos.0).min().unwrap();
    let x_max = 1 + map.iter().map(|(pos, _)| pos.0).max().unwrap();
    let y_min = map.iter().map(|(pos, _)| pos.1).min().unwrap();
    let y_max = 1 + map.iter().map(|(pos, _)| pos.1).max().unwrap();
    let z_min = map.iter().map(|(pos, _)| pos.2).min().unwrap();
    let z_max = 1 + map.iter().map(|(pos, _)| pos.2).max().unwrap();
    let x_range = x_max - x_min;
    let y_range = y_max - y_min;
    let z_range = z_max - z_min;
    let dimensions: Coordinate = (x_range, y_range, z_range);

    let border = 2;
    let block_size: i32 = 1;
    let scale = 1;
    let virtual_size = (
        (block_size * (dimensions.0 + border * 2)),
        (block_size * (dimensions.1 + border * 2)),
    );
    let real_size = ((scale * virtual_size.0), (scale * virtual_size.1));

    for z in z_min..z_max {
        for y in y_min..y_max {
            for x in x_min..x_max {
                if let Some(value) = map.get(&(x, y, z)) {
                    print!("{:<1}", value);
                } else {
                    print!("{:<1}", "_");
                }
            }
            println!();
        }
    }
}

fn solve_parts(inputfile: String) -> (usize, usize) {
    let mut scanners = std::fs::read_to_string(inputfile)
        .expect("Scanner went wrong reading the file")
        .split("\n\n")
        .map(|textblob| {
            let scanner = Scanner::from_string(textblob);
            scanner
        })
        .collect::<Vec<Scanner>>();

    let mut global_map = scanners.pop().unwrap();

    let mut scanner_positions = vec![];

    while scanners.len() != 0 {
        let scanner = scanners.pop().unwrap();
        if let Some(scanner_position) = global_map.find_match(&scanner, 12) {
            println!("best match[{}]: {:?}", scanner.id, scanner_position);
            scanner_positions.push(scanner_position);
        } else {
            scanners.insert(0, scanner);
        }
    }

    println!("global beacons: {}", global_map.detections.len(),);
    println!("scanner positions: {:?}", scanner_positions);
    let max_manhattan_distance = scanner_positions
        .iter()
        .combinations(2)
        .fold(0, |max, pair| {
            let a = pair[0];
            let b = pair[1];
            println!("{:?}", (a, b));
            let manhattan_distance = (a.0 - b.0).abs() + (a.1 - b.1).abs() + (a.2 - b.2).abs();
            if manhattan_distance > max {
                manhattan_distance
            } else {
                max
            }
        });
    println!("max manhattan distance: {:?}", max_manhattan_distance);
    (global_map.detections.len(), max_manhattan_distance as usize)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Parts: {:?}", solve_parts(args[1].to_string()));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_small_example() {
        let scanner_0 = Scanner::from_string(
            "--- scanner 0 ---
0,2
4,1
3,3",
        );
        let mut scanner_1 = Scanner::from_string(
            "--- scanner 1 ---
-1,-1
-5,0
-2,1",
        );

        let best_translation = scanner_0.find_maximum_matching_points(&scanner_1);
        assert_eq!(best_translation, (5, 2, 0));

        assert_eq!(
            scanner_0.num_matching_points(&scanner_1, (5, 2, 0), (0, 0, 0)),
            3
        );
        scanner_1.translate((5, 2, 0));
        assert_eq!(scanner_0.is_matching(&scanner_1, 3), true);
    }
    #[test]
    fn test_rotate() {
        let rotated_scanners = [
            "--- scanner 0 ---
-1,-1,1
-2,-2,2
-3,-3,3
-2,-3,1
5,6,-4
8,0,7",
            "--- scanner 0 ---
1,-1,1
2,-2,2
3,-3,3
2,-1,3
-5,4,-6
-8,-7,0",
            "--- scanner 0 ---
-1,-1,-1
-2,-2,-2
-3,-3,-3
-1,-3,-2
4,6,5
-7,0,8",
            "--- scanner 0 ---
1,1,-1
2,2,-2
3,3,-3
1,3,-2
-4,-6,5
7,0,8",
            "--- scanner 0 ---
1,1,1
2,2,2
3,3,3
3,1,2
-6,-4,-5
0,7,-8",
        ]
        .iter()
        .map(|input| Scanner::from_string(input))
        .collect::<Vec<Scanner>>();

        let mut scanner = Scanner::from_string(
            "--- scanner 0 ---
-1,-1,1
-2,-2,2
-3,-3,3
-2,-3,1
5,6,-4
8,0,7",
        );

        for rotated_scanner in rotated_scanners {
            let found = ScannerOrientation::new(&scanner)
                .any(|detections| detections == rotated_scanner.detections);

            assert_eq!(found, true);
        }
        println!("rotated: {:?}", scanner);
    }
}
