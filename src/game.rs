use crate::board::Board;

pub struct Game {
    pub board: Board,
    player_ids: Vec<String>,
}

#[derive(Debug, PartialEq)]
pub enum GameError {
    CannotAddMoreThanThreePlayers,
    CannotAddSamePlayerTwice,
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
    ///
    /// ```
    /// let mut game = tris3d::new_game();
    /// let player_1 = String::from("Alice");
    /// game.add_player(player_1).unwrap();
    /// ```
    pub fn add_player(&mut self, player_id: String) -> Result<(), GameError> {
        if self.num_players() == 3 {
            return Err(GameError::CannotAddMoreThanThreePlayers);
        }
        if self.player_ids.contains(&player_id) {
            return Err(GameError::CannotAddSamePlayerTwice);
        }
        self.player_ids.push(player_id);
        Ok(())
    }

    pub fn num_players(&self) -> usize {
        self.player_ids.len()
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_game_has_no_players() {
        assert_eq!(Game::new().num_players(), 0);
    }

    #[test]
    fn add_player_increments_num_players() {
        let mut tris3d = Game::new();
        match tris3d.add_player(String::from("player 1")) {
            Ok(_) => assert_eq!(tris3d.num_players(), 1),
            Err(_) => assert!(false),
        }
        match tris3d.add_player(String::from("player 2")) {
            Ok(_) => assert_eq!(tris3d.num_players(), 2),
            Err(_) => assert!(false),
        }
        match tris3d.add_player(String::from("player 3")) {
            Ok(_) => assert_eq!(tris3d.num_players(), 3),
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
            Err(error) => assert_eq!(error, GameError::CannotAddSamePlayerTwice),
        }
    }

    #[test]
    fn add_player_does_not_add_more_players_than_allowed() {
        let mut tris3d = Game::new();
        match tris3d.add_player(String::from("player 1")) {
            Ok(_) => assert!(true),
            Err(_) => assert!(false),
        }
        match tris3d.add_player(String::from("player 2")) {
            Ok(_) => assert!(true),
            Err(_) => assert!(false),
        }
        match tris3d.add_player(String::from("player 3")) {
            Ok(_) => assert!(true),
            Err(_) => assert!(false),
        }
        match tris3d.add_player(String::from("player 4")) {
            Ok(_) => assert!(false),
            Err(error) => assert_eq!(error, GameError::CannotAddMoreThanThreePlayers),
        }
    }
}
