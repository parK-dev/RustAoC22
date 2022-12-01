use itertools::Itertools;

fn main() {
    get_highest_calories(include_str!("../src/input.txt"));
    get_top_three_highest_calories(include_str!("../src/input.txt"));
}

fn get_highest_calories(input: &'static str) -> i32 {
    let res = input
        .split("\n\n")
        .map(|x| x.lines().filter_map(|x| x.parse::<i32>().ok()).sum::<i32>())
        .max()
        .unwrap_or(0);
    dbg!(res);
    res
}

fn get_top_three_highest_calories(input: &'static str) -> i32 {
    let res = input
        .split("\n\n")
        .map(|x| x.lines().filter_map(|x| x.parse::<i32>().ok()).sum::<i32>())
        .sorted_by(|a, b| b.cmp(a))
        .take(3)
        .sum::<i32>();
    dbg!(res);
    res
}
