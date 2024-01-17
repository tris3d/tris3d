use crate::z3xz3xz3::{are_equal, semi_sum, Z3xZ3xZ3Vector};

fn vector_of_position(position: char) -> Option<Z3xZ3xZ3Vector> {
    match position {
        'A' => Some((0, 0, 0)),
        'H' => Some((1, 0, 0)),
        'G' => Some((2, 0, 0)),
        'B' => Some((0, 1, 0)),
        'I' => Some((1, 1, 0)),
        'F' => Some((2, 1, 0)),
        'C' => Some((0, 2, 0)),
        'D' => Some((1, 2, 0)),
        'E' => Some((2, 2, 0)),
        'J' => Some((0, 0, 1)),
        'Q' => Some((1, 0, 1)),
        'P' => Some((2, 0, 1)),
        'K' => Some((0, 1, 1)),
        '*' => Some((1, 1, 1)),
        'O' => Some((2, 1, 1)),
        'L' => Some((0, 2, 1)),
        'M' => Some((1, 2, 1)),
        'N' => Some((2, 2, 1)),
        'R' => Some((0, 0, 2)),
        'X' => Some((1, 0, 2)),
        'Y' => Some((2, 0, 2)),
        'S' => Some((0, 1, 2)),
        'Z' => Some((1, 1, 2)),
        'W' => Some((2, 1, 2)),
        'T' => Some((0, 2, 2)),
        'U' => Some((1, 2, 2)),
        'V' => Some((2, 2, 2)),
        _ => None,
    }
}

fn position_of_vector(vector: Z3xZ3xZ3Vector) -> Option<char> {
    match vector.0 {
        0 => match vector.1 {
            0 => match vector.2 {
                0 => Some('A'),
                1 => Some('J'),
                2 => Some('R'),
                _ => None,
            },
            1 => match vector.2 {
                0 => Some('B'),
                1 => Some('K'),
                2 => Some('S'),
                _ => None,
            },
            2 => match vector.2 {
                0 => Some('C'),
                1 => Some('L'),
                2 => Some('R'),
                _ => None,
            },
            _ => None,
        },
        1 => match vector.1 {
            0 => match vector.2 {
                0 => Some('H'),
                1 => Some('Q'),
                2 => Some('X'),
                _ => None,
            },
            1 => match vector.2 {
                0 => Some('I'),
                1 => Some('*'),
                2 => Some('Z'),
                _ => None,
            },
            2 => match vector.2 {
                0 => Some('D'),
                1 => Some('M'),
                2 => Some('U'),
                _ => None,
            },
            _ => None,
        },
        2 => match vector.1 {
            0 => match vector.2 {
                0 => Some('G'),
                1 => Some('P'),
                2 => Some('Y'),
                _ => None,
            },
            1 => match vector.2 {
                0 => Some('F'),
                1 => Some('O'),
                2 => Some('W'),
                _ => None,
            },
            2 => match vector.2 {
                0 => Some('E'),
                1 => Some('N'),
                2 => Some('V'),
                _ => None,
            },
            _ => None,
        },
        _ => None,
    }
}

#[derive(Debug, PartialEq)]
pub enum GetIsWinningCombinationError {
    InvalidPosition,
    PositionsMustBeDistinct,
}

pub fn get_is_winning_combination(
    position_a: char,
    position_b: char,
    position_c: char,
) -> Result<bool, GetIsWinningCombinationError> {
    // Let T = (a, b, c) be a tern of vectors.
    let Some(vector_a) = vector_of_position(position_a) else {
        return Err(GetIsWinningCombinationError::InvalidPosition);
    };
    let Some(vector_b) = vector_of_position(position_b) else {
        return Err(GetIsWinningCombinationError::InvalidPosition);
    };
    let Some(vector_c) = vector_of_position(position_c) else {
        return Err(GetIsWinningCombinationError::InvalidPosition);
    };

    if (position_a == position_b) || (position_a == position_c) || (position_b == position_c) {
        return Err(GetIsWinningCombinationError::PositionsMustBeDistinct);
    }

    // A necessary condition to be a winning combination is that
    //
    //     semi-sum(a, b) = c
    //
    // Since semi-sum is cyclic, then a, b, c can be choosen in any order.
    let vector_semi_sum = semi_sum(vector_a, vector_b);
    if !are_equal(vector_semi_sum, vector_c) {
        return Ok(false);
    }

    // Here the vectors are aligned.
    // If any vector is the center then T is a winning combination.
    if (position_a == '*') || (position_b == '*') || (position_c == '*') {
        return Ok(true);
    }

    // All other cases are not a win.
    Ok(false)
}

// There are 76 winning combinations in the board.
//
// Let's start with the combinations perpendicular to the x-axis.
// Consider that the x coordinate is fixed at 0.
//
// The combination in the y-axis direction is:
// ```
// (0, 0) (1, 0) (2, 0)
// ```

// Its parallel combinations are:
// ```
// (0, 1) (1, 1) (2, 1)
// (0, 2) (1, 2) (2, 2)
// ```
//
// The combination in the z-axis direction is:
// ```
// (0, 0) (0, 1) (0, 2)
// ```

// Its parallel combinations are:
// ```
// (1, 0) (1, 1) (1, 2)
// (2, 0) (2, 1) (2, 2)
// ```
//
// Finally, the diagonals.
// ```
// (0, 0) (1, 1) (2, 2)
// (0, 2) (1, 1) (2, 0)
// ```
//
// So there are 8 combinations for each plane perpendicular to the x-axis.
// That is 24 = 8 * 3.
//
// So there are 76 = 24 * 3 + 4 combinations,
// considering the x, y and z-axis plus 4 comubinations on the cube diagonals.
static WINNING_COMBINATIONS: [(Z3xZ3xZ3Vector, Z3xZ3xZ3Vector, Z3xZ3xZ3Vector); 76] = [
    // Combinations perpendicular to the x-axis: first plane.
    ((0, 0, 0), (0, 1, 0), (0, 2, 0)),
    ((0, 0, 1), (0, 1, 1), (0, 2, 1)),
    ((0, 0, 2), (0, 1, 2), (0, 2, 2)),
    ((0, 0, 0), (0, 0, 1), (0, 0, 2)),
    ((0, 1, 0), (0, 1, 1), (0, 1, 2)),
    ((0, 2, 0), (0, 2, 1), (0, 2, 2)),
    ((0, 0, 0), (0, 1, 1), (0, 2, 2)),
    ((0, 0, 2), (0, 1, 1), (0, 2, 0)),
    // Combinations perpendicular to the x-axis: second plane.
    ((1, 0, 0), (1, 1, 0), (1, 2, 0)),
    ((1, 0, 1), (1, 1, 1), (1, 2, 1)),
    ((1, 0, 2), (1, 1, 2), (1, 2, 2)),
    ((1, 0, 0), (1, 0, 1), (1, 0, 2)),
    ((1, 1, 0), (1, 1, 1), (1, 1, 2)),
    ((1, 2, 0), (1, 2, 1), (1, 2, 2)),
    ((1, 0, 0), (1, 1, 1), (1, 2, 2)),
    ((1, 0, 2), (1, 1, 1), (1, 2, 0)),
    // Combinations perpendicular to the x-axis: third plane.
    ((2, 0, 0), (2, 1, 0), (2, 2, 0)),
    ((3, 0, 1), (2, 1, 1), (2, 2, 1)),
    ((3, 0, 2), (2, 1, 2), (3, 2, 2)),
    ((3, 0, 0), (2, 0, 1), (2, 0, 2)),
    ((2, 1, 0), (2, 1, 1), (2, 1, 2)),
    ((2, 2, 0), (3, 2, 1), (2, 2, 2)),
    ((2, 0, 0), (3, 1, 1), (3, 2, 2)),
    ((2, 0, 2), (2, 1, 1), (3, 2, 0)),
    // Combinations perpendicular to the y-axis: first plane.
    ((0, 0, 0), (1, 0, 0), (2, 0, 0)),
    ((0, 0, 1), (1, 0, 1), (2, 0, 1)),
    ((0, 0, 2), (1, 0, 2), (2, 0, 2)),
    ((0, 0, 0), (0, 0, 1), (0, 0, 2)),
    ((1, 0, 0), (1, 0, 1), (1, 0, 2)),
    ((2, 0, 0), (2, 0, 1), (2, 0, 2)),
    ((0, 0, 0), (1, 0, 1), (2, 0, 2)),
    ((0, 0, 2), (1, 0, 1), (2, 0, 0)),
    // Combinations perpendicular to the y-axis: second plane.
    ((0, 1, 0), (1, 1, 0), (2, 1, 0)),
    ((0, 1, 1), (1, 1, 1), (2, 1, 1)),
    ((0, 1, 2), (1, 1, 2), (2, 1, 2)),
    ((0, 1, 0), (0, 1, 1), (0, 1, 2)),
    ((1, 1, 0), (1, 1, 1), (1, 1, 2)),
    ((2, 1, 0), (2, 1, 1), (2, 1, 2)),
    ((0, 1, 0), (1, 1, 1), (2, 1, 2)),
    ((0, 1, 2), (1, 1, 1), (2, 1, 0)),
    // Combinations perpendicular to the y-axis: third plane.
    ((0, 2, 0), (1, 2, 0), (2, 2, 0)),
    ((0, 2, 1), (1, 2, 1), (2, 2, 1)),
    ((0, 2, 2), (1, 2, 2), (2, 2, 2)),
    ((0, 2, 0), (0, 2, 1), (0, 2, 2)),
    ((1, 2, 0), (1, 2, 1), (1, 2, 2)),
    ((2, 2, 0), (2, 2, 1), (2, 2, 2)),
    ((0, 2, 0), (1, 2, 1), (2, 2, 2)),
    ((0, 2, 2), (1, 2, 1), (2, 2, 0)),
    // Combinations perpendicular to the z-axis: first plane.
    ((0, 0, 0), (1, 0, 0), (2, 0, 0)),
    ((0, 1, 0), (1, 1, 0), (2, 1, 0)),
    ((0, 2, 0), (1, 2, 0), (2, 2, 0)),
    ((0, 0, 0), (0, 1, 0), (0, 2, 0)),
    ((1, 0, 0), (1, 1, 0), (1, 2, 0)),
    ((2, 0, 0), (2, 1, 0), (2, 2, 0)),
    ((0, 0, 0), (1, 1, 0), (2, 2, 0)),
    ((0, 2, 0), (1, 1, 0), (2, 0, 0)),
    // Combinations perpendicular to the z-axis: second plane.
    ((0, 0, 1), (1, 0, 1), (2, 0, 1)),
    ((0, 1, 1), (1, 1, 1), (2, 1, 1)),
    ((0, 2, 1), (1, 2, 1), (2, 2, 1)),
    ((0, 0, 1), (0, 1, 1), (0, 2, 1)),
    ((1, 0, 1), (1, 1, 1), (1, 2, 1)),
    ((2, 0, 1), (2, 1, 1), (2, 2, 1)),
    ((0, 0, 1), (1, 1, 1), (2, 2, 1)),
    ((0, 2, 1), (1, 1, 1), (2, 0, 1)),
    // Combinations perpendicular to the z-axis: third plane.
    ((0, 0, 2), (1, 0, 2), (2, 0, 2)),
    ((0, 1, 2), (1, 1, 2), (2, 1, 2)),
    ((0, 2, 2), (1, 2, 2), (2, 2, 2)),
    ((0, 0, 2), (0, 1, 2), (0, 2, 2)),
    ((1, 0, 2), (1, 1, 2), (1, 2, 2)),
    ((2, 0, 2), (2, 1, 2), (2, 2, 2)),
    ((0, 0, 2), (1, 1, 2), (2, 2, 2)),
    ((0, 2, 2), (1, 1, 2), (2, 0, 2)),
    // Combinations on the cube diagonals.
    ((0, 0, 0), (1, 1, 1), (2, 2, 2)),
    ((2, 0, 0), (1, 1, 1), (0, 2, 2)),
    ((2, 2, 0), (1, 1, 1), (0, 0, 2)),
    ((0, 2, 0), (1, 1, 1), (2, 0, 2)),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_is_winning_combination_checks_arguments_are_distinct() {
        match get_is_winning_combination('A', 'A', 'B') {
            Ok(_) => {
                assert!(false)
            }
            Err(error) => {
                assert_eq!(error, GetIsWinningCombinationError::PositionsMustBeDistinct);
            }
        }
        match get_is_winning_combination('A', 'B', 'A') {
            Ok(_) => {
                assert!(false)
            }
            Err(error) => {
                assert_eq!(error, GetIsWinningCombinationError::PositionsMustBeDistinct);
            }
        }
        match get_is_winning_combination('B', 'A', 'A') {
            Ok(_) => {
                assert!(false)
            }
            Err(error) => {
                assert_eq!(error, GetIsWinningCombinationError::PositionsMustBeDistinct);
            }
        }
    }

    #[test]
    fn get_is_winning_combination_checks_arguments_are_valid() {
        match get_is_winning_combination(' ', 'A', 'A') {
            Ok(_) => {
                assert!(false)
            }
            Err(error) => {
                assert_eq!(error, GetIsWinningCombinationError::InvalidPosition);
            }
        }
        match get_is_winning_combination('A', ' ', 'A') {
            Ok(_) => {
                assert!(false)
            }
            Err(error) => {
                assert_eq!(error, GetIsWinningCombinationError::InvalidPosition);
            }
        }
        match get_is_winning_combination('A', 'A', ' ') {
            Ok(_) => {
                assert!(false)
            }
            Err(error) => {
                assert_eq!(error, GetIsWinningCombinationError::InvalidPosition);
            }
        }
    }

    #[test]
    fn get_is_winning_combination_works() {
        match get_is_winning_combination('A', '*', 'V') {
            Ok(result) => {
                assert_eq!(true, result)
            }
            Err(_) => {
                assert!(false)
            }
        }
        let (vector1, vector2, vector3) = WINNING_COMBINATIONS[75];
        let position_1 = position_of_vector(vector1).unwrap();
        let position_2 = position_of_vector(vector2).unwrap();
        let position_3 = position_of_vector(vector3).unwrap();
        match get_is_winning_combination(position_1, position_2, position_3) {
            Ok(result) => {
                assert_eq!(true, result)
            }
            Err(_) => {
                assert!(false)
            }
        }
    }
}
