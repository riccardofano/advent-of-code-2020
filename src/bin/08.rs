use std::collections::HashSet;

#[derive(Debug)]
enum InstructionKind {
    Nop,
    Acc(isize),
    Jmp(isize),
}

impl InstructionKind {
    fn parse(line: &str) -> Self {
        let (instruction, amount) = line
            .split_once(' ')
            .expect("Could not find a space between the instruction and the amount");

        match instruction {
            "nop" => Self::Nop,
            "acc" => Self::Acc(Self::parse_amount(amount)),
            "jmp" => Self::Jmp(Self::parse_amount(amount)),
            _ => unreachable!(),
        }
    }

    fn parse_amount(amount: &str) -> isize {
        let (sign, number) = amount.split_at(1);
        let number: isize = number.parse().expect("Expected a valid number");
        match sign {
            "+" => number,
            "-" => -number,
            _ => unreachable!(),
        }
    }
}

pub fn part_one(input: &str) -> Option<isize> {
    let lines: Vec<_> = input.lines().collect();
    let mut seen: HashSet<isize> = HashSet::with_capacity(lines.len());
    let mut accumulator = 0;
    let mut i = 0;

    loop {
        if seen.contains(&i) {
            break;
        };
        seen.insert(i);

        match InstructionKind::parse(lines[i as usize]) {
            InstructionKind::Nop => {}
            InstructionKind::Acc(x) => accumulator += x,
            InstructionKind::Jmp(x) => {
                i += x;
                continue;
            }
        }

        i += 1;
    }

    Some(accumulator)
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
