fn find_seat_id(passport: &str) -> usize {
    let mut top = 0;
    let mut bottom = 127;

    let mut left = 0;
    let mut right = 7;

    for letter in passport.bytes() {
        match letter {
            b'F' => bottom = top + (bottom - top) / 2,
            b'B' => top = top + (bottom - top) / 2 + 1,
            b'L' => right = left + (right - left) / 2,
            b'R' => left = left + (right - left) / 2 + 1,
            _ => break,
        }
    }

    let correct_row = match passport.as_bytes()[6] {
        b'F' => top,
        b'B' => bottom,
        x => unreachable!("{:?}", x),
    };

    let correct_col = match passport.as_bytes()[9] {
        b'L' => left,
        b'R' => right,
        x => unreachable!("{:?}", x),
    };

    correct_row * 8 + correct_col
}

pub fn part_one(input: &str) -> Option<usize> {
    input.lines().map(find_seat_id).max()
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut seat_ids: Vec<_> = input.lines().map(find_seat_id).collect();
    seat_ids.sort();

    for (i, seat) in seat_ids.iter().enumerate().skip(1) {
        if *seat != seat_ids[i - 1] + 1 {
            return Some(seat - 1);
        }
    }

    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some(820));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some(69));
    }
}
