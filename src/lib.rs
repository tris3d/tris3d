mod z3;

// Every tris3d board cell is associated with an uppercase latin letter
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
// This is the result of the tris3d board cell labelling.
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
static POSITION: [char; 27] = [
    'A', 'H', 'G', 'B', 'I', 'F', 'C', 'D', 'E', // First layer, `z = 0`.
    'J', 'Q', 'P', 'K', '*', 'O', 'L', 'M', 'N', // Second layer, `z = 1`.
    'R', 'X', 'Y', 'S', 'Z', 'W', 'T', 'U', 'V', // Third layer, `z = 2`.
];

pub struct Board {
    moves: Vec<char>,
}

impl Board {
    /// Create an empty board.
    fn new() -> Self {
        Self { moves: Vec::new() }
    }

    /// Add a move to the board.
    /// It checks that `position` is valid and not already taken.
    pub fn add_move(self, position: char) -> Board {
        let mut moves = self.moves.to_vec();
        for i in 0..POSITION.len() {
            if position == POSITION[i] && !moves.contains(&position) {
                moves.push(position);
            }
        }
        Board { moves: moves }
    }

    /// Check if there is any winner.
    pub fn has_tris(&self) -> bool {
        // The sixth move is the first one a player can win.
        if self.moves.len() < 6 {
            return false;
        }

        return true;
    }

    fn next_player_index(&self) -> usize {
        return self.moves.len() % 3;
    }
}

pub struct Match {
    id: String,
    pub board: Board,
}

impl Match {
    /// Create an new match, with no players and an empty board.
    pub fn new(match_id: String) -> Self {
        Self {
            id: match_id,
            board: Board::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_board_has_no_tris() {
        assert_eq!(Board::new().has_tris(), false);
    }

    #[test]
    fn empty_board_next_player_index() {
        assert_eq!(Board::new().next_player_index(), 0);
    }

    #[test]
    fn add_move_accepts_valid_position() {
        for i in 0..POSITION.len() {
            let board = Board::new().add_move(POSITION[i]);

            assert_eq!(board.next_player_index(), 1);
        }
    }

    #[test]
    fn add_move_checks_position_is_valid() {
        let board = Board::new().add_move(' ');

        assert_eq!(board.next_player_index(), 0);
    }

    #[test]
    fn add_move_checks_position_is_not_already_taken() {
        let board = Board::new().add_move('A').add_move('A');

        assert_eq!(board.next_player_index(), 1);
    }

    #[test]
    fn board_has_tris() {
        let board = Board::new()
            .add_move(POSITION[0])
            .add_move(POSITION[10])
            .add_move(POSITION[11])
            .add_move(POSITION[1])
            .add_move(POSITION[21])
            .add_move(POSITION[22])
            .add_move(POSITION[3]);

        assert_eq!(board.has_tris(), true);
    }
}
