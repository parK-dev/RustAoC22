use pathfinding::prelude::{directions::DIRECTIONS_4, Matrix};

pub fn parse(input: &str) -> Matrix<u8> {
    Matrix::from_rows(input.lines().map(|l| l.bytes().map(|b| b - b'0'))).unwrap()
}

pub fn part_one(input: &str) -> usize {
    let matrix = parse(input);
    matrix
        .indices()
        .filter(|&(row, col)| {
            let value = matrix[(row, col)];
            DIRECTIONS_4.into_iter().any(|direction| {
                matrix
                    .in_direction((row, col), direction)
                    .all(|(rr, cc)| matrix[(rr, cc)] < value)
            })
        })
        .count()
}

pub fn part_two(input: &str) -> usize {
    let matrix = parse(input);
    matrix
        .indices()
        .map(|(row, col)| {
            let value = matrix[(row, col)];
            DIRECTIONS_4
                .into_iter()
                .map(|direction| {
                    matrix
                        .in_direction((row, col), direction)
                        .try_fold(0, |seen, (rr, cc)| {
                            if matrix[(rr, cc)] < value {
                                Ok(seen + 1)
                            } else {
                                Err(seen + 1)
                            }
                        })
                        .unwrap_or_else(std::convert::identity)
                })
                .product()
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_one() {
        let result = part_one(include_str!("./test_input.txt"));
        assert_eq!(result, 21);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(include_str!("./test_input.txt"));
        assert_eq!(result, 8);
    }
}
