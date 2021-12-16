use itertools::Itertools;
use std::collections::HashMap;
use std::env;

#[derive(Debug)]
struct Packet {
    version: u8, // 3 bits
    type_id: u8, // 3 bits
    literals: Vec<u8>,
    subpackets: Vec<Packet>,
}

impl Packet {
    pub fn new(version: u32, type_id: u32) -> Self {
        Self {
            version: version as u8,
            type_id: type_id as u8,
            literals: vec![],
            subpackets: vec![],
        }
    }

    pub fn get_literal(&self) -> u64 {
        assert!(self.literals.len() < 16);
        self.literals
            .iter()
            .rev()
            .enumerate()
            .fold(0, |acc, (index, halfbyte)| {
                acc + ((*halfbyte as u64) << (index * 4))
            })
    }

    pub fn add_subpacket(&mut self, subpacket: Packet) {
        self.subpackets.push(subpacket)
    }

    pub fn add_literal(&mut self, literal: u8) {
        self.literals.push(literal)
    }

    pub fn get_version_sum(&self) -> usize {
        self.version as usize
            + self
                .subpackets
                .iter()
                .fold(0, |acc, packet| acc + packet.get_version_sum())
    }

    pub fn execute(&self) -> u64 {
        match self.type_id {
            0 => {
                // sum
                self.subpackets
                    .iter()
                    .fold(0, |acc, packet| acc + packet.execute())
            }
            1 => {
                // product
                self.subpackets
                    .iter()
                    .fold(1, |acc, packet| acc * packet.execute())
            }
            2 => {
                // minimum
                self.subpackets
                    .iter()
                    .map(|packet| packet.execute())
                    .min()
                    .unwrap()
            }
            3 => {
                // maximum
                self.subpackets
                    .iter()
                    .map(|packet| packet.execute())
                    .max()
                    .unwrap()
            }
            4 => {
                // literal
                self.get_literal()
            }
            5 => {
                // greater than
                assert!(self.subpackets.len() == 2);

                let a = self.subpackets[0].execute();
                let b = self.subpackets[1].execute();
                if a > b {
                    1
                } else {
                    0
                }
            }
            6 => {
                // less than
                assert!(self.subpackets.len() == 2);
                let a = self.subpackets[0].execute();
                let b = self.subpackets[1].execute();
                if a < b {
                    1
                } else {
                    0
                }
            }
            7 => {
                // equal
                assert!(self.subpackets.len() == 2);
                let a = self.subpackets[0].execute();
                let b = self.subpackets[1].execute();
                if a == b {
                    1
                } else {
                    0
                }
            }
            _ => {
                panic!()
            }
        }
    }
}

fn parse_string(input: &str) -> Vec<Packet> {
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

                    packet.add_literal(bits_to_int(it, 4) as u8);
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
                                let subpacket_stream = &mut it.take(length as usize);
                                while let Some(subpacket) = parse_packet(subpacket_stream) {
                                    packet.add_subpacket(subpacket);
                                }
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

    input
        .chars()
        .map(|c| conversion.get(&c).unwrap())
        .join("")
        .chars()
        .batching(|it| {
            let chars_left = it.clone().count();
            if chars_left > 0 {
                parse_packet(it)
            } else {
                None
            }
        })
        .collect::<Vec<Packet>>()
}

fn parse_input(inputfile: String) -> Vec<Packet> {
    std::fs::read_to_string(inputfile)
        .expect("Something went wrong reading the file")
        .lines()
        .map(|line| parse_string(line))
        .flatten()
        .collect::<Vec<Packet>>()
}

fn solve_part1(inputfile: String) -> usize {
    let parsed = parse_input(inputfile);

    parsed
        .iter()
        .fold(0, |acc, packet| acc + packet.get_version_sum())
}

fn solve_part2(inputfile: String) -> usize {
    let parsed = parse_input(inputfile);

    parsed[0].execute() as usize
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
    fn test_version_sum() {
        let packets = parse_string("A0016C880162017C3686B18A3D4780");
        assert_eq!(packets.len(), 1);
        assert_eq!(packets[0].get_version_sum(), 31);
    }

    #[test]
    fn test_literal_value() {
        let packets = parse_string("D2FE28");
        assert_eq!(packets.len(), 1);
        assert_eq!(packets[0].get_literal(), 2021);
    }

    #[test]
    fn test_sum() {
        let packets = parse_string("C200B40A82");
        assert_eq!(packets.len(), 1);
        assert_eq!(packets[0].execute(), 3);
    }

    #[test]
    fn test_product() {
        let packets = parse_string("04005AC33890");
        println!("product test: {:?}: {}", packets, packets.len());
        for packet in &packets {
            println!("packet: {:?}", packet);
        }

        assert_eq!(packets.len(), 1);
        assert_eq!(packets[0].execute(), 54);
    }

    #[test]
    fn test_minimum() {
        let packets = parse_string("880086C3E88112");
        assert_eq!(packets.len(), 1);
        assert_eq!(packets[0].execute(), 7);
    }

    #[test]
    fn test_maximum() {
        let packets = parse_string("CE00C43D881120");
        assert_eq!(packets.len(), 1);
        assert_eq!(packets[0].execute(), 9);
    }

    #[test]
    fn test_less_than() {
        let packets = parse_string("D8005AC2A8F0");
        assert_eq!(packets.len(), 1);
        assert_eq!(packets[0].execute(), 1);
    }

    #[test]
    fn test_greater_than() {
        let packets = parse_string("F600BC2D8F");
        assert_eq!(packets.len(), 1);
        assert_eq!(packets[0].execute(), 0);
    }

    #[test]
    fn test_equality() {
        let packets = parse_string("9C005AC2F8F0");
        assert_eq!(packets.len(), 1);
        assert_eq!(packets[0].execute(), 0);
    }

    #[test]
    fn test_composed_equality() {
        let packets = parse_string("9C0141080250320F1802104A08");
        assert_eq!(packets.len(), 1);
        assert_eq!(packets[0].execute(), 1);
    }
}
