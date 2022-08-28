use std::collections::HashSet;

pub fn part_1(input: &str) -> i32 {
    let program: Vec<Instruction> = input.lines().map(Instruction::from_str).collect();
    let (final_value, _terminated) = execute_program(&program);
    final_value
}

pub fn part_2(input: &str) -> i32 {
    let mut program: Vec<_> = input.lines().map(Instruction::from_str).collect();
    let locations = get_locations(&program);
    for i in locations {
        {
            let nop_or_jmp = &mut program[i];
            nop_or_jmp.flip_op();
        }
        let (value, terminated) = execute_program(&program);
        if terminated {
            return value;
        }
        {
            let nop_or_jmp = &mut program[i];
            nop_or_jmp.flip_op();
        }
    }
    panic!("Failed to find terminating program");
}

fn execute_program(program: &[Instruction]) -> (i32, bool) {
    let mut pc: usize = 0;
    let mut acc: i32 = 0;
    let mut executed: HashSet<usize> = HashSet::new();
    while executed.insert(pc) {
        let inst = &program[pc];
        inst.execute(&mut pc, &mut acc);
        if pc == program.len() {
            return (acc, true);
        }
    }
    (acc, false)
}

fn get_locations(program: &[Instruction]) -> Vec<usize> {
    let mut result = Vec::new();
    let mut offset = 0;
    let mut iter = program.iter();
    while let Some(i) = iter.position(|i| i.op != Op::Acc) {
        result.push(i + offset);
        offset += i + 1;
    }
    result
}

#[derive(PartialEq)]
enum Op {
    Nop,
    Acc,
    Jmp,
}

impl Op {
    fn from_str(s: &str) -> Op {
        match s {
            "nop" => Op::Nop,
            "acc" => Op::Acc,
            "jmp" => Op::Jmp,
            _ => panic!("Unsupported operation"),
        }
    }
}

struct Instruction {
    op: Op,
    arg: i32,
}

impl Instruction {
    fn from_str(line: &str) -> Instruction {
        let mut split_iter = line.split(' ');
        let op = Op::from_str(split_iter.next().unwrap());
        let arg = split_iter.next().unwrap().parse().unwrap();
        Instruction { op, arg }
    }

    fn execute(&self, pc: &mut usize, acc: &mut i32) {
        match self.op {
            Op::Nop => {
                *pc += 1;
            }
            Op::Acc => {
                *acc += self.arg;
                *pc += 1;
            }
            Op::Jmp => {
                *pc = pc.wrapping_add(self.arg as usize);
            }
        }
    }

    fn flip_op(&mut self) {
        match self.op {
            Op::Nop => {
                self.op = Op::Jmp;
            }
            Op::Jmp => {
                self.op = Op::Nop;
            }
            Op::Acc => (),
        }
    }
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "./input/2020/day_8.txt";
    const SAMPLE: &str = "./input/2020/day_8_sample.txt";
    const PART_1_SAMPLE: i32 = 5;
    const PART_1: i32 = 1451;
    const PART_2_SAMPLE: i32 = 8;
    const PART_2: i32 = 1160;

    use std::fs;

    #[test]
    fn part_1_sample() {
        let input = fs::read_to_string(SAMPLE).unwrap();
        assert_eq!(PART_1_SAMPLE, super::part_1(&input));
    }

    #[test]
    fn part_1() {
        let input = fs::read_to_string(INPUT).unwrap();
        assert_eq!(PART_1, super::part_1(&input));
    }

    #[test]
    fn part_2_sample() {
        let input = fs::read_to_string(SAMPLE).unwrap();
        assert_eq!(PART_2_SAMPLE, super::part_2(&input));
    }

    #[test]
    fn part_2() {
        let input = fs::read_to_string(INPUT).unwrap();
        assert_eq!(PART_2, super::part_2(&input));
    }
}
