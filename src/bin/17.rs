use rayon::prelude::*;
use std::{fmt::Display, ops::Rem};

use itertools::Itertools;

advent_of_code::solution!(17);

#[derive(PartialEq, Clone, Debug, Eq, Hash, PartialOrd, Ord)]
struct StrangeDevice {
    instruction_pointer: u8,
    register_a: u64,
    register_b: u64,
    register_c: u64,
    // So, the program 0,1,2,3 would run the instruction whose opcode is 0 and pass it the operand 1, then run the instruction having opcode 2 and pass it the operand 3, then halt.
    program: Vec<u8>,
    output: Vec<u64>,
}

impl Display for StrangeDevice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl StrangeDevice {
    fn step(&mut self) -> Option<()> {
        let (instruction_opcode, literal_operand) = (
            *self.program.get(self.instruction_pointer as usize)?,
            *self.program.get((self.instruction_pointer + 1) as usize)?,
        );

        let combo_operand = match literal_operand {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => self.register_a,
            5 => self.register_b,
            6 => self.register_c,
            7 => {
                // panic!("unexpected operand");
                7
            }
            _ => panic!("out of range for 3bit number"),
        };

        // dbg!((
        //     self.instruction_pointer,
        //     instruction_opcode,
        //     literal_operand,
        //     combo_operand
        // ));

        match instruction_opcode {
            0 => {
                // dbg!("operation 0");
                let numerator = self.register_a;
                let denominator = 2u64.pow(combo_operand.try_into().unwrap());
                self.register_a = numerator / denominator;
            }
            1 => {
                // dbg!("operation 1");
                // println!("{:#010b}", literal_operand);
                // println!("{:#010b}", self.register_b);
                // dbg!(
                //     self.register_b,
                //     literal_operand,
                //     self.register_b ^ literal_operand as u64
                // );
                self.register_b ^= literal_operand as u64;
            }
            2 => {
                // dbg!("operation 2");
                self.register_b = combo_operand.rem_euclid(8);
            }
            3 => {
                // dbg!("operation 3");
                if self.register_a != 0 {
                    self.instruction_pointer = literal_operand;
                }
            }
            4 => {
                // dbg!("operation 4");
                self.register_b ^= self.register_c;
            }
            5 => {
                // dbg!("operation 5");
                let value = combo_operand.rem_euclid(8);
                self.output.push(value);
            }
            6 => {
                // dbg!("operation 6");
                let numerator = self.register_a;
                let denominator = 2u64.pow(combo_operand.try_into().unwrap());
                self.register_b = numerator / denominator;
            }
            7 => {
                // dbg!("operation 7");
                let numerator = self.register_a;
                let denominator = 2u64.pow(combo_operand.try_into().unwrap());
                self.register_c = numerator / denominator;
            }
            _ => panic!("unexpected opcode"),
        }

        // dbg!(instruction_opcode, self.register_a);

        // increase instruction pointer, except if opcode was 3 with register a at 0
        match (instruction_opcode, self.register_a) {
            (3, register_a) if register_a != 0 => {
                // dbg!("skippin increase");
            }
            _ => {
                self.instruction_pointer += 2;
            }
        }

        Some(())
    }

    fn complete(&mut self) -> String {
        loop {
            if self.step().is_none() {
                break;
            }
        }

        self.output
            .iter()
            .map(|out| out.to_string())
            .collect_vec()
            .join(",")
    }
}

fn parse(input: &str) -> StrangeDevice {
    let mut lines = input.lines();

    let mut get_next_register = || -> u64 {
        lines
            .next()
            .unwrap()
            .split_once(": ")
            .unwrap()
            .1
            .parse()
            .unwrap()
    };

    let register_a = get_next_register();
    let register_b = get_next_register();
    let register_c = get_next_register();

    let program = input
        .split_once("\n\n")
        .unwrap()
        .1
        .split_once("Program: ")
        .unwrap()
        .1
        .split(",")
        .map(|op| op.trim().parse().unwrap())
        .collect_vec();

    StrangeDevice {
        instruction_pointer: 0,
        register_a,
        register_b,
        register_c,
        program,
        output: vec![],
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let mut strange_device = parse(input);

    let output = strange_device.complete();

    Some(output)
}

pub fn part_two(input: &str) -> Option<u64> {
    let strange_device = parse(input);
    let program = strange_device
        .program
        .iter()
        .map(|out| out.to_string())
        .collect_vec()
        .join(",");

    let result = (0..u64::MAX).into_par_iter().find_first(|i| {
        if i.rem(1000u64) == 0 {
            println!("running: {i}");
        }
        // for i in 0..=5 {
        let mut copied_strange_device = strange_device.clone();
        copied_strange_device.register_a = *i;
        let output = copied_strange_device.complete();

        program == output
    });
    result
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
    fn test_part_example_1() {
        let mut strange_device = StrangeDevice {
            register_a: 0,
            register_b: 0,
            register_c: 9,
            instruction_pointer: 0,
            program: vec![2, 6],
            output: vec![],
        };
        strange_device.complete();
        assert_eq!(strange_device.register_b, 1);
    }

    #[test]
    fn test_part_example_2() {
        let mut strange_device = StrangeDevice {
            register_a: 0,
            register_b: 29,
            register_c: 0,
            instruction_pointer: 0,
            program: vec![1, 7],
            output: vec![],
        };
        strange_device.complete();
        assert_eq!(strange_device.register_b, 26);
    }

    #[test]
    fn test_part_example_3() {
        let mut strange_device = StrangeDevice {
            register_a: 0,
            register_b: 2024,
            register_c: 43690,
            instruction_pointer: 0,
            program: vec![4, 0],
            output: vec![],
        };
        strange_device.complete();
        assert_eq!(strange_device.register_b, 44354);
    }

    #[test]
    fn test_part_example_4() {
        let mut strange_device = StrangeDevice {
            register_a: 2024,
            register_b: 0,
            register_c: 0,
            instruction_pointer: 0,
            program: vec![0, 1, 5, 4, 3, 0],
            output: vec![],
        };
        strange_device.complete();
        assert_eq!(strange_device.register_a, 0);
    }

    #[test]
    fn test_part_example_5() {
        let mut strange_device = StrangeDevice {
            register_a: 10,
            register_b: 10,
            register_c: 10,
            instruction_pointer: 0,
            program: vec![5, 1],
            output: vec![],
        };
        strange_device.complete();
        assert_eq!(strange_device.output, vec![1]);
    }

    #[test]
    fn test_part_example_6() {
        let mut strange_device = StrangeDevice {
            register_a: 10,
            register_b: 0,
            register_c: 0,
            instruction_pointer: 0,
            program: vec![5, 0, 5, 1, 5, 4],
            output: vec![],
        };
        strange_device.complete();
        assert_eq!(strange_device.output, vec![0, 1, 2]);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(117440));
    }
}
