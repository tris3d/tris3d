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
    pub fn add_player(&mut self, player_id: String) -> Result<(), &str> {
        if self.num_players() == 3 {
            return Err("There are already 3 players.");
        }
        if self.player_ids.contains(&player_id) {
            return Err("Player already added.");
        }
        self.player_ids.push(player_id);
        return Ok(());
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

    #[test]
    fn add_player_increments_num_players() {
        let player_id = String::from("ID");
        let mut tris3d = Game::new();
        match tris3d.add_player(player_id) {
            Ok(_) => assert_eq!(tris3d.num_players(), 1),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn add_player_checks_it_was_not_already_added() {
        let player_id = String::from("ID");
        let same_player_id = String::from("ID");
        let mut tris3d = Game::new();
        match tris3d.add_player(player_id) {
            Ok(_) => assert!(true),
            Err(_) => assert!(false),
        }
        match tris3d.add_player(same_player_id) {
            Ok(_) => assert!(false),
            Err(message) => assert_eq!(message, "Player already added."),
        }
    }

    #[test]
    fn add_player_does_not_add_more_players_than_allowed() {
        let player_id_1 = String::from("ID1");
        let player_id_2 = String::from("ID2");
        let player_id_3 = String::from("ID3");
        let player_id_4 = String::from("ID4");
        let mut tris3d = Game::new();
        match tris3d.add_player(player_id_1) {
            Ok(_) => assert!(true),
            Err(_) => assert!(false),
        }
        match tris3d.add_player(player_id_2) {
            Ok(_) => assert!(true),
            Err(_) => assert!(false),
        }
        match tris3d.add_player(player_id_3) {
            Ok(_) => assert!(true),
            Err(_) => assert!(false),
        }
        match tris3d.add_player(player_id_4) {
            Ok(_) => assert!(false),
            Err(message) => assert_eq!(message, "There are already 3 players."),
        }
    }
}
