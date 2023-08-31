//     nw      w       sw       e       e
// (-1, -1) (-2,  0) (-1, +1) ( 2,  0) (2, 0);
// (-1, -1) (-3, -1) (-4,  0) (-2,  0) (0, 0)

use std::collections::{HashMap, HashSet};

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
            Some('e') => (2, 0),
            Some('w') => (-2, 0),
            Some('s') => (Self::match_char(iter)?.0 / 2, 1),
            Some('n') => (Self::match_char(iter)?.0 / 2, -1),
            Some(x) => unreachable!("{:?}", x),
            None => return None,
        };

        Some(offset)
    }

    #[rustfmt::skip]
    fn neighbors(&self) -> Vec<TilePosition> {
        vec![
            TilePosition(self.0 -1, self.1 -1),
            TilePosition(self.0 + 1, self.1 -1),
            TilePosition(self.0 -2,self.1),
            TilePosition(self.0 + 2, self.1),
            TilePosition(self.0 -1,self.1 + 1),
            TilePosition(self.0 + 1, self.1 + 1),
        ]
    }
}

struct Lobby {
    black_tiles: HashSet<TilePosition>,
}

impl Lobby {
    fn new(input: &str) -> Self {
        let mut black_tiles: HashSet<TilePosition> = HashSet::new();

        input
            .lines()
            .map(TilePosition::parse_line)
            .for_each(|tile| {
                if black_tiles.take(&tile).is_none() {
                    black_tiles.insert(tile);
                }
            });

        Self { black_tiles }
    }

    fn simulate_day(&self) -> Self {
        let mut next = self.black_tiles.clone();
        let mut inactive_neighbors: HashMap<TilePosition, usize> = HashMap::new();

        for tile in &self.black_tiles {
            let mut active_neighbors = 0;
            for neighbor in tile.neighbors() {
                if self.black_tiles.contains(&neighbor) {
                    active_neighbors += 1;
                } else {
                    *inactive_neighbors.entry(neighbor).or_insert(0) += 1;
                }
            }
            if active_neighbors == 0 || active_neighbors > 2 {
                next.remove(tile);
            }
        }

        for (tile, active_neighbors) in inactive_neighbors {
            if active_neighbors == 2 {
                next.insert(tile);
            }
        }

        Self { black_tiles: next }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let lobby = Lobby::new(input);

    Some(lobby.black_tiles.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut lobby = Lobby::new(input);
    // println!("Day 0: {}", lobby.black_tiles.len());

    for _day in 1..=100 {
        lobby = lobby.simulate_day();
        // println!("Day {day}: {}", lobby.black_tiles.len());
    }

    Some(lobby.black_tiles.len())
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
        assert_eq!(part_two(&input), Some(2208));
    }
}
