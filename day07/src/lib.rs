#[test]
fn test_part_one() {
    assert_eq!(solve_part_one(include_str!("./test_input.txt")), 95437)
}

#[test]
fn test_part_two() {
    assert_eq!(solve_part_two(include_str!("./test_input.txt")), 24933642)
}

pub fn parse(input: &mut std::str::Lines) -> Vec<u64> {
    let (mut total, mut subdirs) = (0, vec![]);
    loop {
        match input
            .next()
            .map(|s| s.split_whitespace().collect::<Vec<_>>())
            .as_deref()
        {
            Some(["$", "cd", ".."]) | None => break,
            Some(["$", "cd", dir]) if *dir != "/" => {
                subdirs.extend(parse(input));
                total += subdirs.last().unwrap();
            }
            Some([s, _]) if *s != "$" && *s != "dir" => {
                total += s.parse::<u64>().unwrap();
            }
            _ => (),
        }
    }
    subdirs.push(total);
    subdirs
}

pub fn solve_part_one(input: &str) -> u64 {
    parse(&mut input.lines())
        .into_iter()
        .filter(|&s| s <= 100_000)
        .sum()
}

pub fn solve_part_two(input: &str) -> u64 {
    let mut sizes = parse(&mut input.lines());
    let missing = 30_000_000 - (70_000_000 - sizes.last().unwrap());
    sizes.sort_unstable();
    sizes.into_iter().find(|&s| s >= missing).unwrap()
}
