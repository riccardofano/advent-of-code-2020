use std::collections::HashMap;

fn mask_value(mask: &str, mut value: u64) -> u64 {
    for (i, bit) in mask.bytes().rev().enumerate() {
        match bit {
            b'0' => value &= !(1 << i),
            b'1' => value |= 1 << i,
            b'X' => continue,
            _ => unreachable!(),
        }
    }

    value
}

fn apply_memory_mask(mask: &str, mut address: u64) -> (Vec<usize>, u64) {
    let mut floating_bits = Vec::new();
    for (i, bit) in mask.bytes().rev().enumerate() {
        match bit {
            b'0' => continue,
            b'1' => address |= 1 << i,
            b'X' => floating_bits.push(i),
            _ => unreachable!(),
        }
    }

    (floating_bits, address)
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut lines = input.lines();

    let mut current_mask = lines.next().unwrap().split_at(7).1;
    let mut memory: HashMap<usize, u64> = HashMap::new();

    for line in lines {
        match line.split_once(" = ").unwrap() {
            ("mask", bits) => current_mask = bits,
            (memory_address, value) => {
                // memory_address is something like `mem[1234]`
                let address = memory_address[4..memory_address.len() - 1].parse().unwrap();
                let value = mask_value(current_mask, value.parse().unwrap());
                memory.insert(address, value);
            }
        }
    }

    Some(memory.values().sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input.lines();

    let mut current_mask = lines.next().unwrap().split_at(7).1;
    let mut memory: HashMap<u64, u64> = HashMap::new();

    for line in lines {
        match line.split_once(" = ").unwrap() {
            ("mask", bits) => current_mask = bits,
            (memory_address, value) => {
                let original_address = memory_address[4..memory_address.len() - 1].parse().unwrap();
                let value = value.parse().unwrap();

                let (floating_bits, original_address) =
                    apply_memory_mask(current_mask, original_address);

                for iteration in 0..1 << floating_bits.len() {
                    let mut address = original_address;

                    for (i, bit_position) in floating_bits.iter().enumerate() {
                        match (iteration & (1 << i)) >> i {
                            0 => address &= !(1 << bit_position),
                            1 => address |= 1 << bit_position,
                            _ => unreachable!(),
                        }

                        memory.insert(address, value);
                    }
                }
            }
        }
    }

    Some(memory.values().sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 14);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_one(&input), Some(165));
    }

    #[test]
    fn test_part_two() {
        let input = include_str!("../examples/14-2.txt");
        assert_eq!(part_two(input), Some(208));
    }
}
