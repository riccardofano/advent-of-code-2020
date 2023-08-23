pub fn part_one(input: &str) -> Option<usize> {
    input
        .lines()
        .map(|passport| {
            let mut top = 0;
            let mut bottom = 127;

            let mut left = 0;
            let mut right = 7;

            for letter in passport.bytes() {
                match letter {
                    b'F' => bottom = top + (bottom - top) / 2,
                    b'B' => top = top + (bottom - top) / 2,
                    b'L' => right = left + (right - left) / 2,
                    b'R' => left = left + (right - left) / 2,
                    _ => break,
                }
            }

            let correct_row = match passport.as_bytes()[6] {
                b'F' => top + 1,
                b'B' => bottom,
                x => unreachable!("{:?}", x),
            };

            let correct_col = match passport.as_bytes()[9] {
                b'L' => left + 1,
                b'R' => right,
                x => unreachable!("{:?}", x),
            };

            correct_row * 8 + correct_col
        })
        .max()
}

pub fn part_two(input: &str) -> Option<u32> {
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
        assert_eq!(part_two(&input), None);
    }
}
