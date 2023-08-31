struct Game {
    current: usize,
    circle: Vec<usize>,
}

impl Game {
    fn parse(input: &str) -> Self {
        let cups = input
            .trim()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect::<Vec<usize>>();

        // The index of the cups is their actual value, the value the index holds is the cup they point to.
        // So using example "3->8->9->1->2->5->4->6->7" (and 7 wraps around to 3)

        // There's an extra zero at the start so we don't have to wrap the index around on every operation
        // Inserting them on their proper spot we'll end up with
        // [0, 1, 2, 3, 4, 5, 6, 7, 8, 9] <- their index
        // [0, 2, 5, 8, 6, 4, 7, 3, 9, 1] <- values they point to (the actual result)
        let mut circle = vec![0; cups.len() + 1];
        for i in 0..cups.len() {
            circle[cups[i]] = cups[(i + 1) % cups.len()];
        }

        Self {
            current: cups[0],
            circle,
        }
    }

    fn parse_extended(input: &str) -> Self {
        let cups = input
            .trim()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect::<Vec<usize>>();

        let mut circle = vec![0; 1_000_000 + 1];
        for i in 0..cups.len() - 1 {
            circle[cups[i]] = cups[i + 1];
        }

        // Make last cup in the input point to the value of the first extended cup
        // in this case the max cup was 9 so the next cup is going to be 10
        let last_cup = cups[cups.len() - 1];
        let max_cup = cups.len();
        circle[last_cup] = max_cup + 1;

        for (i, cup) in circle
            .iter_mut()
            .enumerate()
            .take(1_000_000)
            .skip(max_cup + 1)
        {
            *cup = i + 1;
        }
        // Make the last extended value wrap around to the start of input cups
        *circle.last_mut().unwrap() = cups[0];

        Self {
            current: cups[0],
            circle,
        }
    }

    fn step(&mut self) {
        let a = self.circle[self.current];
        let b = self.circle[a];
        let c = self.circle[b];

        let mut destination = self.current - 1;
        if destination == 0 {
            destination = self.circle.len() - 1;
        }
        // Check to see if what was going to be the next destination was one of the cups you just picked up
        // if it was pick the highest in the circle which is always the last element
        while [a, b, c].contains(&destination) {
            destination = destination.saturating_sub(1);
            if destination == 0 {
                destination = self.circle.len() - 1;
            }
        }

        // Now we're ready to assign the new order to the cups
        // Since we removed 3 cups after the current one, the current should now point to what `c` was pointing to
        // The cups we picked up get moved just after the destination so the `destination cup` points to `a`
        // and `c` should now point to what `destination` was pointing to.
        self.circle[self.current] = self.circle[c];
        let temp = self.circle[destination];
        self.circle[destination] = a;
        self.circle[c] = temp;

        // Finally the new current is what current was pointing to.
        self.current = self.circle[self.current];
    }

    fn stringify_positions(&self) -> String {
        let mut result = String::new();

        let mut i = 1;
        for _ in 0..self.circle.len() - 2 {
            result.push_str(&self.circle[i].to_string());
            i = self.circle[i];
        }

        result
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let mut game = Game::parse(input);

    for _ in 0..100 {
        game.step();
    }

    Some(game.stringify_positions())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut game = Game::parse_extended(input);

    for _ in 0..10_000_000 {
        game.step();
    }

    let result = game.circle[1] * game.circle[game.circle[1]];

    Some(result)
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
        assert_eq!(part_two(&input), Some(149245887792));
    }
}
