use itertools::Itertools;
use std::collections::HashMap;
use std::env;

#[derive(Debug)]
struct Packet {
    version: u8, // 3 bits
    type_id: u8, // 3 bits
    literal: Option<Vec<u8>>,
    subpackets: Option<Vec<Packet>>,
}
impl Packet {
    pub fn new(version: u32, type_id: u32) -> Self {
        Self {
            version: version as u8,
            type_id: type_id as u8,
            literal: None,
            subpackets: None,
        }
    }

    pub fn add_subpacket(&mut self, subpacket: Packet) {
        match &mut self.subpackets {
            Some(subpackets) => subpackets.push(subpacket),
            None => self.subpackets = Some(vec![subpacket]),
        }
    }

    pub fn get_version_sum(&self) -> usize {
        self.version as usize
            + match &self.subpackets {
                Some(subpackets) => subpackets
                    .iter()
                    .fold(0, |acc, packet| acc + packet.get_version_sum()),
                None => 0,
            }
    }
}

fn parse_input(inputfile: String) -> Vec<Packet> {
    let mut packets: Vec<Packet> = vec![];

    let conversion: HashMap<char, &str> = HashMap::from([
        ('0', "0000"),
        ('1', "0001"),
        ('2', "0010"),
        ('3', "0011"),
        ('4', "0100"),
        ('5', "0101"),
        ('6', "0110"),
        ('7', "0111"),
        ('8', "1000"),
        ('9', "1001"),
        ('A', "1010"),
        ('B', "1011"),
        ('C', "1100"),
        ('D', "1101"),
        ('E', "1110"),
        ('F', "1111"),
    ]);

    fn bits_to_int(it: &mut dyn Iterator<Item = char>, num_bits: usize) -> u32 {
        it.take(num_bits).enumerate().fold(0, |acc, (index, bit)| {
            acc + (bit.to_digit(10).unwrap() << ((num_bits - 1) - index))
        })
    }

    fn parse_packet(it: &mut dyn Iterator<Item = char>) -> Option<Packet> {
        let version = bits_to_int(it, 3);
        let type_id = bits_to_int(it, 3);

        let mut packet = Packet::new(version, type_id);
        let mut values: Vec<u32> = vec![];
        match type_id {
            4 => {
                // Literal value
                let mut end_of_packet = false;
                while !end_of_packet {
                    end_of_packet = match it.next() {
                        Some('1') => false,
                        Some('0') => true,
                        Some(_) => panic!(),
                        None => panic!(),
                    };

                    let value = bits_to_int(it, 4);

                    values.push(value)
                }
                Some(packet)
            }
            _ => {
                // Operator
                match it.next() {
                    Some(length_type_id) => {
                        let static_length = length_type_id == '0';
                        if static_length {
                            let length = bits_to_int(it, 15);
                            if length > 0 {
                                packet.add_subpacket(parse_packet(it).unwrap());
                                Some(packet)
                            } else {
                                None
                            }
                        } else {
                            let num_subpackets = bits_to_int(it, 11);
                            for _ in 0..num_subpackets {
                                packet.add_subpacket(parse_packet(it).unwrap());
                            }
                            Some(packet)
                        }
                    }
                    None => None,
                }
            }
        }
    }

    let packets = std::fs::read_to_string(inputfile)
        .expect("Something went wrong reading the file")
        .lines()
        .map(|line| line.chars().map(|c| conversion.get(&c).unwrap()).join(""))
        .collect::<String>()
        .chars()
        .batching(|it| {
            let chars_left = it.clone().count();
            println!("chars_left: {:?}", chars_left);
            if chars_left > 0 {
                parse_packet(it)
            } else {
                None
            }
        })
        .collect::<Vec<Packet>>();

    packets
}

fn solve_part1(inputfile: String) -> usize {
    let parsed = parse_input(inputfile);
    //    println!("parsed: {:?}", parsed);

    parsed
        .iter()
        .fold(0, |acc, packet| acc + packet.get_version_sum())
}

fn solve_part2(inputfile: String) -> usize {
    let parsed = parse_input(inputfile);
    //    println!("parsed: {:?}", parsed);

    parsed
        .iter()
        .fold(0, |acc, packet| acc + packet.get_version_sum())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Part1: {}", solve_part1(args[1].to_string()));
    println!("Part2: {}", solve_part2(args[1].to_string()));
}
