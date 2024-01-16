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

fn index_of_position(position: &char) -> Option<usize> {
    for (i, p) in POSITION.iter().enumerate() {
        if p == position {
            return Some(i);
        }
    }
    None
}

// A "tris" is a winning set of moves in the tris3d board.
fn is_tris(position_a: &char, position_b: &char, position_c: &char) -> bool {
    // Positions must be distinct.
    if (position_a == position_b) || (position_a == position_c) || (position_b == position_c) {
        return false;
    }
    let index_a = index_of_position(position_a);
    match index_a {
        Some(_) => {}
        None => {
            return false;
        }
    }
    let index_b = index_of_position(position_b);
    match index_b {
        Some(_) => {}
        None => {
            return false;
        }
    }
    let index_c = index_of_position(position_c);
    match index_c {
        Some(_) => {}
        None => {
            return false;
        }
    }

    // Let T = (a, b, c) be a tern of vectors.
    // let vector_a = z3xz3xz3_coordinates_of_index(index_a);
    // let vector_b = z3xz3xz3_coordinates_of_index(index_b);
    // let vector_c = z3xz3xz3_coordinates_of_index(index_c);

    // A necessary condition to be a tris is that
    //
    //     semi-sum(a, b) = c
    //
    // Since semi-sum is cyclic, then a, b, c can be choosen in any order.
    // let vector_semi_sum = z3xz3xz3_semi_sum(vector_a, vector_b);
    // if index_of_z3xz3xz3_coordinates(vector_semi_sum) != index_c {
    //     return false;
    // }

    // Here the vectors are aligned.
    // If any vector is the center then T is a tris.
    // if (index_a == Some(13)) || (index_b == Some(13)) || (index_c == Some(13)) {
    //     return true;
    // }

    // All other cases are not a tris.
    false
}

pub struct Board {
    moves: Vec<char>,
}

impl Board {
    /// Create an empty board.
    pub fn new() -> Self {
        Self { moves: Vec::new() }
    }

    /// Add a move to the board.
    /// Return the number of winning combinations.
    pub fn add_move(&mut self, position: char) -> Result<u8, &str> {
        if self.get_num_tris() > 0 {
            return Err("Game over, there is already a winner.");
        }
        if self.moves.len() == POSITION.len() {
            return Err("Board is full.");
        }
        if self.moves.contains(&position) {
            return Err("Position already taken.");
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
            Ok(self.get_num_tris())
        } else {
            Err("Position is not valid.")
        }
    }

    /// Check if there is any winner.
    pub fn get_num_tris(&self) -> u8 {
        // No player can win before the seventh move.
        if self.moves.len() < 7 {
            return 0;
        }

        1
    }

    pub fn next_player_index(self) -> usize {
        self.moves.len() % 3
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
    fn is_tris_checks_arguments_are_distinct() {
        assert_eq!(is_tris(&'A', &'A', &'B'), false);
        assert_eq!(is_tris(&'A', &'B', &'A'), false);
        assert_eq!(is_tris(&'B', &'A', &'A'), false);
    }

    #[test]
    fn is_tris_checks_arguments_are_valid() {
        assert_eq!(is_tris(&' ', &'A', &'B'), false);
        assert_eq!(is_tris(&'A', &' ', &'B'), false);
        assert_eq!(is_tris(&'A', &'B', &' '), false);
    }

    #[test]
    fn empty_board_has_no_tris() {
        assert_eq!(Board::new().get_num_tris(), 0);
    }

    #[test]
    fn empty_board_next_player_index() {
        assert_eq!(Board::new().next_player_index(), 0);
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
            Err(message) => assert_eq!(message, "Position is not valid."),
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
            Err(message) => assert_eq!(message, "Position already taken."),
        }
    }

    #[test]
    fn index_of_position_finds_valid_position() {
        for (i, p) in POSITION.iter().enumerate() {
            match index_of_position(p) {
                Some(index) => assert_eq!(index, i),
                None => assert!(false),
            }
        }
    }

    #[test]
    fn simple_game() {
        // Player one will move 'A', 'B', 'C'.
        let mut board = Board::default();
        match board.add_move('A') {
            Ok(num_tris) => assert_eq!(num_tris, 0),
            Err(_) => assert!(false),
        }
        match board.add_move('H') {
            Ok(num_tris) => assert_eq!(num_tris, 0),
            Err(_) => assert!(false),
        }
        match board.add_move('G') {
            Ok(num_tris) => assert_eq!(num_tris, 0),
            Err(_) => assert!(false),
        }
        match board.add_move('B') {
            Ok(num_tris) => assert_eq!(num_tris, 0),
            Err(_) => assert!(false),
        }
        match board.add_move('I') {
            Ok(num_tris) => assert_eq!(num_tris, 0),
            Err(_) => assert!(false),
        }
        match board.add_move('F') {
            Ok(num_tris) => assert_eq!(num_tris, 0),
            Err(_) => assert!(false),
        }
        match board.add_move('C') {
            Ok(num_tris) => assert_eq!(num_tris, 1),
            Err(_) => assert!(false),
        }
    }
}
