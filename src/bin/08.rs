use std::collections::HashSet;

#[derive(Debug, Clone, Copy)]
enum InstructionKind {
    Nop,
    Acc,
    Jmp,
}

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone)]
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

    fn run_to_end(&mut self) -> Option<isize> {
        let mut seen: HashSet<isize> = HashSet::default();

        while self.pointer < self.instructions.len() as isize {
            if !seen.insert(self.pointer) {
                return None;
            }

            let current_instruction = &self.instructions[self.pointer as usize];
            self.pointer += 1;
            match current_instruction.kind {
                InstructionKind::Nop => {}
                InstructionKind::Acc => self.accumulator += current_instruction.value,
                InstructionKind::Jmp => self.pointer += current_instruction.value - 1,
            };
        }

        Some(self.accumulator)
    }
}

pub fn part_one(input: &str) -> Option<isize> {
    let mut program = Program::parse(input);
    program.run_to_end();
    Some(program.accumulator)
}

pub fn part_two(input: &str) -> Option<isize> {
    let program = Program::parse(input);

    (0..program.instructions.len()).find_map(|i| {
        let mut test_program = program.clone();

        let new_instruction = match test_program.instructions[i].kind {
            InstructionKind::Acc => return None,
            InstructionKind::Nop => InstructionKind::Jmp,
            InstructionKind::Jmp => InstructionKind::Nop,
        };
        test_program.instructions[i].kind = new_instruction;
        test_program.run_to_end()
    })
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
        assert_eq!(part_two(&input), Some(8));
    }
}
