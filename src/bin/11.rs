use std::fmt::Debug;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Seat {
    Floor,
    Empty,
    Occupied,
}

impl Seat {
    fn parse(seat: u8) -> Self {
        match seat {
            b'.' => Self::Floor,
            b'L' => Self::Empty,
            b'#' => Self::Occupied,
            _ => unreachable!(),
        }
    }

    fn next_state(&self, layout: &Layout, position: usize) -> Self {
        if *self == Seat::Floor {
            return *self;
        }

        let x = (position % layout.width) as isize;
        let y = (position / layout.width) as isize;

        let occupied_seats = layout
            .all_neightbors((x, y))
            .into_iter()
            .filter(|point| {
                if !layout.is_in_bounds(*point) {
                    return false;
                }

                let seat = layout.seats[layout.point_to_index(*point)];
                seat == Seat::Occupied
            })
            .count();

        match (self, occupied_seats) {
            (Seat::Empty, 0) => Seat::Occupied,
            (Seat::Occupied, n) if n >= 4 => Seat::Empty,
            _ => *self,
        }
    }

    fn next_state_two(&self, layout: &Layout, position: usize) -> Self {
        if *self == Seat::Floor {
            return *self;
        }

        let x = (position % layout.width) as isize;
        let y = (position / layout.width) as isize;

        let occupied_seats = layout
            .all_directions()
            .into_iter()
            .filter(|&direction| {
                if let Some(seat) = layout.find_first_seat((x, y), direction) {
                    seat == Seat::Occupied
                } else {
                    false
                }
            })
            .count();

        match (self, occupied_seats) {
            (Seat::Empty, 0) => Seat::Occupied,
            (Seat::Occupied, n) if n >= 5 => Seat::Empty,
            _ => *self,
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
struct Layout {
    seats: Vec<Seat>,
    width: usize,
    height: usize,
}

impl Layout {
    fn parse(input: &str) -> Self {
        let lines: Vec<_> = input.lines().collect();
        let height = lines.len();
        let width = lines[0].len();
        let mut seats = Vec::with_capacity(height * width);

        for line in lines {
            for byte in line.as_bytes() {
                seats.push(Seat::parse(*byte))
            }
        }

        Self {
            seats,
            width,
            height,
        }
    }

    fn next_state(&mut self) -> bool {
        let mut has_changed = false;
        let mut next_seats = Vec::with_capacity(self.seats.len());

        for (i, seat) in self.seats.iter().enumerate() {
            let new_seat = seat.next_state(self, i);
            next_seats.push(new_seat);
            if *seat != new_seat {
                has_changed = true;
            }
        }

        self.seats = next_seats;
        has_changed
    }

    fn next_state_two(&mut self) -> bool {
        let mut has_changed = false;
        let mut next_seats = Vec::with_capacity(self.seats.len());

        for (i, seat) in self.seats.iter().enumerate() {
            let new_seat = seat.next_state_two(self, i);
            next_seats.push(new_seat);
            if *seat != new_seat {
                has_changed = true;
            }
        }

        self.seats = next_seats;
        has_changed
    }

    fn is_in_bounds(&self, (x, y): (isize, isize)) -> bool {
        x >= 0 && x < self.width as isize && y >= 0 && y < self.height as isize
    }

    fn point_to_index(&self, (x, y): (isize, isize)) -> usize {
        (y * self.width as isize + x) as usize
    }

    fn find_first_seat(
        &self,
        from_point: (isize, isize),
        direction: (isize, isize),
    ) -> Option<Seat> {
        let mut i = 1;
        loop {
            let point_to_check = (
                from_point.0 + (i * direction.0),
                from_point.1 + (i * direction.1),
            );

            if !self.is_in_bounds(point_to_check) {
                return None;
            }

            let seat = self.seats[self.point_to_index(point_to_check)];
            if seat != Seat::Floor {
                return Some(seat);
            }
            i += 1;
        }
    }

    fn all_neightbors(&self, (x, y): (isize, isize)) -> [(isize, isize); 8] {
        [
            (x - 1, y - 1),
            (x, y - 1),
            (x + 1, y - 1),
            (x - 1, y),
            (x + 1, y),
            (x - 1, y + 1),
            (x, y + 1),
            (x + 1, y + 1),
        ]
    }

    fn all_directions(&self) -> [(isize, isize); 8] {
        [
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ]
    }
}

impl Debug for Layout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, seat) in self.seats.iter().enumerate() {
            let symbol = match seat {
                Seat::Floor => ".",
                Seat::Empty => "L",
                Seat::Occupied => "#",
            };

            if i % self.width == 0 {
                writeln!(f)?;
            }
            write!(f, "{}", symbol)?;
        }
        Ok(())
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut layout = Layout::parse(input.trim());

    while layout.next_state() {
        // dbg!(&layout);
    }

    let occupied_seats = layout
        .seats
        .into_iter()
        .filter(|s| *s == Seat::Occupied)
        .count();

    Some(occupied_seats)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut layout = Layout::parse(input.trim());

    while layout.next_state_two() {
        // dbg!(&layout);
    }

    let occupied_seats = layout
        .seats
        .into_iter()
        .filter(|s| *s == Seat::Occupied)
        .count();

    Some(occupied_seats)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), Some(37));
    }

    #[test]
    fn test_part_two() {
        //         let input = r#".......#.
        // ...#.....
        // .#.......
        // .........
        // ..#L....#
        // ....#....
        // .........
        // #........
        // ...#....."#;
        //         assert_eq!(part_two(input), None);
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), Some(26));
    }
}
