type Crate = char;
type Stack = Vec<Crate>;

pub struct Instruction {
    number_of_containers: usize,
    from: usize,
    to: usize,
}

pub fn parse(input: &'static str) -> (Vec<Stack>, Vec<Instruction>) {
    let (schema, instructions) = input.split_once("\n\n").unwrap();

    let graph: Vec<_> = schema.lines().collect();

    let graph_len = graph[0].len() / 4 + 1;
    let mut stacks: Vec<Stack> = vec![Vec::with_capacity(graph.len() - 1); graph_len + 1];

    for row_number in (0..graph.len() - 1).rev() {
        let row = graph[row_number];
        for (stack_number, stack) in stacks.iter_mut().enumerate().skip(1).take(graph_len) {
            let container = row.chars().nth((stack_number - 1) * 4 + 1).unwrap();

            if !container.is_ascii_whitespace() {
                stack.push(container);
            }
        }
    }

    let instructions = instructions
        .lines()
        .map(|instruction| {
            let mut numbers = instruction
                .split_ascii_whitespace()
                .filter_map(|token| token.parse().ok());

            Instruction {
                number_of_containers: numbers.next().unwrap(),
                from: numbers.next().unwrap(),
                to: numbers.next().unwrap(),
            }
        })
        .collect();

    (stacks, instructions)
}

pub fn solve_part_one((stacks, instructions): (Vec<Stack>, Vec<Instruction>)) -> String {
    let mut stacks = stacks.to_owned();

    for instruction in instructions {
        for _ in 0..instruction.number_of_containers {
            let supply_crate = stacks[instruction.from].pop().unwrap();
            stacks[instruction.to].push(supply_crate);
        }
    }

    stacks
        .iter()
        .filter_map(|stack| stack.iter().last())
        .collect()
}

pub fn solve_part_two((stacks, instructions): (Vec<Stack>, Vec<Instruction>)) -> String {
    let mut stacks = stacks;

    for instruction in instructions {
        let from = &mut stacks[instruction.from];
        let mut moved = from.split_off(from.len() - instruction.number_of_containers);

        stacks[instruction.to].append(&mut moved);
    }

    stacks
        .iter()
        .filter_map(|stack| stack.iter().last())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        let parsed = parse(include_str!("./test_input.txt"));
        let result = solve_part_one(parsed);
        assert_eq!(result, "CMZ".to_string())
    }

    #[test]
    fn part_two() {
        let parsed = parse(include_str!("./test_input.txt"));
        let result = solve_part_two(parsed);
        assert_eq!(result, "MCD".to_string())
    }
}
