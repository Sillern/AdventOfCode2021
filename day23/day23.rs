use itertools::Itertools;
use std::collections::HashMap;
use std::fmt;

type Energy = usize;
type Coordinate = (i32, i32);
type Move = (MapType, Coordinate, Coordinate, Energy);

#[derive(Debug, PartialEq, Clone, Copy)]
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

            while let Some((current_pos, path)) = queue.pop() {
                if current_pos == *end_pos {
                    return Some(path.len());
                }
                for adjacent in &adjacents {
                    let next_pos = (current_pos.0 + adjacent.0, current_pos.1 + adjacent.1);

                    if !visited.contains(&next_pos) {
                        visited.push(next_pos);

                        if let Some(MapType::Path) = map.get(&next_pos) {
                            let mut next_path = path.clone();
                            next_path.push(current_pos);
                            queue.push((next_pos, next_path));
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

    fn get_valid_moves(&self, moves: &Vec<Move>) -> Vec<(Move, IsHome)> {
        let mut map_copy = self.map.clone();
        for (map_type, from, to, _) in moves {
            map_copy
                .entry(to.clone())
                .and_modify(|dest| *dest = *map_type);
            map_copy
                .entry(from.clone())
                .and_modify(|source| *source = MapType::Path);
        }

        //print_map(&map_copy);

        map_copy
            .iter()
            .filter_map(|(coordinate, map_type)| match map_type {
                MapType::AmphipodAmber
                | MapType::AmphipodBronze
                | MapType::AmphipodCopper
                | MapType::AmphipodDesert => {
                    let valid_locations = Amphipod::get_valid_locations(coordinate, &map_copy);
                    if valid_locations.len() != 0 {
                        Some(
                            valid_locations
                                .iter()
                                .map(|(pos, energy, is_home)| {
                                    ((*map_type, *coordinate, *pos, *energy), *is_home)
                                })
                                .collect::<Vec<(Move, IsHome)>>(),
                        )
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .flatten()
            .sorted_by(|((_, _, _, energy_a), _), ((_, _, _, energy_b), _)| energy_a.cmp(energy_b))
            .collect::<Vec<(Move, IsHome)>>()
    }

    pub fn organize(&mut self) -> usize {
        let starting_path: Vec<Move> = vec![];
        let starting_locations = self.get_valid_moves(&starting_path);
        let mut queue = vec![(0, starting_locations, starting_path)];
        let mut visited = vec![];

        let mut steps = 0;
        while let Some((total_energy, valid_moves, path)) = queue.pop() {
            if visited.contains(&path) {
                continue;
            }
            visited.push(path.clone());

            println!(
                "Step[{}], energy: {}, num_moves: {}",
                steps,
                total_energy,
                path.len(),
            );
            if self.num_organized_amphipods(&path) == self.num_amphipods() {
                print_map_with_moves(&self.map, &path);
                break;
            }

            for (next_move, is_home) in valid_moves {
                if let Some((_, was_coming_from, _, _)) = path.last() {
                    if was_coming_from == &next_move.2 {
                        continue;
                    }
                }

                let mut next_path = path.clone();
                next_path.push(next_move);
                let next_moves = self.get_valid_moves(&next_path);
                let next_total_energy = next_path
                    .iter()
                    .fold(0, |acc, (_, _, _, energy)| acc + energy);

                queue.insert(0, ((next_total_energy, next_moves, next_path)));
            }
            steps += 1;
            if steps > 10000 {
                //break;
            }

            //queue.sort_by(|(energy_a, _, _), (energy_b, _, _)| energy_b.cmp(energy_a));
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

    pub fn num_organized_amphipods(&self, moves: &Vec<Move>) -> usize {
        let mut map_copy = self.map.clone();
        for (map_type, from, to, _) in moves {
            map_copy
                .entry(to.clone())
                .and_modify(|dest| *dest = *map_type);
            map_copy
                .entry(from.clone())
                .and_modify(|source| *source = MapType::Path);
        }

        map_copy
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

fn print_map_with_moves(map: &HashMap<Coordinate, MapType>, moves: &Vec<Move>) {
    let mut map_copy = map.clone();
    for (map_type, from, to, _) in moves {
        map_copy
            .entry(to.clone())
            .and_modify(|dest| *dest = *map_type);
        map_copy
            .entry(from.clone())
            .and_modify(|source| *source = MapType::Path);
    }

    print_map(&map_copy);
}
fn print_map(map: &HashMap<Coordinate, MapType>) {
    let x_min = map.iter().map(|(pos, _)| pos.0).min().unwrap();
    let x_max = map.iter().map(|(pos, _)| pos.0).max().unwrap();
    let y_min = map.iter().map(|(pos, _)| pos.1).min().unwrap();
    let y_max = map.iter().map(|(pos, _)| pos.1).max().unwrap();

    println!(
        "{}",
        (y_min..y_max + 1)
            .map(|y| {
                (x_min..x_max + 1)
                    .map(|x| {
                        let position = (x, y);
                        match map.get(&position) {
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
    );
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

    burrows.organize()
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
            .filter_map(|(amphipod_move, is_home)| {
                if *is_home == IsHome::Yes {
                    Some((*amphipod_move, *is_home))
                } else {
                    None
                }
            })
            .collect::<Vec<(Move, IsHome)>>();

        assert_eq!(could_go_home.len(), 1);
        let (amphipod_move, is_home) = could_go_home[0];
        assert_eq!(
            amphipod_move,
            (MapType::AmphipodCopper, (5, 2), (7, 2), 400)
        );
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
            .filter_map(|(amphipod_move, is_home)| {
                if *is_home == IsHome::Yes {
                    Some((*amphipod_move, *is_home))
                } else {
                    None
                }
            })
            .collect::<Vec<(Move, IsHome)>>();

        println!("could go home: {:?}", could_go_home);
        assert_eq!(could_go_home.len(), 1);
        let (amphipod_move, is_home) = could_go_home[0];
        assert_eq!(amphipod_move, (MapType::AmphipodBronze, (0, 0), (5, 3), 30));
        assert_eq!(is_home, IsHome::Yes);
    }
}
