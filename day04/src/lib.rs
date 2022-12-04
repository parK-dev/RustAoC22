// 1-5,2-8
// To parse the string, we split once at ','
// and a second time on '-' to get a tuple (a, b, c, d)
// We then proceed to check if the tuples include each other
pub fn solve_part_one(input: &'static str) -> usize {
    input
        .lines()
        .map(|l| {
            let (l, r) = l.split_once(',').unwrap();
            let ((a, b), (c, d)) = (l.split_once('-').unwrap(), r.split_once('-').unwrap());
            (
                a.parse::<u8>().unwrap(),
                b.parse::<u8>().unwrap(),
                c.parse::<u8>().unwrap(),
                d.parse::<u8>().unwrap(),
            )
        })
        .filter(|(a, b, c, d)| (a >= c && b <= d) || (a <= c && b >= d))
        .count()
}
pub fn solve_part_two(input: &'static str) -> usize {
    input
        .lines()
        .map(|l| {
            let (l, r) = l.split_once(',').unwrap();
            let ((a, b), (c, d)) = (l.split_once('-').unwrap(), r.split_once('-').unwrap());
            (
                a.parse::<u8>().unwrap(),
                b.parse::<u8>().unwrap(),
                c.parse::<u8>().unwrap(),
                d.parse::<u8>().unwrap(),
            )
        })
        .filter(|(a, b, c, d)| (a <= d && c <= b))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        let result = solve_part_one(include_str!("./test_input.txt"));
        assert_eq!(result, 2)
    }

    #[test]
    fn part_two() {
        let result = solve_part_two(include_str!("./test_input.txt"));
        assert_eq!(result, 4)
    }
}
