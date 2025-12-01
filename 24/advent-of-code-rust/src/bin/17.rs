#![feature(never_type)]

use std::str::FromStr;
use itertools::Itertools;

advent_of_code::solution!(17);

#[derive(Debug, Clone, Copy)]
struct Instruction {
    opcode: Opcode,
    operand: U3,
}

#[derive(Debug, Clone, Copy)]
enum Opcode {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl From<U3> for Opcode {
    fn from(u: U3) -> Self {
        match u.0 {
            0 => Self::Adv,
            1 => Self::Bxl,
            2 => Self::Bst,
            3 => Self::Jnz,
            4 => Self::Bxc,
            5 => Self::Out,
            6 => Self::Bdv,
            7 => Self::Cdv,
            _ => unreachable!(),
        }
    }
}

impl From<Opcode> for U3 {
    fn from(opcode: Opcode) -> Self {
        Self(match opcode {
            Opcode::Adv => 0,
            Opcode::Bxl => 1,
            Opcode::Bst => 2,
            Opcode::Jnz => 3,
            Opcode::Bxc => 4,
            Opcode::Out => 5,
            Opcode::Bdv => 6,
            Opcode::Cdv => 7,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct U3(u8);

impl TryFrom<u8> for U3 {
    type Error = ();

    fn try_from(i: u8) -> Result<Self, Self::Error> {
        match i {
            0..8 => Ok(U3(i)),
            _ => Err(()),
        }
    }
}

impl TryFrom<u128> for U3 {
    type Error = ();

    fn try_from(i: u128) -> Result<Self, Self::Error> {
        (i as u8).try_into()
    }
}

impl From<U3> for u8 {
    fn from(value: U3) -> Self {
        value.0
    }
}

impl From<U3> for u128 {
    fn from(value: U3) -> Self {
        value.0.into()
    }
}

impl FromStr for U3 {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<u8>().unwrap().try_into()
    }
}

#[derive(Debug, Clone)]
struct State {
    ip: usize,
    instructions: Vec<Instruction>,
    a: u128,
    b: u128,
    c: u128,
    output: Vec<U3>,
}

impl State {
    // Returns true if should increment ip
    fn do_instruction(&mut self, instruction: &Instruction) -> bool {
        match instruction.opcode {
            Opcode::Adv | Opcode::Bdv | Opcode::Cdv => {
                let numerator = self.a;
                let denominator = 2_u128.pow(self.combo(&instruction.operand) as u32);
                let result = numerator / denominator;
                match instruction.opcode {
                    Opcode::Adv => self.a = result,
                    Opcode::Bdv => self.b = result,
                    Opcode::Cdv => self.c = result,
                    _ => unreachable!(),
                }
            }
            Opcode::Bxl => {
                self.b = self.b ^ self.literal(&instruction.operand);
            }
            Opcode::Bst => {
                self.b = self.combo(&instruction.operand) % 8;
            }
            Opcode::Jnz => {
                if self.a != 0 {
                    self.ip = (self.literal(&instruction.operand) / 2) as usize;
                    return false;
                }
            }
            Opcode::Bxc => {
                self.b = self.b ^ self.c;
            }
            Opcode::Out => {
                self.output.push((self.combo(&instruction.operand) % 8).try_into().unwrap());
            }
        }
        true
    }

    fn literal(&self, operand: &U3) -> u128 {
        operand.0.try_into().unwrap()
    }

    fn combo(&self, operand: &U3) -> u128 {
        match operand.0 {
            0..=3 => operand.0.try_into().unwrap(),
            4 => self.a,
            5 => self.b,
            6 => self.c,
            7 => panic!("Combo operand 7"),
            _ => unreachable!(),
        }
    }

    // Returns true if finished successfully
    fn run(&mut self) -> bool {
        let mut had = vec![];
        while self.ip < self.instructions.len() {
            had.push((self.ip, self.a, self.b, self.c));
            let instruction = self.instructions[self.ip].clone();
            // println!("{}: {:?}", self.ip, instruction);
            let orig_ip = self.ip;
            let increment_ip = self.do_instruction(&instruction);
            println!("ip: {}, a: {}, b: {}, c: {}", orig_ip * 2, self.a, self.b, self.c);
            if increment_ip {
                self.ip += 1;
            }
            // i += 1;
            // if i > timeout {
            //     return false;
            // }
            if had.contains(&(self.ip, self.a, self.b, self.c)) {
                return false;
            }
        }
        true
    }

    // Returns true if finished successfully
    fn run_expect(&mut self, expected: &Vec<U3>) -> bool {
        let mut had = vec![];
        while self.ip < self.instructions.len() {
            had.push((self.ip, self.a, self.b, self.c));
            let instruction = self.instructions[self.ip].clone();
            // println!("{}: {:?}", self.ip, instruction);
            let orig_ip = self.ip;
            let increment_ip = self.do_instruction(&instruction);
            // println!("ip: {}, a: {}, b: {}, c: {}", orig_ip * 2, self.a, self.b, self.c);
            if increment_ip {
                self.ip += 1;
            }
            // i += 1;
            // if i > timeout {
            //     return false;
            // }
            if had.contains(&(self.ip, self.a, self.b, self.c)) {
                return false;
            }
            if self.output.len() > expected.len() || self.output != expected[0..self.output.len()] {
                // println!("Hey");
                // if self.output != expected[0..self.output.len()] {
                //     println!("  Because of the thingy")
                // }
                // println!("{} is how far I came", self.output.len());
                return false;
            }
        }
        // println!("Nice");
        true
    }
}

impl FromStr for State {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (registers, program) = s.split_once("\n\n").unwrap();

        let (a, b, c) = registers.split("\n").collect_tuple().unwrap();
        let a = a.chars().filter(|x| x.is_digit(10)).collect::<String>().parse().unwrap();
        let b = b.chars().filter(|x| x.is_digit(10)).collect::<String>().parse().unwrap();
        let c = c.chars().filter(|x| x.is_digit(10)).collect::<String>().parse().unwrap();

        let (_, program) = program.split_once(": ").unwrap();
        let instructions = program
            .split(",")
            .map(|x| x.parse::<U3>().unwrap())
            .chunks(2)
            .into_iter()
            .map(|x| x.collect_tuple().unwrap())
            .map(|(opcode, operand)| Instruction { opcode: opcode.into(), operand: operand.into() })
            .collect();

        Ok(Self {
            ip: 0,
            a,
            b,
            c,
            instructions,
            output: vec![],
        })
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let mut state: State = input.parse().unwrap();
    state.run();
    Some(state.output.into_iter().map(|x| x.0).join(","))
}

fn check(orig_a: u128, program: &Vec<U3>) -> bool {
    let mut a = orig_a;
    for expected in program {
        if a == 0 {
            // println!("Finished early");
            return false;
        }
        let mut b = a % 8;
        b ^= 1;
        let c = a >> b;
        b ^= 4;
        a >>= 3;
        b ^= c;
        let output = b % 8;
        // if orig_a % 2_u128.pow(16) == 554 {
        //     println!("{orig_a}:  {output}");
        // }
        if output != u128::from(*expected) {
            return false;
        }
    }
    a == 0
}

pub fn part_two(input: &str) -> Option<u128> {
    let orig_state: State = input.parse().unwrap();
    let program: Vec<U3> = orig_state.instructions
        .iter()
        .flat_map(|x| [x.opcode.into(), x.operand])
        .collect();
    let thingies: Vec<u128> = include_str!("17_helper.txt").split("\n").map(|x| x.parse().unwrap()).collect();
    let thingies_len = u128::try_from(thingies.len()).unwrap();
    let factor = 2_u128.pow(16);
    // `140500000000` is a binary-search-found number that's just before `a >> (3 * 15) == 0`
    for i in 140500000000..u128::MAX {
    // for i in 0..10000 {
        if i % 10000000 == 0 {
            println!("i: {i}");
        }
        // 35000000000000..290000000000000
        let large_blah = i / thingies_len;
        let a = (factor * large_blah) + thingies[usize::try_from(i % thingies_len).unwrap()];
        // if a < 281474976710656 {
        //     continue;
        // }
        if a >> (3 * 15) == 0 {
            // println!("Too small");
            continue;
        }
        if a >> (3 * 16) != 0 {
            println!("{a} is too big!");
            break;
        }
        // println!("{}", a % 2_u128.pow(16));
        // let a = i;
        if large_blah % 100000 == 0 && i % thingies_len == 0 {
            println!("a: {a}");
        }
        if check(a, &program) {
            return Some(a);
        }
        // let mut state = orig_state.clone();
        // state.a = i;
        // if state.run_expect(&program) {
        //     // println!("Successful: {:?} == {:?}?", state.output, program);
        //     if state.output == program {
        //         return Some(i);
        //     }
        // }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("4,6,3,5,6,3,5,2,1,0".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(117440));
    }
}
