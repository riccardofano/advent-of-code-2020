//     nw      w       sw       e       e
// (-1, -1) ( 0, -2) (+1, -1) (0,  2) (0, 2);
// (-1, -1) (-1, -3) ( 0, -4) (0, -2) (0, 0)

use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct TilePosition(isize, isize);

impl TilePosition {
    fn parse_line(line: &str) -> Self {
        let mut chars = line.chars();
        let mut position = (0, 0);

        loop {
            let Some(new_offset) = Self::match_char(&mut chars) else {
                break;
            };
            position.0 += new_offset.0;
            position.1 += new_offset.1;
        }

        Self(position.0, position.1)
    }

    fn match_char(iter: &mut impl Iterator<Item = char>) -> Option<(isize, isize)> {
        let offset = match iter.next() {
            Some('e') => (0, 2),
            Some('w') => (0, -2),
            Some('s') => (1, Self::match_char(iter)?.1 / 2),
            Some('n') => (-1, Self::match_char(iter)?.1 / 2),
            Some(x) => unreachable!("{:?}", x),
            None => return None,
        };

        Some(offset)
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut black_tiles: HashSet<TilePosition> = HashSet::new();

    input
        .lines()
        .map(TilePosition::parse_line)
        .for_each(|tile| {
            if black_tiles.take(&tile).is_none() {
                black_tiles.insert(tile);
            }
        });

    Some(black_tiles.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 24);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = "nwwswee";
        assert_eq!(TilePosition::parse_line(input), TilePosition(0, 0));
        let input = advent_of_code::read_file("examples", 24);
        assert_eq!(part_one(&input), Some(10));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 24);
        assert_eq!(part_two(&input), None);
    }
}
