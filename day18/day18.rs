use itertools::Itertools;
use std::env;
use std::fmt;
use std::ops;

#[derive(Debug)]
struct SnailFish {
    a: Vec<SnailFish>,
    b: Vec<SnailFish>,
    a_literal: usize,
    b_literal: usize,
}

impl SnailFish {
    pub fn new(a: SnailFish, b: SnailFish) -> Self {
        Self {
            a: vec![a],
            b: vec![b],
            a_literal: 0,
            b_literal: 0,
        }
    }

    pub fn from_literal(value: usize) -> Self {
        Self {
            a: vec![],
            b: vec![],
            a_literal: (value / 2) as usize,
            b_literal: ((value + 1) / 2) as usize,
        }
    }

    pub fn empty() -> Self {
        Self {
            a: vec![],
            b: vec![],
            a_literal: 0,
            b_literal: 0,
        }
    }

    fn magnitude(&self) -> usize {
        let a_value = if self.a.len() != 0 {
            self.a[0].magnitude()
        } else {
            self.a_literal
        };
        let b_value = if self.b.len() != 0 {
            self.b[0].magnitude()
        } else {
            self.b_literal
        };

        3 * a_value + 2 * b_value
    }

    fn could_explode(&self, num_pairs: usize) -> bool {
        if num_pairs == 3 && (self.a.len() != 0 || self.b.len() != 0) {
            true
        } else {
            if self.a.len() == 0 && self.b.len() != 0 {
                self.b[0].could_explode(num_pairs + 1)
            } else if self.b.len() == 0 && self.a.len() != 0 {
                self.a[0].could_explode(num_pairs + 1)
            } else if self.a.len() == 0 && self.b.len() == 0 {
                false
            } else {
                let a = self.a[0].could_explode(num_pairs + 1);
                let b = self.b[0].could_explode(num_pairs + 1);
                a || b
            }
        }
    }

    fn add_leftmost(&mut self, value: usize) {
        if self.a.len() == 0 {
            self.a_literal += value;
        } else {
            self.a[0].add_leftmost(value);
        }
    }

    fn add_rightmost(&mut self, value: usize) {
        if self.b.len() == 0 {
            self.b_literal += value;
        } else {
            self.b[0].add_rightmost(value);
        }
    }

    fn explode(&mut self, num_pairs: usize, has_exploded: bool) -> (usize, usize, bool) {
        if !has_exploded {
            if num_pairs == 3 && (self.a.len() != 0 || self.b.len() != 0) {
                if self.a.len() != 0 {
                    println!(
                        "{:indent$}exploding branch a: {}",
                        "",
                        self,
                        indent = num_pairs + 1
                    );
                    assert!(self.a[0].b.len() == 0);
                    assert!(self.a[0].a.len() == 0);

                    let a = self.a[0].a_literal;
                    let mut b = self.a[0].b_literal;

                    if self.b.len() == 0 {
                        self.b_literal += b;
                        b = 0;
                    } else {
                        self.b[0].add_leftmost(b);
                        b = 0;
                    }

                    self.a_literal = 0;
                    self.a = vec![];
                    (a, b, true)
                } else if self.b.len() != 0 {
                    println!(
                        "{:indent$}exploding branch b: {}",
                        "",
                        self,
                        indent = num_pairs + 1
                    );
                    assert!(self.b[0].a.len() == 0);
                    assert!(self.b[0].b.len() == 0);

                    let mut a = self.b[0].a_literal;
                    let b = self.b[0].b_literal;

                    if self.a.len() == 0 {
                        self.a_literal += a;
                        a = 0;
                    } else {
                        panic!();
                    }

                    self.b = vec![];
                    self.b_literal = 0;
                    (a, b, true)
                } else {
                    panic!();
                    (0, 0, has_exploded)
                }
            } else {
                if self.a.len() == 0 && self.b.len() != 0 {
                    let values = self.b[0].explode(num_pairs + 1, has_exploded);
                    let mut a = values.0;
                    let mut b = values.1;
                    println!(
                        "{:indent$}recursed with literal a: {}, {:?}",
                        "",
                        self,
                        values,
                        indent = num_pairs + 1
                    );

                    if values.2 {
                        self.a_literal += a;
                        a = 0;
                        println!(
                            "{:indent$}updated with literal a: {}, {:?}",
                            "",
                            self,
                            (a, b, values.2),
                            indent = num_pairs + 1
                        );
                    }
                    (a, b, values.2)
                } else if self.b.len() == 0 && self.a.len() != 0 {
                    let values = self.a[0].explode(num_pairs + 1, has_exploded);
                    let mut a = values.0;
                    let mut b = values.1;
                    println!(
                        "{:indent$}recursed with literal b: {}, {:?}",
                        "",
                        self,
                        values,
                        indent = num_pairs + 1
                    );
                    if values.2 {
                        self.b_literal += b;
                        b = 0;
                        println!(
                            "{:indent$}updated with literal b: {}, {:?}",
                            "",
                            self,
                            (a, b, values.2),
                            indent = num_pairs + 1
                        );
                    }
                    (a, b, values.2)
                } else if self.a.len() == 0 && self.b.len() == 0 {
                    assert!(!has_exploded);
                    (0, 0, has_exploded)
                } else {
                    println!(
                        "{:indent$}recursing both branches: {}",
                        "",
                        self,
                        indent = num_pairs + 1
                    );

                    let a_values = self.a[0].explode(num_pairs + 1, has_exploded);
                    if a_values.2 {
                        if a_values.1 != 0 {
                            println!(
                                "{:indent$}updated[a]: {} --   -- {}",
                                "",
                                self.a[0],
                                self.b[0],
                                indent = num_pairs + 1
                            );
                            self.b[0].add_leftmost(a_values.1);
                            println!(
                                "{:indent$}updated[a]: {} -- {} -> {}",
                                "",
                                self.a[0],
                                a_values.1,
                                self.b[0],
                                indent = num_pairs + 1
                            );
                        }

                        return (a_values.0, 0, true);
                    } else {
                        let b_values = self.b[0].explode(num_pairs + 1, has_exploded);

                        if b_values.2 {
                            if b_values.0 != 0 {
                                println!(
                                    "{:indent$}updated[b]:  {} --   -- {}",
                                    "",
                                    self.a[0],
                                    self.b[0],
                                    indent = num_pairs + 1
                                );

                                self.a[0].add_rightmost(b_values.0);

                                println!(
                                    "{:indent$}updated[b]: {} <- {} -- {}",
                                    "",
                                    self.a[0],
                                    b_values.0,
                                    self.b[0],
                                    indent = num_pairs + 1
                                );
                            }
                            return (0, b_values.1, true);
                        }
                    }

                    println!(
                        "{:indent$}recursed both branches: {}: {:?}",
                        "",
                        self,
                        a_values,
                        indent = num_pairs + 1
                    );
                    a_values
                }
            }
        } else {
            panic!();
            (0, 0, has_exploded)
        }
    }

    fn could_split(&self) -> bool {
        let a_could_split = if self.a.len() != 0 {
            self.a[0].could_split()
        } else {
            false
        };

        let b_could_split = if self.b.len() != 0 {
            self.b[0].could_split()
        } else {
            false
        };

        if self.a.len() == 0 && self.a_literal >= 10 {
            true
        } else if self.b.len() == 0 && self.b_literal >= 10 {
            true
        } else {
            a_could_split || b_could_split
        }
    }

    fn split(&mut self, has_split: bool) -> bool {
        let a_has_split = if self.a.len() != 0 {
            self.a[0].split(has_split)
        } else if !has_split && self.a_literal >= 10 {
            self.a.push(Self::from_literal(self.a_literal));
            self.a_literal = 0;
            true
        } else {
            false
        };

        let b_has_split = if self.b.len() != 0 {
            self.b[0].split(has_split || a_has_split)
        } else if !has_split && !a_has_split && self.b_literal >= 10 {
            self.b.push(Self::from_literal(self.b_literal));
            self.b_literal = 0;
            true
        } else {
            false
        };

        a_has_split || b_has_split
    }
}

impl ops::Add<Self> for SnailFish {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let mut addition = SnailFish::new(self, rhs);
        println!("after addition: {}", addition);

        while addition.could_explode(0) || addition.could_split() {
            if addition.could_explode(0) {
                addition.explode(0, false);
                println!("after explode:  {}", addition);
            } else if addition.could_split() {
                addition.split(false);
                println!("after split:    {}", addition);
            }
        }
        addition
    }
}

impl fmt::Display for SnailFish {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[{},{}]",
            if let Some(a_snailfish) = self.a.first() {
                format!("{}", a_snailfish)
            } else {
                format!("{}", self.a_literal)
            },
            if let Some(b_snailfish) = self.b.first() {
                format!("{}", b_snailfish)
            } else {
                format!("{}", self.b_literal)
            }
        )
    }
}

fn parse_snailfish(it: &mut dyn Iterator<Item = char>, first_iteration: bool) -> SnailFish {
    let mut snailfish: SnailFish = SnailFish::empty();
    let mut first_value = true;
    loop {
        match it.next() {
            Some('[') => {
                let next_snailfish = parse_snailfish(it, false);

                if first_iteration {
                    snailfish = next_snailfish;
                } else {
                    if first_value {
                        snailfish.a.push(next_snailfish);
                    } else {
                        snailfish.b.push(next_snailfish);
                    }
                }
            }
            Some(']') => break,
            Some(',') => {
                first_value = false;
            }
            Some(value) => {
                let digit = value.to_digit(10).unwrap() as usize;

                if first_value {
                    snailfish.a_literal = digit;
                } else {
                    snailfish.b_literal = digit;
                }
            }
            None => break,
        }
    }
    snailfish
}

fn parse_string(input: &str) -> SnailFish {
    let it = &mut input.chars();
    parse_snailfish(it, true)
}

fn solve_part1(inputfile: String) -> usize {
    std::fs::read_to_string(inputfile)
        .expect("Something went wrong reading the file")
        .lines()
        .map(|input| parse_string(input))
        .reduce(|sum, snailfish| sum + snailfish)
        .unwrap()
        .magnitude()
}

fn solve_part2(inputfile: String) -> usize {
    0
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
    fn test_magnitude() {
        let a = parse_string("[1,2]");
        let b = parse_string("[[3,4],5]");
        println!("a, b -> {}, {}", a, b);

        let c = a + b;
        println!("c -> {}", c.magnitude());

        assert_eq!(c.magnitude(), 143);
        let d = parse_string("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]");
        assert_eq!(d.magnitude(), 3488);
    }

    #[test]
    fn test_small_homework() {
        let sum = vec![
            "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]",
            "[[[5,[2,8]],4],[5,[[9,9],0]]]",
            "[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]",
            "[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]",
            "[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]",
            "[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]",
            "[[[[5,4],[7,7]],8],[[8,3],8]]",
            "[[9,3],[[9,9],[6,[4,9]]]]",
            "[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]",
            "[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]",
        ]
        .iter()
        .map(|input| parse_string(input))
        .reduce(|sum, snailfish| sum + snailfish)
        .unwrap();

        assert_eq!(sum.magnitude(), 4140);
    }
}
