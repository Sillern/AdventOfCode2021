use itertools::Itertools;
use regex::Regex;
use std::cmp::max;
use std::cmp::min;
use std::collections::HashMap;
use std::env;

type Coordinate = (i32, i32);
type Vector = (i32, i32);

#[derive(Debug)]
struct Target {
    x_range: std::ops::Range<i32>,
    y_range: std::ops::Range<i32>,
}

impl Target {
    pub fn new(input: &str) -> Self {
        let re = Regex::new(r"target area: x=(?P<x_start>[-0-9]+)..(?P<x_stop>[-0-9]+), y=(?P<y_start>[-0-9]+)..(?P<y_stop>[-0-9]+)").unwrap();

        let parsed = re.captures(input).unwrap();
        let x_range = parsed["x_start"].parse::<i32>().unwrap()
            ..(parsed["x_stop"].parse::<i32>().unwrap() + 1);
        let y_range = parsed["y_start"].parse::<i32>().unwrap()
            ..(parsed["y_stop"].parse::<i32>().unwrap() + 1);

        Self { x_range, y_range }
    }

    pub fn x_max(&self) -> i32 {
        max(self.x_range.start, self.x_range.end)
    }

    pub fn y_min(&self) -> i32 {
        min(self.y_range.start, self.y_range.end)
    }

    pub fn has_missed_target(&self, position: Coordinate) -> bool {
        let x_max = self.x_max();
        let y_min = self.y_min();

        position.1 < y_min || position.0 > x_max
    }

    pub fn hit_target(&self, position: Coordinate) -> bool {
        self.x_range.contains(&position.0) && self.y_range.contains(&position.1)
    }
}

#[derive(Clone, Copy, Debug)]
struct Trajectory {
    position: Coordinate,
    velocity: Vector,
    max_position: Coordinate,
    min_position: Coordinate,
}

impl Trajectory {
    pub fn new(position: Coordinate, velocity: Vector) -> Self {
        Self {
            position,
            velocity,
            max_position: (0, 0),
            min_position: (0, 0),
        }
    }

    pub fn step(&mut self) {
        self.position.0 += self.velocity.0;
        self.position.1 += self.velocity.1;
        if self.velocity.0 > 0 {
            self.velocity.0 -= 1;
        }
        self.velocity.1 -= 1;

        if self.position.1 > self.max_position.1 {
            self.max_position = self.position;
        }

        if self.position.1 < self.min_position.1 {
            self.min_position = self.position;
        }
    }
}

struct TrajectoryRange<'b> {
    start: Trajectory,
    current: Trajectory,
    target: &'b Target,
}

impl<'b> TrajectoryRange<'b> {
    fn new(trajectory: &Trajectory, target: &'b Target) -> Self {
        Self {
            start: trajectory.clone(),
            current: trajectory.clone(),
            target: target,
        }
    }
}

impl<'b> Iterator for TrajectoryRange<'b> {
    type Item = Trajectory;
    fn next(&mut self) -> Option<Self::Item> {
        self.current.step();

        if self.target.has_missed_target(self.current.position) {
            return None;
        } else {
            Some(self.current)
        }
    }
}

fn solve_part1() -> i32 {
    let target = Target::new("target area: x=25..67, y=-260..-200");

    let mut max_y = 0;
    for x_vel in 1..target.x_max() {
        for y_vel in target.y_min()..(-1 * target.y_min()) {
            let trajectory = Trajectory::new((0, 0), (x_vel, y_vel));

            if let Some(result) = TrajectoryRange::new(&trajectory, &target)
                .filter(|p| target.hit_target(p.position))
                .max_by(|a, b| a.max_position.1.cmp(&b.max_position.1))
            {
                if result.max_position.1 > max_y {
                    max_y = result.max_position.1;
                }
            }
        }
    }
    max_y
}

fn solve_part2() -> i32 {
    let target = Target::new("target area: x=25..67, y=-260..-200");

    let mut num_valid = 0;
    for x_vel in 1..target.x_max() {
        for y_vel in target.y_min()..(-1 * target.y_min()) {
            let trajectory = Trajectory::new((0, 0), (x_vel, y_vel));

            num_valid += if TrajectoryRange::new(&trajectory, &target)
                .any(|p| target.hit_target(p.position))
            {
                1
            } else {
                0
            };
        }
    }
    num_valid
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

fn draw_image(
    startpoint: &Coordinate,
    target: &Target,
    trajectories: &Vec<Trajectory>,
    y_limits: (i32, i32),
    frame: u32,
) {
    let x_min = 0;
    let x_max = target.x_max();
    let y_min = y_limits.0;
    let y_max = y_limits.1;
    let x_range = (1 + x_max - x_min);
    let y_range = (1 + y_max - y_min);
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

    draw_pixel(&mut pixels, *startpoint, block_size, 23);

    for y in target.y_range.clone() {
        for x in target.x_range.clone() {
            draw_pixel(&mut pixels, (x, y), block_size, 0);
        }
    }

    for (index, trajectory) in trajectories.iter().enumerate() {
        for current in TrajectoryRange::new(trajectory, target) {
            let color_index = index;
            let (x, y) = current.position;
            draw_pixel(&mut pixels, current.position, block_size, color_index);
        }
    }

    let mut img = ImageBuffer::from_fn(real_size.0 as u32, real_size.1 as u32, |_x, _y| {
        image::Rgb([255, 255, 255])
    });

    for ((x_, y_), color) in pixels {
        let pixel = image::Rgb([color.0, color.1, color.2]);
        let (x, y) = (
            x_ - ((x_min - border) * block_size),
            (y_range * block_size - (y_ - y_min * block_size)) + border * block_size,
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

    img.save(format!("frames/day17.frame{:05}.png", frame));
}

fn main() {
    println!("Part1: {}", solve_part1());
    println!("Part2: {}", solve_part2());

    let mut frame = 0;
    let target = Target::new("target area: x=20..30, y=-10..-5");
    //let target = Target::new("target area: x=25..67, y=-260..-200");
    let startpoint: Coordinate = (0, 0);
    let mut y_limits = (0, 0);

    let valid_trajectories = (1..target.x_max())
        .map(|x_vel| {
            (target.y_min()..(-1 * target.y_min()))
                .filter_map(|y_vel| {
                    let trajectory = Trajectory::new(startpoint, (x_vel, y_vel));

                    if let Some(result) = TrajectoryRange::new(&trajectory, &target)
                        .filter(|p| target.hit_target(p.position))
                        .max_by(|a, b| a.max_position.1.cmp(&b.max_position.1))
                    {
                        if result.max_position.1 > y_limits.1 {
                            y_limits.1 = result.max_position.1;
                        }
                        if result.min_position.1 < y_limits.0 {
                            y_limits.0 = result.min_position.1;
                        }
                        Some(trajectory)
                    } else {
                        None
                    }
                })
                .collect::<Vec<Trajectory>>()
        })
        .flatten()
        .collect::<Vec<Trajectory>>();

    draw_image(&startpoint, &target, &valid_trajectories, y_limits, frame);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_small_example() {
        let target = Target::new("target area: x=20..30, y=-10..-5");
        assert_eq!(target.hit_target((25, -7)), true);
        assert_eq!(target.hit_target((19, -7)), false);
        assert_eq!(target.hit_target((25, -2)), false);
        assert_eq!(target.has_missed_target((0, 0)), false);

        let mut max_y = 0;
        for x_vel in 1..target.x_max() {
            for y_vel in target.y_min()..(x_vel * 2) {
                let trajectory = Trajectory::new((0, 0), (x_vel, y_vel));

                if let Some(result) = TrajectoryRange::new(&trajectory, &target)
                    .filter(|p| target.hit_target(p.position))
                    .max_by(|a, b| a.max_position.1.cmp(&b.max_position.1))
                {
                    if result.max_position.1 > max_y {
                        max_y = result.max_position.1;
                    }
                }
            }
        }
        assert_eq!(max_y, 45);
    }

    #[test]
    fn test_trajectory() {
        let mut trajectory = Trajectory::new((0, 0), (6, 9));
        trajectory.step();
        assert_eq!(trajectory.position.0, 6);
        assert_eq!(trajectory.position.1, 9);
        trajectory.step();
        assert_eq!(trajectory.position.0, 11);
        assert_eq!(trajectory.position.1, 17);
    }

    #[test]
    fn test_hit_target() {
        let trajectory = Trajectory::new((0, 0), (6, 3));
        let target = Target::new("target area: x=20..30, y=-10..-5");

        let result =
            TrajectoryRange::new(&trajectory, &target).any(|p| target.hit_target(p.position));
        assert_eq!(result, true);
    }

    #[test]
    fn test_hit_problematic_target() {
        let trajectory = Trajectory::new((0, 0), (27, -5));
        let target = Target::new("target area: x=20..30, y=-10..-5");

        let result =
            TrajectoryRange::new(&trajectory, &target).any(|p| target.hit_target(p.position));
        assert_eq!(result, true);
    }

    #[test]
    fn test_hit_all_targets() {
        let initial_velocities = vec![
            (23, -10),
            (25, -9),
            (27, -5),
            (29, -6),
            (22, -6),
            (21, -7),
            (9, 0),
            (27, -7),
            (24, -5),
            (25, -7),
            (26, -6),
            (25, -5),
            (6, 8),
            (11, -2),
            (20, -5),
            (29, -10),
            (6, 3),
            (28, -7),
            (8, 0),
            (30, -6),
            (29, -8),
            (20, -10),
            (6, 7),
            (6, 4),
            (6, 1),
            (14, -4),
            (21, -6),
            (26, -10),
            (7, -1),
            (7, 7),
            (8, -1),
            (21, -9),
            (6, 2),
            (20, -7),
            (30, -10),
            (14, -3),
            (20, -8),
            (13, -2),
            (7, 3),
            (28, -8),
            (29, -9),
            (15, -3),
            (22, -5),
            (26, -8),
            (25, -8),
            (25, -6),
            (15, -4),
            (9, -2),
            (15, -2),
            (12, -2),
            (28, -9),
            (12, -3),
            (24, -6),
            (23, -7),
            (25, -10),
            (7, 8),
            (11, -3),
            (26, -7),
            (7, 1),
            (23, -9),
            (6, 0),
            (22, -10),
            (27, -6),
            (8, 1),
            (22, -8),
            (13, -4),
            (7, 6),
            (28, -6),
            (11, -4),
            (12, -4),
            (26, -9),
            (7, 4),
            (24, -10),
            (23, -8),
            (30, -8),
            (7, 0),
            (9, -1),
            (10, -1),
            (26, -5),
            (22, -9),
            (6, 5),
            (7, 5),
            (23, -6),
            (28, -10),
            (10, -2),
            (11, -1),
            (20, -9),
            (14, -2),
            (29, -7),
            (13, -3),
            (23, -5),
            (24, -8),
            (27, -9),
            (30, -7),
            (28, -5),
            (21, -10),
            (7, 9),
            (6, 6),
            (21, -5),
            (27, -10),
            (7, 2),
            (30, -9),
            (21, -8),
            (22, -7),
            (24, -9),
            (20, -6),
            (6, 9),
            (29, -5),
            (8, -2),
            (27, -8),
            (30, -5),
            (24, -7),
        ];
        let target = Target::new("target area: x=20..30, y=-10..-5");

        for initial_velocity in initial_velocities {
            let trajectory = Trajectory::new((0, 0), initial_velocity);
            println!("Testing: {:?}", initial_velocity);
            assert_eq!(
                TrajectoryRange::new(&trajectory, &target).any(|p| target.hit_target(p.position)),
                true
            );
        }
    }
}
