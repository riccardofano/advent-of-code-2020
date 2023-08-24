#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn_left(&self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }

    fn turn_right(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Action {
    Direction(Direction),
    Forward,
    Left,
    Right,
}

impl Action {
    fn parse(input: &str) -> Self {
        match input {
            "N" => Self::Direction(Direction::North),
            "E" => Self::Direction(Direction::East),
            "S" => Self::Direction(Direction::South),
            "W" => Self::Direction(Direction::West),
            "F" => Self::Forward,
            "L" => Self::Left,
            "R" => Self::Right,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Instruction {
    value: isize,
    action: Action,
}

impl Instruction {
    fn parse(input: &str) -> Self {
        let (action, value) = input.split_at(1);

        Self {
            value: value.parse().expect("Expected to find a number"),
            action: Action::parse(action),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct State {
    facing: Direction,
    vertical_distance: isize,
    horizonal_distance: isize,
}

impl State {
    fn next(&mut self, instruction: &Instruction) {
        match instruction.action {
            Action::Direction(d) => self.increase_direction(d, instruction.value),
            Action::Forward => self.increase_direction(self.facing, instruction.value),
            Action::Left => {
                for _ in 0..(instruction.value / 90) {
                    self.facing = self.facing.turn_left();
                }
            }
            Action::Right => {
                for _ in 0..(instruction.value / 90) {
                    self.facing = self.facing.turn_right();
                }
            }
        }
    }

    fn increase_direction(&mut self, direction: Direction, value: isize) {
        match direction {
            Direction::North => self.vertical_distance += value,
            Direction::South => self.vertical_distance -= value,
            Direction::East => self.horizonal_distance += value,
            Direction::West => self.horizonal_distance -= value,
        }
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            facing: Direction::East,
            vertical_distance: 0,
            horizonal_distance: 0,
        }
    }
}

pub fn part_one(input: &str) -> Option<isize> {
    let mut state = State::default();

    for line in input.lines() {
        let instruction = Instruction::parse(line);
        eprintln!("{:?} {:?}", state, instruction);
        state.next(&instruction);
    }

    dbg!(&state);
    Some(state.horizonal_distance.abs() + state.vertical_distance.abs())
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_one(&input), Some(25));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(&input), None);
    }
}
