use std::collections::{HashMap, HashSet};

type Point3d = (isize, isize, isize);

fn parse_starting_active_cubes(input: &str) -> HashSet<Point3d> {
    let mut set = HashSet::new();

    input.lines().enumerate().for_each(|(y, line)| {
        line.bytes().enumerate().for_each(|(x, byte)| {
            if byte == b'#' {
                set.insert((x as isize, y as isize, 0));
            }
        })
    });

    set
}

fn step_simulation(active_cubes: &HashSet<Point3d>) -> HashSet<Point3d> {
    let mut next_active_cubes = HashSet::new();

    let mut might_become_active = HashMap::new();

    for point in active_cubes {
        let mut active_neighbors = 0;
        let neighbors = get_neighbors(point);
        for neighbor in neighbors {
            if active_cubes.contains(&neighbor) {
                active_neighbors += 1;
            } else {
                *might_become_active.entry(neighbor).or_insert(0) += 1;
            }
        }

        if (2..=3).contains(&active_neighbors) {
            next_active_cubes.insert(*point);
        }
    }

    for (cube, active_neighbors) in might_become_active {
        if active_neighbors == 3 {
            next_active_cubes.insert(cube);
        }
    }

    next_active_cubes
}

fn get_neighbors((x, y, z): &Point3d) -> HashSet<Point3d> {
    let mut neighbors = HashSet::with_capacity(27);
    for delta_z in -1..=1 {
        for delta_y in -1..=1 {
            for delta_x in -1..=1 {
                neighbors.insert((x - delta_x, y - delta_y, z - delta_z));
            }
        }
    }

    neighbors.remove(&(*x, *y, *z));
    neighbors
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut active_cubes = parse_starting_active_cubes(input);

    for _ in 0..6 {
        dbg!(active_cubes.len());
        active_cubes = step_simulation(&active_cubes);
    }

    Some(active_cubes.len())
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 17);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 17);
        assert_eq!(part_one(&input), Some(112));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 17);
        assert_eq!(part_two(&input), None);
    }
}
