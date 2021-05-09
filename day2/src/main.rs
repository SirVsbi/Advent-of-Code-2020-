use recap::Recap;
use serde::Deserialize;


#[derive(Debug, Recap, Deserialize)]
#[recap(regex = r#"(?P<min>\d+)-(?P<max>\d+) (?P<character>.): (?P<password>.*)"#)]
struct Policy{
    min: usize,
    max: usize,
    character: char,
    password: String
}

impl Policy{
    fn validate(&self) -> bool {
        let char_count = self.password.chars().filter(|&char| char == self.character).count();
        return char_count >= self.min && char_count <= self.max;
    }

    fn validate_part2(&self) -> bool{
        let char1 = self.password.chars().nth(self.min - 1).unwrap_or('0');
        let char2 = self.password.chars().nth(self.max - 1).unwrap_or('0');
        return (self.character == char1) ^ (self.character == char2);

    }
}

fn parse_input() -> Vec<Policy> {
    let lines: Vec<Policy> = include_str!("../input.txt")
        .lines()
        .map(|line| line.parse::<Policy>().unwrap())
        .collect();
    lines
}


fn main() {
    let lines = parse_input();
    println!("{}", lines.iter().filter(|it| it.validate()).count());
    println!("{}", lines.iter().filter(|it| it.validate_part2()).count());
}


