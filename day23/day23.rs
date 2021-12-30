use itertools::Itertools;
use std::collections::HashMap;


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

fn get_energy_cost(amphipod_type: &MapType) -> Energy {
    match amphipod_type {
        MapType::AmphipodAmber => 1,
        MapType::AmphipodBronze => 10,
        MapType::AmphipodCopper => 100,
        MapType::AmphipodDesert => 1000,
        _ => panic!(),
    }
}

fn get_path(
    start_pos: &Coordinate,
    end_pos: &Coordinate,
    map: &HashMap<Coordinate, MapType>,
) -> Option<Vec<Coordinate>> {
    let adjacents = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];

    let mut queue = vec![(*start_pos, vec![])];

    while let Some((current_pos, mut path)) = queue.pop() {
        if current_pos == *end_pos {
            path.remove(0);
            path.push(current_pos);
            return Some(path);
        }
        for adjacent in &adjacents {
            let next_pos = (current_pos.0 + adjacent.0, current_pos.1 + adjacent.1);

            if let Some(was_coming_from) = path.last() {
                if was_coming_from == &next_pos {
                    continue;
                }
            }

            if let Some(MapType::Path) = map.get(&next_pos) {
                let mut next_path = path.clone();
                next_path.push(current_pos);
                queue.push((next_pos, next_path));
            }
        }
    }
    None
}

fn get_valid_locations(
    amphipod_position: &Coordinate,
    amphipod_type: &MapType,
    map: &HashMap<Coordinate, MapType>,
) -> Vec<(Coordinate, Energy)> {
    let valid_hallways = vec![(1, 1), (2, 1), (4, 1), (6, 1), (8, 1), (10, 1), (11, 1)];
    let home_cave_x_position = match amphipod_type {
        MapType::AmphipodAmber => 3,
        MapType::AmphipodBronze => 5,
        MapType::AmphipodCopper => 7,
        MapType::AmphipodDesert => 9,
        _ => panic!(),
    };

    let hallway_y_position = 1;

    let y_min = map.iter().map(|(pos, _)| pos.1).min().unwrap();
    let y_max = map.iter().map(|(pos, _)| pos.1).max().unwrap();

    let could_go_home = (y_min..y_max)
        .filter(|y| *y != hallway_y_position)
        .all(|y| {
            if let Some(map_type) = map.get(&(home_cave_x_position, y)) {
                *map_type == *amphipod_type
                    || *map_type == MapType::Path
                    || *map_type == MapType::Wall
            } else {
                false
            }
        });

    if could_go_home && amphipod_position.0 == home_cave_x_position {
        // is already in a home cave
        return vec![];
    }

    if could_go_home {
        if let Some(furthest_in_y_position) = (y_min..y_max)
            .rev()
            .filter(|y| *y != hallway_y_position)
            .find(|y| {
                if let Some(map_type) = map.get(&(home_cave_x_position, *y)) {
                    *map_type == MapType::Path
                } else {
                    false
                }
            })
        {
            let coordinate = (home_cave_x_position, furthest_in_y_position);
            if let Some(valid_path) = get_path(amphipod_position, &coordinate, map) {
                return vec![(
                    coordinate,
                    get_energy_cost(amphipod_type) * valid_path.len(),
                )];
            }
        }
    }

    if valid_hallways.contains(amphipod_position) {
        // is in the hallway can only go to an empty home cave, or non guest home cave
        return vec![];
    } else {
        // is in a cave and can go to a home cave or hallway
        valid_hallways
            .iter()
            .filter_map(|coordinate| {
                get_path(amphipod_position, coordinate, map).map(|valid_path| (
                        *coordinate,
                        get_energy_cost(amphipod_type) * valid_path.len(),
                    ))
            })
            .collect::<Vec<(Coordinate, Energy)>>()
    }
}

fn num_amphipods(map: &Map) -> usize {
    map.iter()
        .filter(|(_coordinate, map_type)| match map_type {
            MapType::AmphipodAmber
            | MapType::AmphipodBronze
            | MapType::AmphipodCopper
            | MapType::AmphipodDesert => true,
            _ => false,
        })
        .count()
}

fn num_organized_amphipods(map: &Map) -> usize {
    let mut count = 0;
    for amphipod_type in [
        MapType::AmphipodAmber,
        MapType::AmphipodBronze,
        MapType::AmphipodCopper,
        MapType::AmphipodDesert,
    ] {
        let home_cave_x_position = match amphipod_type {
            MapType::AmphipodAmber => 3,
            MapType::AmphipodBronze => 5,
            MapType::AmphipodCopper => 7,
            MapType::AmphipodDesert => 9,
            _ => panic!(),
        };

        let hallway_y_position = 1;

        let y_min = map.iter().map(|(pos, _)| pos.1).min().unwrap();
        let y_max = map.iter().map(|(pos, _)| pos.1).max().unwrap();

        // is in the hallway can only go to an empty home cave, or non guest home cave
        let could_go_home = (y_min..y_max)
            .filter(|y| {
                if *y == hallway_y_position {
                    false
                } else if let Some(map_type) = map.get(&(home_cave_x_position, *y)) {
                    *map_type == amphipod_type
                } else {
                    false
                }
            })
            .count();

        count += could_go_home;
    }
    count
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

fn make_move(map: &mut Map, next_move: &Move) {
    let (map_type, from, to, _) = next_move;
    map.entry(*to).and_modify(|dest| *dest = *map_type);
    map.entry(*from)
        .and_modify(|source| *source = MapType::Path);
}

fn get_valid_moves(map: &Map) -> Vec<Move> {
    map.iter()
        .filter_map(|(coordinate, map_type)| match map_type {
            MapType::AmphipodAmber
            | MapType::AmphipodBronze
            | MapType::AmphipodCopper
            | MapType::AmphipodDesert => {
                let valid_locations = get_valid_locations(coordinate, map_type, map);
                if !valid_locations.is_empty() {
                    Some(
                        valid_locations
                            .iter()
                            .map(|(pos, energy)| (*map_type, *coordinate, *pos, *energy))
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

    let organized_count = num_amphipods(map);

    fn get_home_cave(amphipod_type: &MapType) -> Vec<Coordinate> {
        match amphipod_type {
            MapType::AmphipodAmber => vec![(3, 2), (3, 3), (3, 4), (3, 5)],
            MapType::AmphipodBronze => vec![(5, 2), (5, 3), (5, 4), (5, 5)],
            MapType::AmphipodCopper => vec![(7, 2), (7, 3), (7, 4), (7, 5)],
            MapType::AmphipodDesert => vec![(9, 2), (9, 3), (9, 4), (9, 5)],
            _ => panic!(),
        }
    }

    let mut lowest_cost = 10000000000;
    let mut steps = 0;
    while let Some((total_energy, map_state, path)) = queue.pop() {
        if total_energy >= lowest_cost {
            continue;
        }

        let currently_organized = num_organized_amphipods(&map_state);

        if steps % 50000 == 0 {
            println!(
                "Step[{}], queue: {}, num moves: {}, num organized: {}, energy: {}, lowest: {}",
                steps,
                queue.len(),
                path.len(),
                currently_organized,
                total_energy,
                lowest_cost,
            );
        }
        if currently_organized == organized_count {
            if total_energy < lowest_cost {
                println!(
                    "Step[{}], queue: {}, num moves: {}, num organized: {}, energy: {}, lowest: {}",
                    steps,
                    queue.len(),
                    path.len(),
                    currently_organized,
                    total_energy,
                    lowest_cost,
                );
                print_map(&map_state);
                println!();
                println!("total energy: {}", total_energy);

                lowest_cost = total_energy;
            }
            continue;
        }

        for next_move in get_valid_moves(&map_state) {
            let mut next_map_state = map_state.clone();
            make_move(&mut next_map_state, &next_move);

            let mut next_path = path.clone();
            next_path.push(next_move);

            let next_total_energy = next_path
                .iter()
                .fold(0, |acc, (_, _, _, energy)| acc + energy);
            if next_total_energy < lowest_cost {
                //queue.insert(0, (next_total_energy, next_map_state, next_path));
                queue.push((next_total_energy, next_map_state, next_path));
            }
        }
        steps += 1;
    }

    lowest_cost
}

fn solve_part1() -> usize {
    let _burrows_example = map_from_string(
        "#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########",
    );

    let mut burrows = map_from_string(
        "#############
#...........#
###D#D#B#A###
  #C#A#B#C#
  #########",
    );

    organize(&mut burrows)
}

fn solve_part2() -> usize {
    let mut burrows = map_from_string(
        "#############
#...........#
###D#D#B#A###
  #D#C#B#A#
  #D#B#A#C#
  #C#A#B#C#
  #########",
    );
    organize(&mut burrows)
}

fn main() {
    println!("Part2: {}", solve_part2());
    println!("Part1: {}", solve_part1());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_path() {
        let burrows = map_from_string(
            "#############
#...B........#
###B#C#.#D###
  #A#D#C#A#
  #########",
        );

        let path = get_path(&(5, 2), &(7, 2), &burrows).unwrap();
        println!("path: {:?}", path);
        assert_eq!(path.len(), 4);
    }

    #[test]
    fn test_initial_burrows() {
        let burrows = map_from_string(
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
    fn test_initial_burrows2() {
        let burrows = map_from_string(
            "#############
#...........#
###B#C#B#D###
  #D#C#B#A#
  #D#B#A#C#
  #A#D#C#A#
  #########",
        );

        print_map(&burrows);
        assert_eq!(num_amphipods(&burrows), 16);
        assert_eq!(num_organized_amphipods(&burrows), 4);
    }

    #[test]
    fn test_organized_count() {
        let burrows = map_from_string(
            "#############
#.D.C...C.C.#
###A#B#.#.###
  #A#B#.#D#
  #A#B#.#D#
  #A#B#C#D#
  #########",
        );

        print_map(&burrows);
        assert_eq!(num_amphipods(&burrows), 16);
        assert_eq!(num_organized_amphipods(&burrows), 12);
    }

    #[test]
    fn test_initial_valid_locations() {
        let burrows = map_from_string(
            "#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########",
        );

        let valid_moves = get_valid_moves(&burrows).len();
        assert_eq!(valid_moves, 28);
    }

    #[test]
    fn test_second_valid_locations() {
        let burrows = map_from_string(
            "#############
#...B.......#
###B#C#.#D###
  #A#D#C#A#
  #########",
        );

        let valid_moves = get_valid_moves(&burrows).len();
        assert_eq!(valid_moves, 10);
    }

    #[test]
    fn test_third_valid_locations() {
        let burrows = map_from_string(
            "#############
#...B.......#
###B#.#C#D###
  #A#D#C#A#
  #########",
        );

        let valid_moves = get_valid_moves(&burrows).len();
        assert_eq!(valid_moves, 10);
    }

    #[test]
    fn test_fourth_valid_locations() {
        let burrows = map_from_string(
            "#############
#...B.D.....#
###B#.#C#D###
  #A#.#C#A#
  #########",
        );

        let valid_moves = get_valid_moves(&burrows).len();
        assert_eq!(valid_moves, 6);
    }

    #[test]
    fn test_fifth_valid_locations() {
        let burrows = map_from_string(
            "#############
#.....D.....#
###B#.#C#D###
  #A#B#C#A#
  #########",
        );

        print_map(&burrows);
        let valid_moves = get_valid_moves(&burrows);
        for valid_move in &valid_moves {
            println!("move: {:?}", valid_move);
        }
        assert_eq!(valid_moves.len(), 7);
    }

    #[test]
    fn test_sixth_valid_locations() {
        let burrows = map_from_string(
            "#############
#.....D.....#
###.#B#C#D###
  #A#B#C#A#
  #########",
        );

        let valid_moves = get_valid_moves(&burrows);
        assert_eq!(valid_moves.len(), 3);
    }

    #[test]
    fn test_seventh_valid_locations() {
        let burrows = map_from_string(
            "#############
#.....D.D...#
###.#B#C#.###
  #A#B#C#A#
  #########",
        );

        let valid_moves = get_valid_moves(&burrows);
        assert_eq!(valid_moves.len(), 2);
    }

    #[test]
    fn test_eigth_valid_locations() {
        let burrows = map_from_string(
            "#############
#.....D.D.A.#
###.#B#C#.###
  #A#B#C#.#
  #########",
        );

        let valid_moves = get_valid_moves(&burrows);
        assert_eq!(valid_moves.len(), 1);
    }

    #[test]
    fn test_ninth_valid_locations() {
        let burrows = map_from_string(
            "#############
#.....D...A.#
###.#B#C#.###
  #A#B#C#D#
  #########",
        );

        let valid_moves = get_valid_moves(&burrows);
        assert_eq!(valid_moves.len(), 1);
    }

    #[test]
    fn test_tenth_valid_locations() {
        let burrows = map_from_string(
            "#############
#.........A.#
###.#B#C#D###
  #A#B#C#D#
  #########",
        );

        let valid_moves = get_valid_moves(&burrows);
        assert_eq!(valid_moves.len(), 1);
    }

    #[test]
    fn test_no_moves() {
        {
            let burrows = map_from_string(
                "#############
#...A.......#
###A#.#C#D###
  #A#B#C#D#
  #########",
            );

            let valid_moves = get_valid_moves(&burrows);
            for valid_move in &valid_moves {
                println!("valid move: {:?}", valid_move);
            }
            print_map(&burrows);

            assert_eq!(valid_moves.len(), 0);
        }
        {
            let burrows = map_from_string(
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
            let burrows = map_from_string(
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
        let burrows = map_from_string(
            "#############
#.....B.C...#
###B#C#.#D###
  #A#D#.#A#
  #########",
        );

        let valid_moves = get_valid_moves(&burrows);

        let mut has_a_valid_home_move = false;
        for valid_move in valid_moves {
            println!("valid_move: {:?}", valid_move);
            if valid_move == (MapType::AmphipodCopper, (8, 1), (7, 3), 300) {
                has_a_valid_home_move = true;
            }
        }
        assert_eq!(has_a_valid_home_move, true);
    }
}
