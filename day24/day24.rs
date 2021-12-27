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

    pub fn execute_reverse(&self, registers: &mut Registers, output_stream: &mut Vec<i64>) {
        let source = match self.addressing_mode {
            AddressingMode::NoSource => 0,
            AddressingMode::Indirect => registers[self.indirect],
            AddressingMode::Direct => self.direct,
        };

        match self.verb {
            Verb::Input => output_stream.push(registers[self.destination]),
            Verb::Add => registers[self.destination] -= source,
            Verb::Multiply => registers[self.destination] /= source,
            Verb::Divide => registers[self.destination] *= source,
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
    /*
    let mut alu = ALU::from_file(&inputfile);

    for a in 0..9 {
        for b in 0..9 {
            for c in 0..9 {
                for d in 0..9 {
                    for e in 0..9 {
                        for f in 0..9 {
                            println!("{:?}", [9 - a, 9 - b, 9 - c, 9 - d, 9 - e, 9 - f,],);
                            for g in 0..9 {
                                for h in 0..9 {
                                    for i in 0..9 {
                                        for j in 0..9 {
                                            for k in 0..9 {
                                                for l in 0..9 {
                                                    for m in 0..9 {
                                                        for n in 0..9 {
                                                            /*
                                                            alu.reset();
                                                            let mut input_stream = vec![
                                                                9 - n,
                                                                9 - m,
                                                                9 - l,
                                                                9 - k,
                                                                9 - j,
                                                                9 - i,
                                                                9 - h,
                                                                9 - g,
                                                                9 - f,
                                                                9 - e,
                                                                9 - d,
                                                                9 - c,
                                                                9 - b,
                                                                9 - a,
                                                            ];

                                                            if alu.calculate(&mut input_stream) == 0
                                                            {
                                                                println!(
                                                                    "{:?}",
                                                                    [
                                                                        9 - a,
                                                                        9 - b,
                                                                        9 - c,
                                                                        9 - d,
                                                                        9 - e,
                                                                        9 - f,
                                                                        9 - g,
                                                                        9 - h,
                                                                        9 - i,
                                                                        9 - j,
                                                                        9 - k,
                                                                        9 - l,
                                                                        9 - m,
                                                                        9 - n,
                                                                    ]
                                                                );
                                                                return 0;
                                                            }*/
                                                            if full_equivalent(
                                                                9 - a,
                                                                9 - b,
                                                                9 - c,
                                                                9 - d,
                                                                9 - e,
                                                                9 - f,
                                                                9 - g,
                                                                9 - h,
                                                                9 - i,
                                                                9 - j,
                                                                9 - k,
                                                                9 - l,
                                                                9 - m,
                                                                9 - n,
                                                            ) == 0
                                                            {
                                                                println!(
                                                                    "{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
                                                                    9 - a,
                                                                    9 - b,
                                                                    9 - c,
                                                                    9 - d,
                                                                    9 - e,
                                                                    9 - f,
                                                                    9 - g,
                                                                    9 - h,
                                                                    9 - i,
                                                                    9 - j,
                                                                    9 - k,
                                                                    9 - l,
                                                                    9 - m,
                                                                    9 - n,
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
    }*/
    0
}

fn solve_part2(inputfile: String) -> i64 {
    for a in 0..9 {
        for b in 0..9 {
            for c in 0..9 {
                for d in 0..9 {
                    for e in 0..9 {
                        for f in 0..9 {
                            println!("{:?}", [1 + a, 1 + b, 1 + c, 1 + d, 1 + e, 1 + f,],);
                            for g in 0..9 {
                                for h in 0..9 {
                                    for i in 0..9 {
                                        for j in 0..9 {
                                            for k in 0..9 {
                                                for l in 0..9 {
                                                    for m in 0..9 {
                                                        for n in 0..9 {
                                                            if full_equivalent(
                                                                1 + a,
                                                                1 + b,
                                                                1 + c,
                                                                1 + d,
                                                                1 + e,
                                                                1 + f,
                                                                1 + g,
                                                                1 + h,
                                                                1 + i,
                                                                1 + j,
                                                                1 + k,
                                                                1 + l,
                                                                1 + m,
                                                                1 + n,
                                                            ) == 0
                                                            {
                                                                println!(
                                                                    "{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
                                                                    1 + a,
                                                                    1 + b,
                                                                    1 + c,
                                                                    1 + d,
                                                                    1 + e,
                                                                    1 + f,
                                                                    1 + g,
                                                                    1 + h,
                                                                    1 + i,
                                                                    1 + j,
                                                                    1 + k,
                                                                    1 + l,
                                                                    1 + m,
                                                                    1 + n,
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

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Part1: {}", solve_part1(args[1].to_string()));
    println!("Part2: {}", solve_part2(args[1].to_string()));
}

fn full_equivalent(
    a: i64,
    b: i64,
    c: i64,
    d: i64,
    e: i64,
    f: i64,
    g: i64,
    h: i64,
    i: i64,
    j: i64,
    k: i64,
    l: i64,
    m: i64,
    n: i64,
) -> i64 {
    let mut z = 0;

    if ((z % 26) + 11) != a {
        z = (z * 26) + (a + 6)
    }

    if ((z % 26) + 11) != b {
        z = (z * 26) + (b + 12)
    }

    if ((z % 26) + 15) != c {
        z = (z * 26) + (c + 8)
    }

    if ((z % 26) + -11) == d {
        z = z / 26
    } else {
        z = ((z / 26) * 26) + (d + 7)
    }

    if ((z % 26) + 15) != e {
        z = (z * 26) + (e + 7)
    }

    if ((z % 26) + 15) != f {
        z = (z * 26) + (f + 12)
    }

    if ((z % 26) + 14) != g {
        z = (z * 26) + (g + 2)
    }

    if ((z % 26) + -7) == h {
        z = z / 26
    } else {
        z = ((z / 26) * 26) + (h + 15)
    }

    if ((z % 26) + 12) != i {
        z = (z * 26) + (i + 4)
    }

    if ((z % 26) + -6) == j {
        z = z / 26
    } else {
        z = ((z / 26) * 26) + (j + 5)
    }

    if ((z % 26) + -10) == k {
        z = z / 26
    } else {
        z = ((z / 26) * 26) + (k + 12)
    }

    if ((z % 26) + -15) == l {
        z = z / 26
    } else {
        z = ((z / 26) * 26) + (l + 11)
    }

    if ((z % 26) + -9) == m {
        z = z / 26
    } else {
        z = ((z / 26) * 26) + (m + 13)
    }

    if ((z % 26) + 0) == n {
        z = z / 26
    } else {
        z = ((z / 26) * 26) + (n + 7)
    }

    return z;
}

fn equivalent(z: i64, w: i64, div_constant: i64, factor1: i64, factor2: i64) -> i64 {
    if ((z % 26) + factor1) == w {
        z / div_constant
    } else {
        ((z / div_constant) * 26) + (w + factor2)
    }
}

/*
fn reverse_equivalent(z: &mut i64, div_constant: i64, factor1: i64, factor2: i64) -> i64 {


    let path1_z = *z * div_constant;
    let mut w = (path1_z % 26) + factor1;

    if w ==
    (path1_z % 26) == (w - factor1) % 26;

    let path2_z = ( (*z - (w + factor2)) / 26) * div_constant;


}
*/

#[cfg(test)]
mod tests {
    use super::*;

    /*
    #[test]
    fn test_reverse_calculation() {
        {
            let mut eqv_result = 0;
            let w = reverse_equivalent(&mut eqv_result, 26, 0, 7);
            assert_eq!(w, 5);
        }
    }
    */

    #[test]
    fn test_zero_calculation() {
        {
            let mut alu = ALU::from_file("day24/input.txt");
            // 13579246899999
            let mut input_stream = vec![9, 9, 9, 9, 9, 8, 6, 4, 2, 9, 7, 5, 3, 1];
            let result = alu.calculate(&mut input_stream);

            {
                let input_stream = vec![1, 3, 5, 7, 9, 2, 4, 6, 8, 9, 9, 9, 9, 9];
                //let input_stream = vec![9, 9, 9, 9, 9, 8, 6, 4, 2, 9, 7, 5, 3, 1];

                let eqv_result = full_equivalent(1, 3, 5, 7, 9, 2, 4, 6, 8, 9, 9, 9, 9, 9);
                assert_eq!(result, eqv_result);
            }
        }
    }
    #[test]
    fn test_full_calculation() {
        {
            let mut alu = ALU::from_file("day24/input.txt");

            let mut input_stream = vec![5, 4, 3, 2, 1, 9, 8, 7, 6, 5, 4, 3, 2, 1];
            let result = alu.calculate(&mut input_stream);
            {
                let mut eqv_result = 0;
                eqv_result = equivalent(eqv_result, 1, 1, 11, 6);
                eqv_result = equivalent(eqv_result, 2, 1, 11, 12);
                eqv_result = equivalent(eqv_result, 3, 1, 15, 8);
                eqv_result = equivalent(eqv_result, 4, 26, -11, 7);
                eqv_result = equivalent(eqv_result, 5, 1, 15, 7);
                eqv_result = equivalent(eqv_result, 6, 1, 15, 12);
                eqv_result = equivalent(eqv_result, 7, 1, 14, 2);
                eqv_result = equivalent(eqv_result, 8, 26, -7, 15);
                eqv_result = equivalent(eqv_result, 9, 1, 12, 4);
                eqv_result = equivalent(eqv_result, 1, 26, -6, 5);
                eqv_result = equivalent(eqv_result, 2, 26, -10, 12);
                eqv_result = equivalent(eqv_result, 3, 26, -15, 11);
                eqv_result = equivalent(eqv_result, 4, 26, -9, 13);
                eqv_result = equivalent(eqv_result, 5, 26, 0, 7);
                assert_eq!(result, eqv_result);
            }

            {
                let eqv_result = full_equivalent(1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 2, 3, 4, 5);
                assert_eq!(result, eqv_result);
            }
        }
    }

    #[test]
    fn test_factor_calculation() {
        {
            let mut alu = ALU::from_string(
                "add z 1000
inp w
mul x 0
add x z
mod x 26
div z 1
add x 11
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 6
mul y x
add z y",
            );

            let mut input_stream = vec![9];
            let result = alu.calculate(&mut input_stream);
            let mut eqv_result = 1000;
            eqv_result = equivalent(eqv_result, 9, 1, 11, 6);
            assert_eq!(result, eqv_result);
        }

        {
            let mut alu = ALU::from_string(
                "inp w
mul x 0
add x z
mod x 26
div z 1
add x 15
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 8
mul y x
add z y",
            );
            let mut input_stream = vec![9];
            let result = alu.calculate(&mut input_stream);
            let mut eqv_result = 0;
            eqv_result = equivalent(eqv_result, 9, 1, 15, 8);
            assert_eq!(result, eqv_result);
        }

        {
            let mut alu = ALU::from_string(
                "add z 10
inp w
mul x 0
add x z
mod x 26
div z 26
add x -10
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 12
mul y x
add z y",
            );
            let mut input_stream = vec![3];
            let result = alu.calculate(&mut input_stream);
            let mut eqv_result = 10;
            eqv_result = equivalent(eqv_result, 3, 26, -10, 12);
            assert_eq!(result, eqv_result);
        }
    }
}
