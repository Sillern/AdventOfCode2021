use itertools::Itertools;
use std::collections::HashMap;
use std::env;
use std::fmt;

type Registers = Vec<i64>;

#[derive(Debug)]
enum Verb {
    Input,
    Add,
    Multiply,
    Divide,
    Modulo,
    Equal,
}

#[derive(Debug)]
enum AddressingMode {
    NoSource,
    Indirect,
    Direct,
}

#[derive(Debug)]
struct Instruction {
    verb: Verb,
    destination: usize,
    addressing_mode: AddressingMode,
    indirect: usize,
    direct: i64,
}

struct ALU {
    instructions: Vec<Instruction>,
    registers: Registers,
}

impl Instruction {
    pub fn from_string(input: &str) -> Self {
        let mut tokens = input.split(" ");
        let verb = match tokens.next() {
            Some("inp") => Verb::Input,
            Some("add") => Verb::Add,
            Some("mul") => Verb::Multiply,
            Some("div") => Verb::Divide,
            Some("mod") => Verb::Modulo,
            Some("eql") => Verb::Equal,
            _ => panic!(),
        };

        let mut indirect = 0;
        let mut direct = 0;
        let mut addressing_mode = AddressingMode::NoSource;
        let destination = Self::register_index(tokens.next().unwrap());
        if let Some(source) = tokens.next() {
            if let Ok(value) = source.parse::<i64>() {
                addressing_mode = AddressingMode::Direct;
                direct = value;
            } else {
                addressing_mode = AddressingMode::Indirect;
                indirect = Self::register_index(source);
            }
        }

        Self {
            verb,
            destination,
            addressing_mode,
            indirect,
            direct,
        }
    }

    pub fn register_index(operand: &str) -> usize {
        match operand {
            "w" => 0,
            "x" => 1,
            "y" => 2,
            "z" => 3,
            _ => panic!(),
        }
    }

    pub fn execute(&self, registers: &mut Registers, input_stream: &mut Vec<i64>) {
        let source = match self.addressing_mode {
            AddressingMode::NoSource => 0,
            AddressingMode::Indirect => registers[self.indirect],
            AddressingMode::Direct => self.direct,
        };

        match self.verb {
            Verb::Input => registers[self.destination] = input_stream.pop().unwrap(),
            Verb::Add => registers[self.destination] += source,
            Verb::Multiply => registers[self.destination] *= source,
            Verb::Divide => registers[self.destination] /= source,
            Verb::Modulo => registers[self.destination] %= source,
            Verb::Equal => {
                registers[self.destination] = if source == registers[self.destination] {
                    1
                } else {
                    0
                }
            }
            _ => panic!(),
        }
    }
}

impl ALU {
    pub fn from_file(inputfile: &str) -> Self {
        Self::from_string(
            &std::fs::read_to_string(inputfile).expect("Something went wrong reading the file"),
        )
    }
    pub fn from_string(input: &str) -> Self {
        Self {
            instructions: input
                .lines()
                .map(|line| Instruction::from_string(line))
                .collect::<Vec<Instruction>>(),
            registers: vec![0, 0, 0, 0],
        }
    }

    pub fn calculate(&mut self, input_stream: &mut Vec<i64>) -> i64 {
        for instruction in &self.instructions {
            instruction.execute(&mut self.registers, input_stream);
        }
        self.registers[3]
    }
    pub fn reset(&mut self) {
        self.registers = vec![0, 0, 0, 0];
    }
}

fn solve_part1(inputfile: String) -> i64 {
    let mut alu = ALU::from_file(&inputfile);

    for a in 1..9 {
        for b in 1..9 {
            for c in 1..9 {
                for d in 1..9 {
                    for e in 1..9 {
                        for f in 1..9 {
                            for g in 1..9 {
                                println!("{:?}", (a, b, c, d, e, f, g));
                                for h in 1..9 {
                                    for i in 1..9 {
                                        for j in 1..9 {
                                            for k in 1..9 {
                                                for l in 1..9 {
                                                    for m in 1..9 {
                                                        for n in 1..9 {
                                                            alu.reset();
                                                            let mut input_stream = vec![
                                                                10 - n,
                                                                10 - m,
                                                                10 - l,
                                                                10 - k,
                                                                10 - j,
                                                                10 - i,
                                                                10 - h,
                                                                10 - g,
                                                                10 - f,
                                                                10 - e,
                                                                10 - d,
                                                                10 - c,
                                                                10 - b,
                                                                10 - a,
                                                            ];
                                                            if alu.calculate(&mut input_stream) == 0
                                                            {
                                                                println!(
                                                                    "{:?}",
                                                                    [
                                                                        10 - a,
                                                                        10 - b,
                                                                        10 - c,
                                                                        10 - d,
                                                                        10 - e,
                                                                        10 - f,
                                                                        10 - g,
                                                                        10 - h,
                                                                        10 - i,
                                                                        10 - j,
                                                                        10 - k,
                                                                        10 - l,
                                                                        10 - m,
                                                                        10 - n,
                                                                    ]
                                                                );
                                                                return 0;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    0
}

fn solve_part2(inputfile: String) -> i64 {
    0
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Part1: {}", solve_part1(args[1].to_string()));
    println!("Part2: {}", solve_part2(args[1].to_string()));
}
