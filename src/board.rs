use crate::winning_combinations::get_is_winning_combination;

// Every board cell is associated with an uppercase latin letter
// or the asterisc for the center. To enumerate cells, start from the center,
// that is the '*' char. By convention the center of the cube has coordinates
// `(1, 1, 1). Move in diagonal and label the cell as 'A'.
// Any direction can be choosen, it will be then the 3d point with coordinates
// `(0, 0, 0)`. Then proceed clock-wise walking through the perimeter of the
// `z = -1` plane, and finally label the center of the plane. Done that,
// continue with the upper plane `z = 0` in the same way. Notice that
// this time the center is already taken. At the end do the same with
// the last plane `z = 1`.
//
// This is the result of the board cell labelling.
//
// ```
//            ______________________
//          /       /       /       /
//         /   T   /   U   /   V   /
//        /_______/______ /______ /
//       /       /       /       /
//      /   S   /   Z   /   W   /
//     /_______/______ /______ /
//    /       /       /       /
//   /   R   /   X   /   Y   /
//  /_______/______ /______ /
//            ______________________
//          /       /       /       /
//         /   L   /   M   /   N   /
//        /_______/______ /______ /
//       /       /       /       /
//      /   K   /   *   /   O   /
//     /_______/______ /______ /
//    /       /       /       /
//   /   J   /   Q   /   P   /
//  /_______/______ /______ /
//            ______________________
//          /       /       /       /
//         /   C   /   D   /   E   /
//        /_______/______ /______ /
//       /       /       /       /
//      /   B   /   I   /   F   /
//     /_______/______ /______ /
//    /       /       /       /
//   /   A   /   H   /   G   /
//  /_______/______ /______ /
//
//  z   y
//  ↑ ↗
//  o → x
//
// ```
//
// These are the corresponding coordinates in 3d space.
//
// ```
//            ______________________
//          /       /       /       /
//         / 0,2,2 / 1,2,2 / 2,2,2 /
//        /_______/______ /______ /
//       /       /       /       /
//      / 0,1,2 / 1,1,2 / 2,1,2 /
//     /_______/______ /______ /
//    /       /       /       /
//   / 0,0,2 / 1,0,2 / 2,0,2 /
//  /_______/______ /______ /
//            ______________________
//          /       /       /       /
//         / 0,2,1 / 1,2,1 / 2,2,1 /
//        /_______/______ /______ /
//       /       /       /       /
//      / 0,1,1 / 1,1,1 / 2,1,1 /
//     /_______/______ /______ /
//    /       /       /       /
//   / 0,0,1 / 1,0,1 / 2,0,1 /
//  /_______/______ /______ /
//            ______________________
//          /       /       /       /
//         / 0,2,0 / 1,2,0 / 2,2,0 /
//        /_______/______ /______ /
//       /       /       /       /
//      / 0,1,0 / 1,1,0 / 2,1,0 /
//     /_______/______ /______ /
//    /       /       /       /
//   / 0,0,0 / 1,0,0 / 2,0,0 /
//  /_______/______ /______ /
//
//  z   y
//  ↑ ↗
//  o → x
//
// ```
//
// The index in the `POSITION` array corresponds to the `x, y, z`
// coordinate in base 3, that is:
//
// ```
// x, y, z -> x * 9 + y * 3 + z
// ```
pub static POSITION: [char; 27] = [
    'A', 'H', 'G', 'B', 'I', 'F', 'C', 'D', 'E', // First layer, `z = 0`.
    'J', 'Q', 'P', 'K', '*', 'O', 'L', 'M', 'N', // Second layer, `z = 1`.
    'R', 'X', 'Y', 'S', 'Z', 'W', 'T', 'U', 'V', // Third layer, `z = 2`.
];

#[derive(Debug, PartialEq)]
enum BoardStatus {
    IsPlaying,
    HasWinner,
    Tie,
}

pub struct Board {
    status: BoardStatus,
    moves: Vec<char>,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    BoardIsFull,
    InvalidPosition,
    PositionAlreadyTaken,
    ThereIsAlreadyAWinner,
}

impl Board {
    /// Create an empty board.
    pub fn new() -> Self {
        Self {
            moves: Vec::new(),
            status: BoardStatus::IsPlaying,
        }
    }

    /// Add a move to the board.
    /// Return the number of winning combinations.
    pub fn add_move(&mut self, position: char) -> Result<u8, Error> {
        if self.status == BoardStatus::Tie {
            return Err(Error::BoardIsFull);
        }
        if self.status == BoardStatus::HasWinner {
            return Err(Error::ThereIsAlreadyAWinner);
        }
        if self.moves.contains(&position) {
            return Err(Error::PositionAlreadyTaken);
        }
        let mut position_is_valid = false;
        for p in POSITION {
            if position == p {
                position_is_valid = true;
                self.moves.push(position);
                break;
            }
        }
        if position_is_valid {
            let num_winning_combinations = self.get_num_winning_combinations();
            if num_winning_combinations == 0 {
                if self.moves.len() == 27 {
                    self.status = BoardStatus::Tie;
                }
                Ok(num_winning_combinations)
            } else {
                self.status = BoardStatus::HasWinner;
                Ok(num_winning_combinations)
            }
        } else {
            Err(Error::InvalidPosition)
        }
    }

    /// Check if there is any winner.
    pub fn get_num_winning_combinations(&self) -> u8 {
        let mut num_winning_combinations = 0;
        let num_moves = self.moves.len();
        // No player can win before the seventh move.
        if num_moves < 7 {
            return 0;
        }
        // Get all combinations of current player and count how many are winning combinations.
        let current_player_index = (num_moves - 1) % 3;
        for i in (current_player_index..num_moves).step_by(3) {
            for j in ((i + 3)..num_moves).step_by(3) {
                for k in ((j + 3)..num_moves).step_by(3) {
                    if get_is_winning_combination(self.moves[i], self.moves[j], self.moves[k])
                        .is_ok()
                    {
                        num_winning_combinations += 1;
                    }
                }
            }
        }

        num_winning_combinations
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_board_has_no_winning_combination() {
        assert_eq!(Board::new().get_num_winning_combinations(), 0);
    }

    #[test]
    fn empty_board_is_playing() {
        assert_eq!(Board::new().status, BoardStatus::IsPlaying);
    }

    #[test]
    fn add_move_accepts_valid_position() {
        for i in 0..POSITION.len() {
            let mut board = Board::new();
            match board.add_move(POSITION[i]) {
                Ok(_) => assert!(true),
                Err(_) => assert!(false),
            }
        }
    }

    #[test]
    fn add_move_checks_position_is_valid() {
        let mut board = Board::new();
        match board.add_move(' ') {
            Ok(_) => assert!(false),
            Err(error) => assert_eq!(error, Error::InvalidPosition),
        }
    }

    #[test]
    fn add_move_checks_position_is_not_already_taken() {
        let mut board = Board::new();
        match board.add_move('A') {
            Ok(_) => assert!(true),
            Err(_) => assert!(false),
        }
        match board.add_move('A') {
            Ok(_) => assert!(false),
            Err(error) => assert_eq!(error, Error::PositionAlreadyTaken),
        }
    }

    #[test]
    fn get_num_winning_combinations_works() {
        assert_eq!(
            Board {
                moves: vec!['A', 'H', 'G', '*', 'I', 'F', 'V'],
                status: BoardStatus::IsPlaying,
            }
            .get_num_winning_combinations(),
            1
        );
    }

    // TODO
    // #[test]
    // fn board_can_be_full_with_no_winner() {
    //     let mut board = Board::default();
    //     board.add_move('*').unwrap();
    //     board.add_move('A').unwrap();
    //     board.add_move('B').unwrap();
    //     board.add_move('C').unwrap();
    //     board.add_move('D').unwrap();
    //     board.add_move('E').unwrap();
    //     board.add_move('F').unwrap();
    //     board.add_move('G').unwrap();
    //     board.add_move('H').unwrap();
    //     board.add_move('I').unwrap();
    //     board.add_move('J').unwrap();
    //     board.add_move('K').unwrap();
    //     board.add_move('L').unwrap();
    //     board.add_move('M').unwrap();
    //     board.add_move('N').unwrap();
    //     board.add_move('O').unwrap();
    //     board.add_move('P').unwrap();
    //     board.add_move('Q').unwrap();
    //     board.add_move('R').unwrap();
    //     board.add_move('S').unwrap();
    //     board.add_move('T').unwrap();
    //     board.add_move('U').unwrap();
    //     board.add_move('V').unwrap();
    //     board.add_move('W').unwrap();
    //     board.add_move('X').unwrap();
    //     board.add_move('Y').unwrap();
    //     board.add_move('Z').unwrap();
    //     assert_eq!(board.status, BoardStatus::Tie);
    //     assert_eq!(board.get_num_winning_combinations(), 0);
    // }

    #[test]
    fn simple_game() {
        // Player one will move 'A', '*', 'V'.
        let mut board = Board::default();
        board.add_move('A').unwrap();
        board.add_move('H').unwrap();
        board.add_move('G').unwrap();
        board.add_move('*').unwrap();
        board.add_move('I').unwrap();
        board.add_move('F').unwrap();
        board.add_move('V').unwrap();
        assert_eq!(board.status, BoardStatus::HasWinner);
        assert_eq!(board.get_num_winning_combinations(), 1);
    }
}
