#[derive(Debug, Clone, Copy)]
struct Cup {
    index: usize,
    label: usize,
}
impl Cup {
    fn new(index: usize, label: usize) -> Self {
        Self { index, label }
    }
    fn from((index, label): (usize, &usize)) -> Self {
        Self {
            index,
            label: *label,
        }
    }
}

fn remove_cups(cups: &mut Vec<usize>, at_index: usize, count: usize) -> Vec<usize> {
    let mut removed = Vec::with_capacity(count);

    for _ in 0..3 {
        let cup = cups.remove((at_index + 1) % cups.len());
        removed.push(cup);
    }

    removed
}

fn insert_cups(cups: &mut Vec<usize>, to_insert: Vec<usize>, destination: usize) {
    to_insert
        .iter()
        .rev()
        .for_each(|c| cups.insert(destination, *c))
}

fn select_next_cup_label(current: usize, cups: &[usize], min: usize) -> Cup {
    let mut offset = 1;
    while current - offset >= min {
        if let Some(cup) = cups
            .iter()
            .enumerate()
            .find(|cup| cup.1 == &(current - offset))
        {
            return Cup::from(cup);
        }
        offset += 1;
    }

    let max_cup = cups.iter().enumerate().max_by(|x, y| x.1.cmp(y.1)).unwrap();
    Cup::from(max_cup)
}

pub fn part_one(input: &str) -> Option<String> {
    let mut cups = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect::<Vec<_>>();

    let min_cup = cups.clone().into_iter().min().unwrap();
    let len = cups.len();

    let mut current_cup = Cup::new(0, cups[0]);
    for _mov in 1..=100 {
        // println!("--- Move {mov} ---");
        // println!("Cups: {}", &cups.iter().map(|c| match *c { c if c == current_cup.label => format!("({c})"), _ => c.to_string(), }).collect::<Vec<_>>().join(" "));

        let removed = remove_cups(&mut cups, current_cup.index, 3);
        // println!("Pick up: {removed:?}");
        let destination_cup = select_next_cup_label(current_cup.label, &cups, min_cup);
        // println!("Destination: {}", destination_cup.label);
        // println!();

        insert_cups(&mut cups, removed, destination_cup.index + 1);

        // Scroll the list until the current cup is index 0
        while cups.iter().position(|c| *c == current_cup.label).unwrap() > 0 {
            let c = cups.remove(0);
            cups.push(c);
        }

        current_cup = Cup::new(1, cups[1]);
    }

    let mut result = String::with_capacity(len);
    let one_cup = cups.iter().position(|c| *c == 1).unwrap();
    for i in 0..len - 1 {
        result.push_str(&cups[(one_cup + 1 + i) % len].to_string());
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 23);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 23);
        assert_eq!(part_one(&input), Some("67384529".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 23);
        assert_eq!(part_two(&input), None);
    }
}
