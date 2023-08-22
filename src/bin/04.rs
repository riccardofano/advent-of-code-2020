use std::collections::HashMap;

#[allow(dead_code)]
struct Document {
    byr: String,         // (Birth Year)
    iyr: String,         // (Issue Year)
    eyr: String,         // (Expiration Year)
    hgt: String,         // (Height)
    hcl: String,         // (Hair Color)
    ecl: String,         // (Eye Color)
    pid: String,         // (Passport ID)
    cid: Option<String>, // (Country ID)
}

impl Document {
    fn is_valid(&self) -> bool {
        let (height, unit) = self.hgt.split_at(self.hgt.len() - 2);
        let Ok(height) = height.parse() else {
            return false
        };
        let valid_height = match unit {
            "cm" => (150..=193).contains(&height),
            "in" => (59..=76).contains(&height),
            _ => false,
        };

        let valid_birth_year = (1920..=2002).contains(&self.byr.parse().unwrap_or(0));
        let valid_issue_year = (2010..=2020).contains(&self.iyr.parse().unwrap_or(0));
        let valid_exp_year = (2020..=2030).contains(&self.eyr.parse().unwrap_or(0));

        let valid_hair_color = self.hcl.starts_with('#')
            && self.hcl.len() == 7
            && self.hcl.chars().skip(1).all(|c| c.is_ascii_hexdigit());

        let valid_eye_color =
            ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&self.ecl.as_str());
        let valid_pid = self.pid.len() == 9 && self.pid.chars().all(|c| c.is_ascii_digit());

        valid_height
            && valid_birth_year
            && valid_issue_year
            && valid_exp_year
            && valid_hair_color
            && valid_eye_color
            && valid_pid
    }
}

impl TryFrom<&str> for Document {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut pairs: HashMap<String, String> = HashMap::new();
        value.split_whitespace().for_each(|pair| {
            let (key, value) = pair.split_once(':').unwrap();
            pairs.insert(key.to_string(), value.to_string());
        });

        Ok(Self {
            byr: pairs.remove("byr").ok_or(String::from("No byr"))?,
            iyr: pairs.remove("iyr").ok_or(String::from("No iyr"))?,
            eyr: pairs.remove("eyr").ok_or(String::from("No eyr"))?,
            hgt: pairs.remove("hgt").ok_or(String::from("No hgt"))?,
            hcl: pairs.remove("hcl").ok_or(String::from("No hcl"))?,
            ecl: pairs.remove("ecl").ok_or(String::from("No ecl"))?,
            pid: pairs.remove("pid").ok_or(String::from("No pid"))?,
            cid: pairs.remove("cid"),
        })
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let documents: Vec<_> = input.split("\n\n").collect();

    let valid_documents = documents
        .into_iter()
        .map(Document::try_from)
        .filter_map(Result::ok)
        .count();

    Some(valid_documents)
}

pub fn part_two(input: &str) -> Option<usize> {
    let documents: Vec<_> = input.split("\n\n").collect();

    let valid_documents = documents
        .into_iter()
        .map(Document::try_from)
        .filter_map(Result::ok)
        .filter(|d| d.is_valid())
        .count();

    Some(valid_documents)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = include_str!("../examples/04-2.txt");
        assert_eq!(part_two(input), Some(4));
    }
}
