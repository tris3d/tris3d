mod board;
mod z3;

use crate::board::Board;

pub struct Player {
    pub id: String,
}

pub struct Match {
    pub board: Board,
    player_ids: Vec<String>,
}

impl Match {
    /// Create an new match, with no players and an empty board.
    ///
    /// ```
    /// let tris3d = tris3d::Match::new();
    /// ```
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
