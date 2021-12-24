use itertools::Itertools;
use std::collections::HashMap;
use std::fmt;

type Energy = usize;
type Coordinate = (i32, i32);
type Move = (Coordinate, Coordinate);

#[derive(Debug, PartialEq)]
enum MapType {
    Wall,
    Path,
    AmphipodAmber,
    AmphipodBronze,
    AmphipodCopper,
    AmphipodDesert,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum IsHome {
    No,
    HasGuest,
    Yes,
}

#[derive(Debug)]
struct Amphipod {
    amphipod_type: MapType,
    energy: Energy,
    position: Coordinate,
    home_cave: Vec<Coordinate>,
}

impl Amphipod {
    pub fn get_energy_cost(amphipod_type: &MapType) -> Energy {
        match amphipod_type {
            MapType::AmphipodAmber => 1,
            MapType::AmphipodBronze => 10,
            MapType::AmphipodCopper => 100,
            MapType::AmphipodDesert => 1000,
            _ => panic!(),
        }
    }

    pub fn get_home_cave(amphipod_type: &MapType) -> Vec<Coordinate> {
        match amphipod_type {
            MapType::AmphipodAmber => vec![(3, 2), (3, 3)],
            MapType::AmphipodBronze => vec![(5, 2), (5, 3)],
            MapType::AmphipodCopper => vec![(7, 2), (7, 3)],
            MapType::AmphipodDesert => vec![(9, 2), (9, 3)],
            _ => panic!(),
        }
    }

    pub fn is_home(amphipod_type: &MapType, position: &Coordinate) -> bool {
        Self::get_home_cave(amphipod_type).contains(position)
    }

    pub fn is_equal(&self, map_type: &MapType) -> bool {
        match map_type {
            MapType::AmphipodAmber => self.amphipod_type == MapType::AmphipodAmber,
            MapType::AmphipodBronze => self.amphipod_type == MapType::AmphipodBronze,
            MapType::AmphipodCopper => self.amphipod_type == MapType::AmphipodCopper,
            MapType::AmphipodDesert => self.amphipod_type == MapType::AmphipodDesert,
            _ => false,
        }
    }

    pub fn get_home_status(
        amphipod_type: &MapType,
        target_pos: &Coordinate,
        map: &HashMap<Coordinate, MapType>,
    ) -> IsHome {
        let home_cave = Self::get_home_cave(amphipod_type);

        if home_cave.contains(&target_pos) {
            if home_cave.iter().all(|cave_pos| match map.get(&cave_pos) {
                Some(map_type) => *map_type == MapType::Path || amphipod_type == map_type,
                None => false,
            }) {
                IsHome::Yes
            } else {
                IsHome::HasGuest
            }
        } else {
            IsHome::No
        }
    }

    pub fn get_valid_locations(
        amphipod_position: &Coordinate,
        map: &HashMap<Coordinate, MapType>,
    ) -> Vec<(Coordinate, Energy, IsHome)> {
        fn is_valid_location(pos: &Coordinate, map: &HashMap<Coordinate, MapType>) -> bool {
            let num_walls = [(-1, 0), (1, 0), (0, -1), (0, 1)]
                .iter()
                .fold(0, |acc, adjacent| {
                    acc + if let Some(MapType::Wall) =
                        map.get(&(pos.0 + adjacent.0, pos.1 + adjacent.1))
                    {
                        1
                    } else {
                        0
                    }
                });
            num_walls > 1
        }

        fn get_distance(
            start_pos: &Coordinate,
            end_pos: &Coordinate,
            map: &HashMap<Coordinate, MapType>,
        ) -> Option<usize> {
            let adjacents = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];

            let mut queue = vec![(*start_pos, vec![])];
            let mut visited = vec![];

            while let Some((current_pos, mut path)) = queue.pop() {
                for adjacent in &adjacents {
                    let next_pos = (current_pos.0 + adjacent.0, current_pos.1 + adjacent.1);

                    if !visited.contains(&next_pos) {
                        visited.push(next_pos);
                        if next_pos == *end_pos {
                            return Some(path.len());
                        }

                        if let Some(MapType::Path) = map.get(&next_pos) {
                            path.push(current_pos);
                            queue.push((next_pos, path.clone()));
                        }
                    }
                }
            }
            None
        }

        let amphipod_type = map.get(&amphipod_position).unwrap();
        if Amphipod::get_home_status(amphipod_type, &amphipod_position, map) == IsHome::Yes {
            return vec![];
        }

        map.iter()
            .filter_map(|(coordinate, map_type)| {
                if *map_type == MapType::Path && is_valid_location(coordinate, map) {
                    if let Some(distance) = get_distance(&amphipod_position, coordinate, map) {
                        Some((
                            *coordinate,
                            Amphipod::get_energy_cost(amphipod_type) * distance,
                            Amphipod::get_home_status(amphipod_type, coordinate, map),
                        ))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect::<Vec<(Coordinate, Energy, IsHome)>>()
    }
}

#[derive(Debug)]
struct Burrows {
    map: HashMap<Coordinate, MapType>,
}

impl Burrows {
    pub fn from_file(inputfile: &str) -> Self {
        Self::from_string(
            &std::fs::read_to_string(inputfile).expect("Something went wrong reading the file"),
        )
    }
    pub fn from_string(input: &str) -> Self {
        let mut map: HashMap<Coordinate, MapType> = HashMap::new();

        input.lines().enumerate().for_each(|(y, row)| {
            row.chars().enumerate().for_each(|(x, c)| {
                let position: Coordinate = (x as i32, y as i32);
                match c {
                    '#' => {
                        map.insert(position, MapType::Wall);
                    }
                    '.' => {
                        map.insert(position, MapType::Path);
                    }
                    ' ' => {}
                    _ => {
                        let map_type = match c {
                            'A' => MapType::AmphipodAmber,
                            'B' => MapType::AmphipodBronze,
                            'C' => MapType::AmphipodCopper,
                            'D' => MapType::AmphipodDesert,
                            _ => panic!(),
                        };

                        map.insert(position, map_type);
                    }
                };
            })
        });

        Self { map }
    }

    fn get_valid_moves(&self, moves: &Vec<Move>) -> Vec<(Move, Energy, IsHome)> {
        self.map
            .iter()
            .filter_map(|(coordinate, map_type)| match map_type {
                MapType::AmphipodAmber
                | MapType::AmphipodBronze
                | MapType::AmphipodCopper
                | MapType::AmphipodDesert => {
                    let valid_locations = Amphipod::get_valid_locations(coordinate, &self.map);
                    if valid_locations.len() != 0 {
                        Some(
                            valid_locations
                                .iter()
                                .map(|(pos, energy, is_home)| {
                                    ((*coordinate, *pos), *energy, *is_home)
                                })
                                .collect::<Vec<(Move, Energy, IsHome)>>(),
                        )
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .flatten()
            .sorted_by(|(_, energy_a, _), (_, energy_b, _)| energy_a.cmp(energy_b))
            .collect::<Vec<(Move, Energy, IsHome)>>()
    }

    pub fn organize(&mut self) -> usize {
        /*
        fn least_energy(
            start_pos: &Coordinate,
            end_pos: &Coordinate,
            map: &HashMap<Coordinate, MapType>,
        ) -> Option<usize> {
            let adjacents = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];

            let mut queue = vec![(*start_pos, vec![])];
            let mut visited = vec![];

            while let Some((current_pos, mut path)) = queue.pop() {
                for adjacent in &adjacents {
                    let next_pos = (current_pos.0 + adjacent.0, current_pos.1 + adjacent.1);

                    if !visited.contains(&next_pos) {
                        visited.push(next_pos);
                        if next_pos == *end_pos {
                            return Some(path.len());
                        }

                        if let Some(MapType::Path) = map.get(&next_pos) {
                            path.push(current_pos);
                            queue.push((next_pos, path.clone()));
                        }
                    }
                }
            }
            None
        }
        */

        let starting_path: Vec<(Coordinate, Coordinate)> = vec![];
        let starting_locations = self.get_valid_moves(&starting_path);
        let mut queue = vec![(starting_locations, starting_path)];

        println!("starting positions: {:?}", queue.len());
        for (entry, path) in &queue {
            for starting_position in entry {
                println!("    {:?}", starting_position);
            }
        }

        while let Some((valid_moves, path)) = queue.pop() {
            for (next_move, energy, is_home) in valid_moves {
                let mut next_path = path.clone();
                next_path.push(next_move);
                let next_moves = self.get_valid_moves(&path);

                for starting_position in next_moves {
                    println!("    {:?}", starting_position);
                }
                break;
            }
        }

        0
    }

    pub fn num_amphipods(&self) -> usize {
        self.map
            .iter()
            .filter(|(coordinate, map_type)| match map_type {
                MapType::AmphipodAmber
                | MapType::AmphipodBronze
                | MapType::AmphipodCopper
                | MapType::AmphipodDesert => true,
                _ => false,
            })
            .count()
    }

    pub fn num_organized_amphipods(&self) -> usize {
        self.map
            .iter()
            .filter(|(coordinate, map_type)| match map_type {
                MapType::AmphipodAmber
                | MapType::AmphipodBronze
                | MapType::AmphipodCopper
                | MapType::AmphipodDesert => Amphipod::is_home(map_type, coordinate),
                _ => false,
            })
            .count()
    }
}

impl fmt::Display for Burrows {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let x_min = self.map.iter().map(|(pos, _)| pos.0).min().unwrap();
        let x_max = self.map.iter().map(|(pos, _)| pos.0).max().unwrap();
        let y_min = self.map.iter().map(|(pos, _)| pos.1).min().unwrap();
        let y_max = self.map.iter().map(|(pos, _)| pos.1).max().unwrap();

        write!(
            f,
            "{}",
            (y_min..y_max + 1)
                .map(|y| {
                    (x_min..x_max + 1)
                        .map(|x| {
                            let position = (x, y);
                            match self.map.get(&position) {
                                Some(MapType::Wall) => "#",
                                Some(MapType::Path) => ".",
                                Some(MapType::AmphipodAmber) => "A",
                                Some(MapType::AmphipodBronze) => "B",
                                Some(MapType::AmphipodCopper) => "C",
                                Some(MapType::AmphipodDesert) => "D",
                                None => " ",
                            }
                        })
                        .join("")
                })
                .join("\n")
        )
    }
}
fn solve_part1() -> usize {
    let mut burrows = Burrows::from_string(
        "#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########",
    );
    println!("{}", burrows);
    let least_energy = burrows.organize();
    println!("{}", burrows);
    least_energy
}

fn solve_part2() -> usize {
    0
}

fn main() {
    println!("Part1: {}", solve_part1());
    println!("Part2: {}", solve_part2());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_initial_burrows() {
        let mut burrows = Burrows::from_string(
            "#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########",
        );

        println!("burrows: {}", burrows);
        assert_eq!(burrows.num_amphipods(), 8);
        assert_eq!(burrows.num_organized_amphipods(), 3);
    }

    #[test]
    fn test_initial_valid_locations() {
        let mut burrows = Burrows::from_string(
            "#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########",
        );

        let num_starting_moves = burrows.get_valid_moves(&vec![]).len();
        assert_eq!(num_starting_moves, 28);
    }

    #[test]
    fn test_detect_home_cave() {
        let mut burrows = Burrows::from_string(
            "#############
#...B.......#
###B#C#.#D###
  #A#D#C#A#
  #########",
        );

        let could_go_home = burrows
            .get_valid_moves(&vec![])
            .iter()
            .filter_map(|(amphipod_move, energy, is_home)| {
                if *is_home == IsHome::Yes {
                    Some((*amphipod_move, *energy, *is_home))
                } else {
                    None
                }
            })
            .collect::<Vec<(Move, Energy, IsHome)>>();

        assert_eq!(could_go_home.len(), 1);
        let (amphipod_move, energy, is_home) = could_go_home[0];
        assert_eq!(amphipod_move, ((5, 2), (7, 2)));
        assert_eq!(energy, 400);
        assert_eq!(is_home, IsHome::Yes);
    }

    #[test]
    fn test_prioritise_home_cave() {
        let mut burrows = Burrows::from_string(
            "#############
#...B.D.....#
###B#.#C#D###
  #A#.#C#A#
  #########",
        );

        let could_go_home = burrows
            .get_valid_moves(&vec![])
            .iter()
            .filter_map(|(amphipod_move, energy, is_home)| {
                if *is_home == IsHome::Yes {
                    Some((*amphipod_move, *energy, *is_home))
                } else {
                    None
                }
            })
            .collect::<Vec<(Move, Energy, IsHome)>>();

        println!("could go home: {:?}", could_go_home);
        assert_eq!(could_go_home.len(), 1);
        let (amphipod_move, energy, is_home) = could_go_home[0];
        assert_eq!(amphipod_move, ((0, 0), (5, 3)));
        assert_eq!(energy, 30);
        assert_eq!(is_home, IsHome::Yes);
    }
}
