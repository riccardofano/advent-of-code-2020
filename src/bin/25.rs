const SUBJECT_NUMBER: usize = 7;

fn perform_handshake(number: &mut usize, subject_number: usize) {
    *number *= subject_number;
    *number %= 20201227;
}

fn find_public_key(public_key: usize) -> usize {
    let mut current = SUBJECT_NUMBER;

    for loop_size in 1.. {
        if current == public_key {
            return loop_size;
        }
        perform_handshake(&mut current, SUBJECT_NUMBER);
    }
    unreachable!()
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut lines = input.lines();
    let card_public: usize = lines.next().unwrap().parse().unwrap();
    let door_public: usize = lines.next().unwrap().parse().unwrap();

    let card_loop_size = find_public_key(card_public);
    let _door_loop_size = find_public_key(door_public);

    let mut encryption_key = door_public;
    for _ in 1..card_loop_size {
        perform_handshake(&mut encryption_key, door_public);
    }

    Some(encryption_key)
}

pub fn part_two(input: &str) -> Option<usize> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 25);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 25);
        assert_eq!(part_one(&input), Some(14897079));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 25);
        assert_eq!(part_two(&input), None);
    }
}
