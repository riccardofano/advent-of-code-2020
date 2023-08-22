struct Password {
    password: String,
    min: usize,
    max: usize,
    letter: char,
}

impl Password {
    fn from(line: &str) -> Self {
        let (min, rest) = line.split_once('-').unwrap();
        let (max, rest) = rest.split_once(' ').unwrap();
        let (letter, password) = rest.split_once(": ").unwrap();

        let min = min.parse::<usize>().unwrap();
        let max = max.parse::<usize>().unwrap();
        let letter = letter.parse::<char>().unwrap();

        Self {
            password: password.to_string(),
            min,
            max,
            letter,
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let valid_passwords = input
        .lines()
        .map(Password::from)
        .filter(|p| {
            let mut letter_count = 0;
            for l in p.password.chars() {
                if l == p.letter {
                    letter_count += 1;
                }
            }
            (p.min..=p.max).contains(&letter_count)
        })
        .count();

    Some(valid_passwords)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), None);
    }
}
