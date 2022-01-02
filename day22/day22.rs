use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::env;

type Coordinate = (i32, i32, i32);

#[derive(Debug, Clone)]
struct Cuboid {
    status: bool,
    x_range: std::ops::Range<i32>,
    y_range: std::ops::Range<i32>,
    z_range: std::ops::Range<i32>,
}

impl Cuboid {
    pub fn new(
        status: bool,
        x_range: std::ops::Range<i32>,
        y_range: std::ops::Range<i32>,
        z_range: std::ops::Range<i32>,
    ) -> Self {
        Self {
            status,
            x_range,
            y_range,
            z_range,
        }
    }
    pub fn from_string(input: &str) -> Self {
        let re = Regex::new(r"(?P<status>on|off) x=(?P<x_start>[-0-9]+)..(?P<x_stop>[-0-9]+),y=(?P<y_start>[-0-9]+)..(?P<y_stop>[-0-9]+),z=(?P<z_start>[-0-9]+)..(?P<z_stop>[-0-9]+)").unwrap();
        let parsed = re.captures(input).unwrap();

        let status = match &parsed["status"] {
            "on" => true,
            "off" => false,
            _ => panic!(),
        };

        let x_range = parsed["x_start"].parse::<i32>().unwrap()
            ..(parsed["x_stop"].parse::<i32>().unwrap() + 1);
        let y_range = parsed["y_start"].parse::<i32>().unwrap()
            ..(parsed["y_stop"].parse::<i32>().unwrap() + 1);
        let z_range = parsed["z_start"].parse::<i32>().unwrap()
            ..(parsed["z_stop"].parse::<i32>().unwrap() + 1);

        Self {
            status,
            x_range,
            y_range,
            z_range,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.x_range.start >= self.x_range.end
            || self.y_range.start >= self.y_range.end
            || self.z_range.start >= self.z_range.end
    }

    pub fn is_out_of_range(&self) -> bool {
        let boundary = 50;
        self.x_range.start.abs() > boundary
            || self.x_range.end.abs() > boundary
            || self.y_range.start.abs() > boundary
            || self.y_range.end.abs() > boundary
            || self.z_range.start.abs() > boundary
            || self.z_range.end.abs() > boundary
    }
}

#[derive(Debug)]
struct Reactor {
    cuboids: Vec<Cuboid>,
    reboot_index: usize,
    cubes: HashMap<Coordinate, bool>,
}

impl Reactor {
    pub fn from_file(inputfile: &str) -> Self {
        Self::from_string(
            &std::fs::read_to_string(inputfile).expect("Something went wrong reading the file"),
        )
    }
    pub fn from_string(input: &str) -> Self {
        Self {
            cuboids: input
                .lines()
                .map(Cuboid::from_string)
                .collect::<Vec<Cuboid>>(),
            reboot_index: 0,
            cubes: HashMap::<Coordinate, bool>::new(),
        }
    }

    pub fn num_cuboids(&self) -> usize {
        self.cuboids.len()
    }

    pub fn split_cuboids(&mut self, limited_range: bool) -> Vec<Cuboid> {
        fn overlaps(a: &Cuboid, b: &Cuboid) -> bool {
            !(b.x_range.start > a.x_range.end
                || b.x_range.end < a.x_range.start
                || b.y_range.start > a.y_range.end
                || b.y_range.end < a.y_range.start
                || b.z_range.start > a.z_range.end
                || b.z_range.end < a.z_range.start)
        }

        fn subtract(a: &Cuboid, b: &Cuboid) -> Vec<Cuboid> {
            if !overlaps(a, b) {
                vec![a.clone()]
            } else {
                [
                    Cuboid::new(
                        true,
                        a.x_range.start..b.x_range.start,
                        a.y_range.clone(),
                        a.z_range.clone(),
                    ),
                    Cuboid::new(
                        true,
                        b.x_range.end..a.x_range.end,
                        a.y_range.clone(),
                        a.z_range.clone(),
                    ),
                    Cuboid::new(
                        true,
                        a.x_range.start.max(b.x_range.start)..a.x_range.end.min(b.x_range.end),
                        a.y_range.start..b.y_range.start,
                        a.z_range.clone(),
                    ),
                    Cuboid::new(
                        true,
                        a.x_range.start.max(b.x_range.start)..a.x_range.end.min(b.x_range.end),
                        b.y_range.end..a.y_range.end,
                        a.z_range.clone(),
                    ),
                    Cuboid::new(
                        true,
                        a.x_range.start.max(b.x_range.start)..a.x_range.end.min(b.x_range.end),
                        a.y_range.start.max(b.y_range.start)..a.y_range.end.min(b.y_range.end),
                        a.z_range.start..b.z_range.start,
                    ),
                    Cuboid::new(
                        true,
                        a.x_range.start.max(b.x_range.start)..a.x_range.end.min(b.x_range.end),
                        a.y_range.start.max(b.y_range.start)..a.y_range.end.min(b.y_range.end),
                        b.z_range.end..a.z_range.end,
                    ),
                ]
                .into_iter()
                .filter(|c| !c.is_empty())
                .collect()
            }
        }

        let mut active_cuboids: Vec<Cuboid> = Vec::new();

        for cuboid in &self.cuboids {
            if limited_range && cuboid.is_out_of_range() {
                println!("Skipping: {:?}", &cuboid);
                continue;
            }

            if cuboid.status {
                let mut cuts = vec![cuboid.clone()];

                for active_cuboid in active_cuboids.iter() {
                    let mut new_cuts = Vec::new();

                    for cut in cuts {
                        new_cuts.extend(subtract(&cut, &active_cuboid));
                    }

                    cuts = new_cuts;
                }

                active_cuboids.extend(cuts);
            } else {
                let mut cuts = Vec::new();

                for active_cuboid in active_cuboids.into_iter() {
                    cuts.extend(subtract(&active_cuboid, &cuboid));
                }

                active_cuboids = cuts;
            }
        }
        active_cuboids
    }

    pub fn num_cubes_in_cuboids(cuboids: &Vec<Cuboid>) -> usize {
        cuboids.iter().fold(0, |acc, cuboid| {
            let x_range = (cuboid.x_range.end - cuboid.x_range.start).abs() as usize;
            let y_range = (cuboid.y_range.end - cuboid.y_range.start).abs() as usize;
            let z_range = (cuboid.z_range.end - cuboid.z_range.start).abs() as usize;

            acc + (x_range * y_range * z_range)
        })
    }

    pub fn print(cuboids: &Vec<Cuboid>) {
        let x_min = cuboids
            .iter()
            .map(|cuboid| cuboid.x_range.start)
            .min()
            .unwrap();
        let x_max = cuboids
            .iter()
            .map(|cuboid| cuboid.x_range.end)
            .max()
            .unwrap();
        let y_min = cuboids
            .iter()
            .map(|cuboid| cuboid.y_range.start)
            .min()
            .unwrap();
        let y_max = cuboids
            .iter()
            .map(|cuboid| cuboid.y_range.end)
            .max()
            .unwrap();

        println!(
            "{}",
            (y_min..y_max + 1)
                .map(|y| {
                    (x_min..x_max + 1)
                        .map(|x| {
                            if let Some((status, index)) = cuboids
                                .iter()
                                .enumerate()
                                .filter_map(|(index, cuboid)| {
                                    if cuboid.x_range.contains(&x) && cuboid.y_range.contains(&y) {
                                        Some((cuboid.status, index))
                                    } else {
                                        None
                                    }
                                })
                                .last()
                            {
                                if status {
                                    format!("{}", index)
                                } else {
                                    ".".to_string()
                                }
                            } else {
                                " ".to_string()
                            }
                        })
                        .join("")
                })
                .join("\n")
        );
    }

    pub fn limited_boot(&mut self) {
        let boundary = 50;
        for cuboid in &self.cuboids {
            for z in cuboid.z_range.clone() {
                if z.abs() > boundary {
                    continue;
                }
                for y in cuboid.y_range.clone() {
                    if y.abs() > boundary {
                        continue;
                    }
                    for x in cuboid.x_range.clone() {
                        if x.abs() > boundary {
                            continue;
                        }
                        self.cubes
                            .entry((x, y, z))
                            .and_modify(|status| *status = cuboid.status)
                            .or_insert(cuboid.status);
                    }
                }
            }

            self.cubes.retain(|_, status| *status);
        }
    }

    pub fn step(&mut self) {
        let cuboid = &self.cuboids[self.reboot_index];

        for z in cuboid.z_range.clone() {
            for y in cuboid.y_range.clone() {
                for x in cuboid.x_range.clone() {
                    self.cubes
                        .entry((x, y, z))
                        .and_modify(|status| *status = cuboid.status)
                        .or_insert(cuboid.status);
                }
            }
        }

        println!("num_cubes: {}", self.cubes.len());
        self.cubes.retain(|_, status| *status);
        println!("num_cubes: {}", self.cubes.len());
        self.reboot_index += 1;
    }

    pub fn num_cubes(&self) -> usize {
        self.cubes
            .iter()
            .fold(0, |acc, (_, status)| acc + if *status { 1 } else { 0 })
    }
}

fn solve_part1(inputfile: String) -> usize {
    let mut reactor = Reactor::from_file(&inputfile);

    //reactor.limited_boot();
    //reactor.num_cubes()
    let active_cuboids = reactor.split_cuboids(true);
    Reactor::num_cubes_in_cuboids(&active_cuboids)
}

fn solve_part2(inputfile: String) -> usize {
    let mut reactor = Reactor::from_file(&inputfile);

    let active_cuboids = reactor.split_cuboids(false);
    Reactor::num_cubes_in_cuboids(&active_cuboids)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Part1: {}", solve_part1(args[1].to_string()));
    println!("Part2: {}", solve_part2(args[1].to_string()));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_split_2d_cuboids() {
        let mut reactor = Reactor::from_string(
            "on x=10..12,y=10..12,z=10..10
on x=11..13,y=11..13,z=10..10",
        );

        assert_eq!(reactor.num_cuboids(), 2);
        let active_cuboids = reactor.split_cuboids(false);
        Reactor::print(&active_cuboids);
        assert_eq!(active_cuboids.len(), 3);

        assert_eq!(Reactor::num_cubes_in_cuboids(&active_cuboids), 14);
    }

    #[test]
    fn test_split_cuboids() {
        let mut reactor = Reactor::from_string(
            "on x=10..12,y=10..12,z=10..12
on x=11..13,y=11..13,z=11..13
off x=9..11,y=9..11,z=9..11
on x=10..10,y=10..10,z=10..10",
        );

        let active_cuboids = reactor.split_cuboids(false);

        Reactor::print(&active_cuboids);
        assert_eq!(
            Reactor::num_cubes_in_cuboids(&active_cuboids),
            27 + 19 - 8 + 1
        );
    }

    #[test]
    fn test_small_example() {
        let mut reactor = Reactor::from_string(
            "on x=10..12,y=10..12,z=10..12
on x=11..13,y=11..13,z=11..13
off x=9..11,y=9..11,z=9..11
on x=10..10,y=10..10,z=10..10",
        );

        reactor.step();
        assert_eq!(reactor.num_cubes(), 27);

        reactor.step();
        assert_eq!(reactor.num_cubes(), 27 + 19);

        reactor.step();
        assert_eq!(reactor.num_cubes(), 27 + 19 - 8);

        reactor.step();
        assert_eq!(reactor.num_cubes(), 27 + 19 - 8 + 1);
    }

    #[test]
    fn test_small_example_boot() {
        let mut reactor = Reactor::from_string(
            "on x=10..12,y=10..12,z=10..12
on x=11..13,y=11..13,z=11..13
off x=9..11,y=9..11,z=9..11
on x=10..10,y=10..10,z=10..10",
        );

        reactor.limited_boot();
        assert_eq!(reactor.num_cubes(), 27 + 19 - 8 + 1);
    }

    /*
    #[test]
    fn test_larger_example() {
        let mut reactor = Reactor::from_string(
            "on x=-20..26,y=-36..17,z=-47..7
    on x=-20..33,y=-21..23,z=-26..28
    on x=-22..28,y=-29..23,z=-38..16
    on x=-46..7,y=-6..46,z=-50..-1
    on x=-49..1,y=-3..46,z=-24..28
    on x=2..47,y=-22..22,z=-23..27
    on x=-27..23,y=-28..26,z=-21..29
    on x=-39..5,y=-6..47,z=-3..44
    on x=-30..21,y=-8..43,z=-13..34
    on x=-22..26,y=-27..20,z=-29..19
    off x=-48..-32,y=26..41,z=-47..-37
    on x=-12..35,y=6..50,z=-50..-2
    off x=-48..-32,y=-32..-16,z=-15..-5
    on x=-18..26,y=-33..15,z=-7..46
    off x=-40..-22,y=-38..-28,z=23..41
    on x=-16..35,y=-41..10,z=-47..6
    off x=-32..-23,y=11..30,z=-14..3
    on x=-49..-5,y=-3..45,z=-29..18
    off x=18..30,y=-20..-8,z=-3..13
    on x=-41..9,y=-7..43,z=-33..15
    on x=-54112..-39298,y=-85059..-49293,z=-27449..7877
    on x=967..23432,y=45373..81175,z=27513..53682",
        );

        reactor.limited_boot();
        assert_eq!(reactor.num_cubes(), 590784);
    }
    */
}
