use std::{num::ParseIntError, str::FromStr};

use crate::input::get_lines;

pub fn run() {
    let program = get_lines("day23");
    let mut computer = Computer::new(&program);

    computer.run();

    println!("Register b: {}", computer.b);
}

#[derive(Clone, PartialEq, Eq)]
enum Instruction {
    Half(String),
    Triple(String),
    Increment(String),
    Jump(isize),
    JumpIfEven(String, isize),
    JumpIfOne(String, isize),
}

#[derive(Debug)]
enum ParseInstructionError {
    ParseIntError,
    UnknownInstruction,
}

impl FromStr for Instruction {
    type Err = ParseInstructionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (instr, args) = s.split_at(3);

        match instr {
            "hlf" => Ok(Instruction::Half(args.trim().to_string())),
            "tpl" => Ok(Instruction::Triple(args.trim().to_string())),
            "inc" => Ok(Instruction::Increment(args.trim().to_string())),
            "jmp" => {
                if let Ok(offset) = isize::from_str(args.trim()) {
                    Ok(Instruction::Jump(offset))
                } else {
                    Err(ParseInstructionError::ParseIntError)
                }
            }
            "jie" => {
                if let Ok((register, offset)) = parse_register_offset(args) {
                    Ok(Instruction::JumpIfEven(register, offset))
                } else {
                    Err(ParseInstructionError::ParseIntError)
                }
            }
            "jio" => {
                if let Ok((register, offset)) = parse_register_offset(args) {
                    Ok(Instruction::JumpIfOne(register, offset))
                } else {
                    Err(ParseInstructionError::ParseIntError)
                }
            }
            _ => Err(ParseInstructionError::UnknownInstruction),
        }
    }
}

fn parse_register_offset(s: &str) -> Result<(String, isize), ParseIntError> {
    let mut split = s.split(',');
    let register = split.next().unwrap().trim();
    let offset_str = split.next().unwrap().trim();
    let offset = isize::from_str(offset_str)?;
    Ok((register.to_string(), offset))
}

struct Computer {
    a: isize,
    b: isize,
    cursor: usize,
    program: Vec<Instruction>,
}

impl Computer {
    fn new(lines: &[String]) -> Self {
        let program = lines
            .iter()
            .map(|line| Instruction::from_str(line).expect(&format!("couldn't parse {}", line)))
            .collect();
        Self {
            a: 0,
            b: 0,
            cursor: 0,
            program,
        }
    }

    fn run(&mut self) {
        while self.cursor < self.program.len() {
            let instruction = self.program[self.cursor].clone();
            match instruction {
                Instruction::Half(r) => self.register_operation(&r, 2, &|x, y| x / y),
                Instruction::Triple(r) => self.register_operation(&r, 3, &|x, y| x * y),
                Instruction::Increment(r) => self.register_operation(&r, 1, &|x, y| x + y),
                Instruction::Jump(offset) => {
                    self.cursor = (self.cursor as isize + offset) as usize;
                }
                Instruction::JumpIfEven(r, offset) => {
                    self.conditional_jump(&r, offset, &|x| x % 2 == 0)
                }
                Instruction::JumpIfOne(r, offset) => self.conditional_jump(&r, offset, &|x| x == 1),
            }
        }
    }

    fn register_operation(&mut self, r: &str, val: isize, op: &dyn Fn(isize, isize) -> isize) {
        match r {
            "a" => self.a = op(self.a, val),
            "b" => self.b = op(self.b, val),
            _ => (),
        }
        self.cursor += 1;
    }

    fn conditional_jump(&mut self, r: &str, offset: isize, condition: &dyn Fn(isize) -> bool) {
        match r {
            "a" => {
                if condition(self.a) {
                    self.cursor = (self.cursor as isize + offset) as usize;
                } else {
                    self.cursor += 1;
                }
            }
            "b" => {
                if condition(self.b) {
                    self.cursor = (self.cursor as isize + offset) as usize;
                } else {
                    self.cursor += 1;
                }
            }
            _ => self.cursor += 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let program = [
            "inc a".to_string(),
            "jio a, +2".to_string(),
            "tpl a".to_string(),
            "inc a".to_string(),
        ];
        let mut computer = Computer::new(&program);

        computer.run();

        assert_eq!(computer.a, 2);
    }
}
