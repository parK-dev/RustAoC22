struct Round {
    win: bool,
    draw: bool,
    shape: Shape,
}

impl Round {
    fn new(win: bool, draw: bool, shape: Shape) -> Self {
        Self { win, draw, shape }
    }
    fn get_points(&self) -> u16 {
        let mut score: u16 = 0;
        if self.win {
            score += 6;
        }

        if self.draw {
            score += 3;
        }
        match &self.shape {
            Shape::Rock => score += 1,
            Shape::Paper => score += 2,
            Shape::Scissors => score += 3,
        }
        score
    }
}

pub enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn from_char(s: char) -> Self {
        match s {
            'A' | 'X' => Self::Rock,
            'B' | 'Y' => Self::Paper,
            'C' | 'Z' => Self::Scissors,
            _ => unreachable!(),
        }
    }
}

pub fn get_score_with_first_strategy(input: &'static str) -> u16 {
    include_str!("./input.txt")
        .lines()
        .map(|round| {
            get_score_first_strat((
                Shape::from_char(round.chars().next().unwrap().into()),
                Shape::from_char(round.chars().nth(2).unwrap().into()),
            ))
        })
        .collect::<Vec<u16>>()
        .iter()
        .sum::<u16>()
}

pub fn get_score_with_second_strategy(input: &'static str) -> u16 {
    include_str!("./input.txt")
        .lines()
        .map(|round| {
            get_score_second_strat((
                Shape::from_char(round.chars().next().unwrap().into()),
                Shape::from_char(round.chars().nth(2).unwrap().into()),
            ))
        })
        .collect::<Vec<u16>>()
        .iter()
        .sum::<u16>()
}

// Normal rules of RPS for strat 1
pub fn get_score_first_strat(input: (Shape, Shape)) -> u16 {
    match input {
        (Shape::Scissors, Shape::Scissors)
        | (Shape::Paper, Shape::Paper)
        | (Shape::Rock, Shape::Rock) => Round::new(false, true, input.1).get_points(),
        (Shape::Scissors, Shape::Paper)
        | (Shape::Rock, Shape::Scissors)
        | (Shape::Paper, Shape::Rock) => Round::new(false, false, input.1).get_points(),
        (Shape::Paper, Shape::Scissors)
        | (Shape::Rock, Shape::Paper)
        | (Shape::Scissors, Shape::Rock) => Round::new(true, false, input.1).get_points(),
    }
}

// start 2
// Player gets:
// Rock he needs lose
// Paper he needs to draw
// Scissors he needs win
pub fn get_score_second_strat(input: (Shape, Shape)) -> u16 {
    match input {
        (Shape::Rock, Shape::Rock) => Round::new(false, false, Shape::Scissors).get_points(),
        (Shape::Paper, Shape::Rock) => Round::new(false, false, Shape::Rock).get_points(),
        (Shape::Scissors, Shape::Rock) => Round::new(false, false, Shape::Paper).get_points(),
        (Shape::Scissors, Shape::Scissors) => Round::new(true, false, Shape::Rock).get_points(),
        (Shape::Rock, Shape::Scissors) => Round::new(true, false, Shape::Paper).get_points(),
        (Shape::Paper, Shape::Scissors) => Round::new(true, false, Shape::Scissors).get_points(),
        (Shape::Rock, Shape::Paper)
        | (Shape::Paper, Shape::Paper)
        | (Shape::Scissors, Shape::Paper) => Round::new(false, true, input.0).get_points(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn score_following_first_strategy() {
        let result = get_score_with_first_strategy(include_str!("./input.txt"));
        assert_eq!(result, 11666)
    }

    #[test]
    fn score_following_second_strategy() {
        let result = get_score_with_second_strategy(include_str!("./input.txt"));
        assert_eq!(result, 23)
    }
}
