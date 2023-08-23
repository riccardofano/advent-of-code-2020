use std::collections::HashSet;

#[derive(Debug)]
enum InstructionKind {
    Nop,
    Acc,
    Jmp,
}

struct Instruction {
    kind: InstructionKind,
    value: isize,
}

impl Instruction {
    fn parse(line: &str) -> Self {
        let (instruction, amount) = line
            .split_once(' ')
            .expect("Could not find a space between the instruction and the amount");

        let value = amount
            .parse()
            .expect("Could not parse value of the instruction");
        let kind = match instruction {
            "nop" => InstructionKind::Nop,
            "acc" => InstructionKind::Acc,
            "jmp" => InstructionKind::Jmp,
            _ => unreachable!(),
        };

        Self { kind, value }
    }
}

struct Program {
    instructions: Vec<Instruction>,
    pointer: isize,
    accumulator: isize,
}

impl Program {
    fn parse(input: &str) -> Self {
        let instructions = input.lines().map(Instruction::parse).collect();
        Self {
            instructions,
            pointer: 0,
            accumulator: 0,
        }
    }
}

pub fn part_one(input: &str) -> Option<isize> {
    let mut program = Program::parse(input);
    let mut seen: HashSet<isize> = HashSet::default();

    while program.pointer < program.instructions.len() as isize {
        if !seen.insert(program.pointer) {
            break;
        }

        let current_instruction = &program.instructions[program.pointer as usize];
        program.pointer += 1;
        match current_instruction.kind {
            InstructionKind::Nop => {}
            InstructionKind::Acc => program.accumulator += current_instruction.value,
            InstructionKind::Jmp => program.pointer += current_instruction.value - 1,
        };
    }

    Some(program.accumulator)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(5));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), None);
    }
}
