#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let real_input = include_str!("./input.txt");
        assert_eq!(solve("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), 5);
        assert_eq!(solve("nppdvjthqldpwncqszvftbrmjlhg", 4), 6);
        assert_eq!(solve("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4), 10);
        assert_eq!(solve("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4), 11);
        assert_eq!(solve(real_input, 4), 1965);
        assert_eq!(solve("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14), 19);
        assert_eq!(solve("bvwbjplbgvbhsrlpgdmjqwftvncz", 14), 23);
        assert_eq!(solve("nppdvjthqldpwncqszvftbrmjlhg", 14), 23);
        assert_eq!(solve("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14), 29);
        assert_eq!(solve("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14), 26);
        assert_eq!(solve(real_input, 14), 2773);
    }
}

pub fn solve(input: &str, n: usize) -> usize {
    let mut last = input.chars().take(n).collect::<Vec<char>>();
    for (i, c) in input.chars().enumerate() {
        for i in 0..=n - 2 {
            dbg!(i);
            last[i] = last[i + 1];
        }

        last[n - 1] = c;

        if last
            .iter()
            .all(|c| last.iter().filter(|d| *d == c).count() == 1)
        {
            return i + 1;
        }
    }
    0
}
