use pathfinding::prelude::Grid;

fn parse(input: &str) -> impl Iterator<Item = i32> + '_ {
    input
        .lines()
        .scan(1, |x, s| {
            Some(if let Some(v) = s.strip_prefix("addx ") {
                let r = vec![*x, *x];
                *x += v.parse::<i32>().unwrap();
                r
            } else {
                vec![*x]
            })
        })
        .flatten()
}

pub fn solve_part_one(input: &str) -> i32 {
    parse(input)
        .enumerate()
        .skip(19)
        .step_by(40)
        .map(|(i, x)| x * (i + 1) as i32)
        .sum()
}

pub fn solve_part_two(input: &str) -> String {
    let screen = parse(input)
        .enumerate()
        .flat_map(|(i, x)| (x.abs_diff(i as i32 % 40) <= 1).then_some((i % 40, i / 40)))
        .collect::<Grid>();
    format!("{screen:#?}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = solve_part_one(include_str!("./test_input.txt"));
        assert_eq!(result, 13_140);
    }

    #[test]
    fn test_part_two() {
        let result = solve_part_two(include_str!("./input.txt"));
        print!("{:?}", result);
        panic!("force panic to render during test")
    }
}
