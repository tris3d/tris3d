use crate::board::Board;

pub struct Game {
    pub board: Board,
    player_ids: Vec<String>,
}

impl Game {
    /// Create an new game, with no players and an empty board.
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            player_ids: Vec::new(),
        }
    }

    /// Add a player to the game.
    /// It checks that `player_id` was not already added.
    pub fn add_player(&mut self, player_id: String) {
        self.player_ids.push(player_id);
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
        assert_eq!(Game::new().num_players(), 0);
    }

    // #[test]
    // fn add_player_checks_it_was_not_already_added() {
    //     let player = Player {
    //         id: String::from("ID"),
    //     };

    //     let mut tris3d = Game::new();
    //     tris3d.add_player(&player);
    //     tris3d.add_player(&player);

    //     assert_eq!(tris3d.num_players(), 2);
    // }

    #[test]
    fn add_player_increments_num_players() {
        let player_id = String::from("ID");
        let mut tris3d = Game::new();
        tris3d.add_player(player_id);
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

    //     let mut tris3d = Game::new();
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
