enum Bus {
    X,
    Id(usize),
}

impl Bus {
    fn parse(input: &str) -> Self {
        match input {
            "x" => Bus::X,
            n => match n.parse::<usize>() {
                Ok(id) => Bus::Id(id),
                Err(_) => unreachable!(),
            },
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut lines = input.lines();
    let original_timestamp: usize = lines.next().unwrap().parse().unwrap();
    let buses: Vec<usize> = lines
        .next()
        .unwrap()
        .split(',')
        .map(Bus::parse)
        .filter_map(|b| match b {
            Bus::X => None,
            Bus::Id(id) => Some(id),
        })
        .collect();

    let mut current_timestamp = original_timestamp;
    loop {
        for id in &buses {
            if current_timestamp % id == 0 {
                return Some((current_timestamp - original_timestamp) * id);
            }
        }
        current_timestamp += 1;
    }
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut lines = input.lines();
    let _original_timestamp: usize = lines.next().unwrap().parse().unwrap();
    let buses: Vec<Bus> = lines.next().unwrap().split(',').map(Bus::parse).collect();

    let mut current_timestamp = 1;
    let mut wait_time = 1;

    for (i, bus) in buses.iter().enumerate() {
        let Bus::Id(id) = bus else { continue };
        loop {
            if (current_timestamp + i) % id == 0 {
                wait_time *= id;
                break;
            }
            current_timestamp += wait_time;
        }
    }

    Some(current_timestamp)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 13);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_one(&input), Some(295));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_two(&input), Some(1068781));
    }
}
