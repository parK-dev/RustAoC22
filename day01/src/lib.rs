#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = r#"
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
"#;

    #[test]
    fn most_calories() {
        let result = get_highest_calories(INPUT);
        assert_eq!(result, 24000)
    }

    #[test]
    fn sum_three_most_calories() {
        let result = get_top_three_highest_calories(INPUT);
        assert_eq!(result, 45000)
    }
}

pub fn get_highest_calories(input: &'static str) -> u32 {
    let max = input
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .filter_map(|x| x.parse::<u32>().ok())
                .sum::<u32>()
        })
        .max()
        .unwrap_or(0);
    format!("The most amount of calories carried by an elf is: {}", &max);
    max
}

pub fn get_top_three_highest_calories(input: &'static str) -> u32 {
    let mut loads = input
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .filter_map(|x| x.parse::<u32>().ok())
                .sum::<u32>()
        })
        .collect::<Vec<_>>();

    loads.sort_by(|a, b| b.cmp(a));
    let top_three = loads.iter().take(3).sum::<u32>();

    println!(
        "The amount of calories carried by the top three elves is: {}",
        &top_three
    );
    top_three
}
