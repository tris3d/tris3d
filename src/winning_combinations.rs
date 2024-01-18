use crate::board::POSITION;
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
    // Let T = (A, B, C) be a tern of vectors.
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
    //     semi-sum(A, B) = C
    //
    // Since semi-sum is cyclic, then A, B, C can be choosen in any order.
    let vector_semi_sum = semi_sum(vector_a, vector_b);
    if !are_equal(vector_semi_sum, vector_c) {
        return Ok(false);
    }

    // From now on, the vectors are aligned on a line.

    // If any vector is the center then T is a winning combination.
    //
    // Geometrically, it means that T is a line passing through the center.

    if (position_a == '*') || (position_b == '*') || (position_c == '*') {
        return Ok(true);
    }

    // If there exist indexes k, h where
    //
    // ```
    // A[k] = B[k]
    // ```
    //
    // and
    //
    // ```
    // A[h] = B[h]
    // ```
    //
    // then T is a winning combination.
    //
    // Geometrically, it means that T is a line parallel to one of the axises.

    if (vector_a.0 == vector_b.0) && (vector_a.1 == vector_b.1) {
        return Ok(true);
    }

    if (vector_a.0 == vector_b.0) && (vector_a.2 == vector_b.2) {
        return Ok(true);
    }

    if (vector_a.1 == vector_b.1) && (vector_a.2 == vector_b.2) {
        return Ok(true);
    }

    // If it exists an index k where the k-th coordinates of A and B are equal,
    // that is
    //
    // ```
    // A[k] = B[k]
    // ```
    //
    // then let F be a vector where F[h] = 1 for every k != h,
    // in other words: F is the center of a face.
    //
    // If T contains F, then it is a winning combination.
    //
    // Geometrically, it means that T is a diagonal on a cube face.

    if vector_a.0 == vector_b.0 {
        let vector_f: Z3xZ3xZ3Vector = (vector_a.0, 1, 1);
        if are_equal(vector_f, vector_a)
            || are_equal(vector_f, vector_b)
            || are_equal(vector_f, vector_c)
        {
            return Ok(true);
        }
    }

    if vector_a.1 == vector_b.1 {
        let vector_f: Z3xZ3xZ3Vector = (1, vector_a.1, 1);
        if are_equal(vector_f, vector_a)
            || are_equal(vector_f, vector_b)
            || are_equal(vector_f, vector_c)
        {
            return Ok(true);
        }
    }

    if vector_a.2 == vector_b.2 {
        let vector_f: Z3xZ3xZ3Vector = (1, 1, vector_a.2);
        if are_equal(vector_f, vector_a)
            || are_equal(vector_f, vector_b)
            || are_equal(vector_f, vector_c)
        {
            return Ok(true);
        }
    }

    // All other cases are not a win.
    Ok(false)
}

#[cfg(test)]
mod tests {
    use super::*;

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
                    2 => Some('T'),
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

    #[test]
    fn position_of_vector_works() {
        match position_of_vector((0, 0, 0)) {
            Some(position) => {
                assert_eq!(position, 'A')
            }
            None => {
                assert!(false);
            }
        }
        match position_of_vector((0, 1, 0)) {
            Some(position) => {
                assert_eq!(position, 'B')
            }
            None => {
                assert!(false);
            }
        }
        match position_of_vector((0, 2, 0)) {
            Some(position) => {
                assert_eq!(position, 'C')
            }
            None => {
                assert!(false);
            }
        }
        match position_of_vector((1, 2, 0)) {
            Some(position) => {
                assert_eq!(position, 'D')
            }
            None => {
                assert!(false);
            }
        }
        match position_of_vector((2, 2, 0)) {
            Some(position) => {
                assert_eq!(position, 'E')
            }
            None => {
                assert!(false);
            }
        }
        match position_of_vector((2, 1, 0)) {
            Some(position) => {
                assert_eq!(position, 'F')
            }
            None => {
                assert!(false);
            }
        }
        match position_of_vector((2, 0, 0)) {
            Some(position) => {
                assert_eq!(position, 'G')
            }
            None => {
                assert!(false);
            }
        }
        match position_of_vector((1, 0, 0)) {
            Some(position) => {
                assert_eq!(position, 'H')
            }
            None => {
                assert!(false);
            }
        }
        match position_of_vector((1, 1, 0)) {
            Some(position) => {
                assert_eq!(position, 'I')
            }
            None => {
                assert!(false);
            }
        }
        match position_of_vector((0, 0, 1)) {
            Some(position) => {
                assert_eq!(position, 'J')
            }
            None => {
                assert!(false);
            }
        }
        match position_of_vector((0, 1, 1)) {
            Some(position) => {
                assert_eq!(position, 'K')
            }
            None => {
                assert!(false);
            }
        }
        match position_of_vector((0, 2, 1)) {
            Some(position) => {
                assert_eq!(position, 'L')
            }
            None => {
                assert!(false);
            }
        }
        match position_of_vector((1, 2, 1)) {
            Some(position) => {
                assert_eq!(position, 'M')
            }
            None => {
                assert!(false);
            }
        }
        match position_of_vector((2, 2, 1)) {
            Some(position) => {
                assert_eq!(position, 'N')
            }
            None => {
                assert!(false);
            }
        }
        match position_of_vector((2, 1, 1)) {
            Some(position) => {
                assert_eq!(position, 'O')
            }
            None => {
                assert!(false);
            }
        }
        match position_of_vector((2, 0, 1)) {
            Some(position) => {
                assert_eq!(position, 'P')
            }
            None => {
                assert!(false);
            }
        }
        match position_of_vector((1, 0, 1)) {
            Some(position) => {
                assert_eq!(position, 'Q')
            }
            None => {
                assert!(false);
            }
        }
        match position_of_vector((0, 0, 2)) {
            Some(position) => {
                assert_eq!(position, 'R')
            }
            None => {
                assert!(false);
            }
        }
        match position_of_vector((0, 1, 2)) {
            Some(position) => {
                assert_eq!(position, 'S')
            }
            None => {
                assert!(false);
            }
        }
        match position_of_vector((0, 2, 2)) {
            Some(position) => {
                assert_eq!(position, 'T')
            }
            None => {
                assert!(false);
            }
        }
        match position_of_vector((1, 2, 2)) {
            Some(position) => {
                assert_eq!(position, 'U')
            }
            None => {
                assert!(false);
            }
        }
        match position_of_vector((2, 2, 2)) {
            Some(position) => {
                assert_eq!(position, 'V')
            }
            None => {
                assert!(false);
            }
        }
        match position_of_vector((2, 1, 2)) {
            Some(position) => {
                assert_eq!(position, 'W')
            }
            None => {
                assert!(false);
            }
        }
        match position_of_vector((2, 0, 2)) {
            Some(position) => {
                assert_eq!(position, 'Y')
            }
            None => {
                assert!(false);
            }
        }
        match position_of_vector((1, 0, 2)) {
            Some(position) => {
                assert_eq!(position, 'X')
            }
            None => {
                assert!(false);
            }
        }
        match position_of_vector((1, 1, 2)) {
            Some(position) => {
                assert_eq!(position, 'Z')
            }
            None => {
                assert!(false);
            }
        }
        match position_of_vector((1, 1, 1)) {
            Some(position) => {
                assert_eq!(position, '*')
            }
            None => {
                assert!(false);
            }
        }
    }

    #[test]
    fn position_of_vector_is_inverse_of_vector_of_position() {
        for position in POSITION {
            assert_eq!(
                position,
                position_of_vector(vector_of_position(position).unwrap()).unwrap()
            )
        }
    }

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

    // #[test]
    // fn get_is_winning_combination_works() {
    //     match get_is_winning_combination('A', '*', 'V') {
    //         Ok(result) => {
    //             assert_eq!(true, result)
    //         }
    //         Err(_) => {
    //             assert!(false)
    //         }
    //     }
    //     for (vector1, vector2, vector3) in WINNING_COMBINATIONS {
    //         let position_1 = position_of_vector(vector1).unwrap();
    //         let position_2 = position_of_vector(vector2).unwrap();
    //         let position_3 = position_of_vector(vector3).unwrap();
    //         println!("{} {} {}", position_1, position_2, position_3);
    //         match get_is_winning_combination(position_1, position_2, position_3) {
    //             Ok(result) => {
    //                 assert_eq!(true, result)
    //             }
    //             Err(_) => {
    //                 assert!(false)
    //             }
    //         }
    //     }
    // }
}
