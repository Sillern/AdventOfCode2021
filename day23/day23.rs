use itertools::Itertools;
use std::collections::HashMap;
use std::fmt;

type Energy = usize;
type Coordinate = (i32, i32);
type Move = (MapType, Coordinate, Coordinate, Energy, IsHome);

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
        amphipod_type: &MapType,
        map: &HashMap<Coordinate, MapType>,
    ) -> Vec<(Coordinate, Energy, IsHome)> {
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

        let valid_hallways = vec![(1, 1), (2, 1), (4, 1), (6, 1), (8, 1), (10, 1), (11, 1)];
        let home = Self::get_home_cave(amphipod_type);

        if Amphipod::get_home_status(amphipod_type, amphipod_position, map) == IsHome::Yes {
            return vec![];
        }

        if valid_hallways.contains(&amphipod_position) {
            let mut home_candidates = home
                .iter()
                .filter(|coordinate| {
                    Amphipod::get_home_status(amphipod_type, coordinate, map) != IsHome::HasGuest
                })
                .map(|e| *e)
                .collect::<Vec<Coordinate>>();

            if home_candidates.len() == 2 {
                println!("home candidates: {:?}", home_candidates);
                home_candidates.sort_by(|(_, y_a), (_, y_b)| y_b.cmp(y_a));
                home_candidates.pop();
                println!("home candidates: {:?}", home_candidates);
            }
            home_candidates
                .iter()
                .filter_map(|coordinate| {
                    if let Some(distance) = get_distance(&amphipod_position, coordinate, map) {
                        Some((
                            *coordinate,
                            Amphipod::get_energy_cost(amphipod_type) * distance,
                            Amphipod::get_home_status(amphipod_type, coordinate, map),
                        ))
                    } else {
                        None
                    }
                })
                .collect::<Vec<(Coordinate, Energy, IsHome)>>()
        } else {
            valid_hallways
                .iter()
                .filter_map(|coordinate| {
                    let home_status = Amphipod::get_home_status(amphipod_type, coordinate, map);
                    if home_status == IsHome::HasGuest {
                        None
                    } else {
                        if let Some(distance) = get_distance(&amphipod_position, coordinate, map) {
                            Some((
                                *coordinate,
                                Amphipod::get_energy_cost(amphipod_type) * distance,
                                home_status,
                            ))
                        } else {
                            None
                        }
                    }
                })
                .collect::<Vec<(Coordinate, Energy, IsHome)>>()
        }
    }
}

type Map = HashMap<Coordinate, MapType>;

fn map_from_file(inputfile: &str) -> Map {
    map_from_string(
        &std::fs::read_to_string(inputfile).expect("Something went wrong reading the file"),
    )
}
fn map_from_string(input: &str) -> Map {
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

    map
}

fn make_move(map: &mut Map, next_move: &Move) {
    let (map_type, from, to, _, _) = next_move;
    map.entry(to.clone()).and_modify(|dest| *dest = *map_type);
    map.entry(from.clone())
        .and_modify(|source| *source = MapType::Path);
}

fn get_valid_moves(map: &Map) -> Vec<Move> {
    map.iter()
        .filter_map(|(coordinate, map_type)| match map_type {
            MapType::AmphipodAmber
            | MapType::AmphipodBronze
            | MapType::AmphipodCopper
            | MapType::AmphipodDesert => {
                let valid_locations = Amphipod::get_valid_locations(coordinate, &map_type, &map);
                if valid_locations.len() != 0 {
                    Some(
                        valid_locations
                            .iter()
                            .map(|(pos, energy, is_home)| {
                                (*map_type, *coordinate, *pos, *energy, *is_home)
                            })
                            .collect::<Vec<Move>>(),
                    )
                } else {
                    None
                }
            }
            _ => None,
        })
        .flatten()
        .collect::<Vec<Move>>()
}

fn organize(map: &mut Map) -> usize {
    let mut queue = vec![(0, map.clone(), vec![])];
    let mut visited = vec![];

    let organized_count = num_amphipods(&map);

    let mut lowest_cost = 10000000000;
    let mut steps = 0;
    while let Some((total_energy, map_state, path)) = queue.pop() {
        let path_clone = path.clone();
        let map_state_clone = map_state.clone();
        if visited.contains(&(path_clone, map_state_clone)) {
            continue;
        }

        let currently_organized = num_organized_amphipods(&map_state);

        if steps % 500 == 0 {
            println!(
            "Step[{}], queue: {}, cache: {}, num moves: {}, num organized: {}, energy: {}, lowest: {}",
            steps,
            queue.len(),
            visited.len(),
            path.len(),
            currently_organized,
            total_energy,
            lowest_cost,
        );
        }
        if currently_organized == organized_count {
            if total_energy < lowest_cost {
                print_map(&map_state);
                let total_energy = path
                    .iter()
                    .fold(0, |acc, (_, _, _, energy, _)| acc + energy);

                println!("total energy: {}", total_energy);

                lowest_cost = total_energy;
            }
            continue;
        }

        visited.push((path.clone(), map_state.clone()));

        for next_move in get_valid_moves(&map_state) {
            if let Some((_, was_coming_from, _, _, _)) = path.last() {
                if was_coming_from == &next_move.2 {
                    continue;
                }
            }

            let mut next_map_state = map_state.clone();
            make_move(&mut next_map_state, &next_move);

            let mut next_path = path.clone();
            next_path.push(next_move);

            let next_total_energy = next_path
                .iter()
                .fold(0, |acc, (_, _, _, energy, _)| acc + energy);

            queue.insert(0, (next_total_energy, next_map_state, next_path));
            //queue.push((next_total_energy, next_map_state, next_path));
        }
        steps += 1;
        if steps > 200000 {
            break;
        }

        /*
        queue.sort_by(|(energy_a, map_state_a, _), (energy_b, map_state_b, _)| {
            //let organized_a = num_organized_amphipods(&map_state_a);
            // let organized_b = num_organized_amphipods(&map_state_b);

            let order_a = energy_a;
            let order_b = energy_b;
            order_b.cmp(&order_a)
        });*/
    }

    0
}

fn num_amphipods(map: &Map) -> usize {
    map.iter()
        .filter(|(coordinate, map_type)| match map_type {
            MapType::AmphipodAmber
            | MapType::AmphipodBronze
            | MapType::AmphipodCopper
            | MapType::AmphipodDesert => true,
            _ => false,
        })
        .count()
}

fn num_organized_amphipods(map: &Map) -> usize {
    map.iter()
        .filter(|(coordinate, map_type)| match map_type {
            MapType::AmphipodAmber
            | MapType::AmphipodBronze
            | MapType::AmphipodCopper
            | MapType::AmphipodDesert => Amphipod::is_home(map_type, coordinate),
            _ => false,
        })
        .count()
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

fn solve_part1() -> usize {
    let mut burrows = map_from_string(
        "#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########",
    );

    organize(&mut burrows)
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
        let mut burrows = map_from_string(
            "#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########",
        );

        print_map(&burrows);
        assert_eq!(num_amphipods(&burrows), 8);
        assert_eq!(num_organized_amphipods(&burrows), 3);
    }

    #[test]
    fn test_initial_valid_locations() {
        let mut burrows = map_from_string(
            "#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########",
        );

        let num_starting_moves = get_valid_moves(&burrows).len();
        assert_eq!(num_starting_moves, 28);
    }

    #[test]
    fn test_no_moves() {
        {
            let mut burrows = map_from_string(
                "#############
#...A.......#
###A#.#C#D###
  #A#B#C#D#
  #########",
            );

            let valid_moves = get_valid_moves(&burrows);
            assert_eq!(valid_moves.len(), 0);
        }
        {
            let mut burrows = map_from_string(
                "#############
#.A.A.......#
###.#B#C#D###
  #B#B#C#D#
  #########",
            );

            let valid_moves = get_valid_moves(&burrows);
            assert_eq!(valid_moves.len(), 0);
        }
        {
            let mut burrows = map_from_string(
                "#############
#...........#
###A#B#C#D###
  #A#B#C#D#
  #########",
            );

            let valid_moves = get_valid_moves(&burrows);
            assert_eq!(valid_moves.len(), 0);
        }
    }

    #[test]
    fn test_detect_home_cave() {
        let mut burrows = map_from_string(
            "#############
#.....B.C...#
###B#C#.#D###
  #A#D#.#A#
  #########",
        );

        let could_go_home = get_valid_moves(&burrows)
            .iter()
            .filter_map(|amphipod_move| {
                if amphipod_move.4 == IsHome::Yes {
                    Some(*amphipod_move)
                } else {
                    None
                }
            })
            .collect::<Vec<Move>>();

        println!("could go home: {:?}", could_go_home);
        assert_eq!(could_go_home.len(), 1);
        let amphipod_move = could_go_home[0];
        assert_eq!(
            amphipod_move,
            (MapType::AmphipodCopper, (8, 1), (7, 3), 300, IsHome::Yes)
        );
    }

    #[test]
    fn test_prioritise_home_cave() {
        let mut burrows = map_from_string(
            "#############
#...B.D.....#
###B#.#C#D###
  #A#.#C#A#
  #########",
        );

        let could_go_home = get_valid_moves(&burrows)
            .iter()
            .filter_map(|amphipod_move| {
                if amphipod_move.4 == IsHome::Yes {
                    Some(*amphipod_move)
                } else {
                    None
                }
            })
            .collect::<Vec<Move>>();

        println!("could go home: {:?}", could_go_home);
        assert_eq!(could_go_home.len(), 1);
        let amphipod_move = could_go_home[0];
        assert_eq!(
            amphipod_move,
            (MapType::AmphipodBronze, (4, 1), (5, 3), 30, IsHome::Yes)
        );
    }
}
