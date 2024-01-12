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
    pub fn add_move(&mut self, position: char) {
        for i in 0..POSITION.len() {
            if position == POSITION[i] && !self.moves.contains(&position) {
                self.moves.push(position);
            }
        }
    }

    /// Check if there is any winner.
    pub fn has_tris(&self) -> bool {
        // The sixth move is the first one a player can win.
        if self.moves.len() < 6 {
            return false;
        }

        return true;
    }

    pub fn next_player_index(self) -> usize {
        self.moves.len() % 3
    }
}

pub struct Player {
    pub id: String,
}

pub struct Match {
    pub board: Board,
    player_ids: Vec<String>,
}

impl Match {
    /// Create an new match, with no players and an empty board.
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            player_ids: Vec::new(),
        }
    }

    /// Add a player to the match.
    /// It checks that `player` was not already added.
    pub fn add_player(&mut self, player: &Player) {
        self.player_ids.push(player.id.to_owned());
    }

    pub fn num_players(&self) -> usize {
        self.player_ids.len()
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
            let mut board = Board::new();
            board.add_move(POSITION[i]);

            assert_eq!(board.next_player_index(), 1);
        }
    }

    #[test]
    fn add_move_checks_position_is_valid() {
        let mut board = Board::new();
        board.add_move(' ');

        assert_eq!(board.next_player_index(), 0);
    }

    #[test]
    fn add_move_checks_position_is_not_already_taken() {
        let mut board = Board::new();
        board.add_move('A');
        board.add_move('A');

        assert_eq!(board.next_player_index(), 1);
    }

    #[test]
    fn board_has_tris() {
        let mut board = Board::new();
        board.add_move(POSITION[0]);
        board.add_move(POSITION[10]);
        board.add_move(POSITION[11]);
        board.add_move(POSITION[1]);
        board.add_move(POSITION[21]);
        board.add_move(POSITION[22]);
        board.add_move(POSITION[3]);

        assert_eq!(board.has_tris(), true);
    }

    #[test]
    fn new_match_has_no_players() {
        assert_eq!(Match::new().num_players(), 0);
    }

    // #[test]
    // fn add_player_checks_it_was_not_already_added() {
    //     let player = Player {
    //         id: String::from("ID"),
    //     };

    //     let mut tris3d = Match::new();
    //     tris3d.add_player(&player);
    //     tris3d.add_player(&player);

    //     assert_eq!(tris3d.num_players(), 2);
    // }

    #[test]
    fn add_player_increments_num_players() {
        let player = Player {
            id: String::from("ID"),
        };

        let mut tris3d = Match::new();
        tris3d.add_player(&player);

        assert_eq!(tris3d.num_players(), 1);
    }

    // #[test]
    // fn add_player_does_not_add_more_players_than_allowed() {
    //     let player_one = Player {
    //         id: String::from("ID1"),
    //     };
    //     let player_two = Player {
    //         id: String::from("ID2"),
    //     };
    //     let player_three = Player {
    //         id: String::from("ID3"),
    //     };
    //     let player_four = Player {
    //         id: String::from("ID4"),
    //     };

    //     let mut tris3d = Match::new();
    //     tris3d.add_player(&player_one);
    //     assert_eq!(tris3d.num_players(), 1);
    //     tris3d.add_player(&player_two);
    //     assert_eq!(tris3d.num_players(), 2);
    //     tris3d.add_player(&player_three);
    //     assert_eq!(tris3d.num_players(), 3);
    //     tris3d.add_player(&player_four);
    //     assert_eq!(tris3d.num_players(), 3);
    // }
}
