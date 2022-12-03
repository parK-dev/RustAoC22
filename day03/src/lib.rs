// Alphabetical values:
// a..z = 1..26
// A..Z = 27.. 52

// Byte values as u16:
// b'a' = 97
// b'A' = 65

// Map byte value to alphabetical value:

// if byte is lowercase -> 'a'(97) then 97 - 97 + 1 = 1 (alphabet's a)
// if byte is uppercase ->  'A'(65) then  65 - 65 + 27 = 27 (alphabet's A)

pub fn solve_part_one(input: &'static [u8]) -> u16 {
    input
        .split(|b| b == &b'\n')
        .map(|l| l.split_at(l.len() / 2))
        .map(|(first_half, second_half)| {
            first_half
                .iter()
                .filter(|b| second_half.contains(b))
                .map(|b| {
                    if b >= &b'a' {
                        (b - b'a') as u16 + 1
                    } else {
                        (b - b'A') as u16 + 27
                    }
                })
                .next()
                .unwrap_or_default()
        })
        .sum::<u16>()
}

pub fn solve_part_two(input: &'static [u8]) -> u16 {
    input
        .split(|b| b == &b'\n')
        .collect::<Vec<_>>()
        .chunks_exact(3)
        .map(|set| {
            set[0]
                .iter()
                .find(|b| set[1].contains(b) && set[2].contains(b))
                .unwrap()
        })
        .map(|b| {
            if b >= &b'a' {
                (b - b'a') as u16 + 1
            } else {
                (b - b'A') as u16 + 27
            }
        })
        .sum::<u16>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        let result = solve_part_one(include_bytes!("./test_input.txt"));
        assert_eq!(result, 157)
    }

    #[test]
    fn part_two() {
        let result = solve_part_two(include_bytes!("./test_input.txt"));
        assert_eq!(result, 70)
    }
}
