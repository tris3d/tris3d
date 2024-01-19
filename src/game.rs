use crate::board::{Board, Status as BoardStatus};
use crate::errors::Error;

#[derive(Debug, PartialEq)]
pub enum Status {
    WaitingForPlayers,
    IsPlaying,
    IsOver,
}

pub struct Game {
    board: Board,
    player_ids: Vec<String>,
    pub status: Status,
}

impl Game {
    /// Create an new game, with no players and an empty board.
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            player_ids: Vec::new(),
            status: Status::WaitingForPlayers,
        }
    }

    /// Add a player to the game.
    ///
    /// ```
    /// # let mut game = tris3d::new_game();
    /// game.add_player(String::from("Alice")).unwrap();
    /// ```
    pub fn add_player(&mut self, player_id: String) -> Result<(), Error> {
        if self.num_players() == 3 {
            return Err(Error::CannotAddMoreThanThreePlayers);
        }
        if self.player_ids.contains(&player_id) {
            return Err(Error::CannotAddSamePlayerTwice);
        }
        self.player_ids.push(player_id);
        if self.num_players() == 3 {
            self.status = Status::IsPlaying;
        }
        Ok(())
    }

    /// Add a move to the board.
    /// Return the number of winning combinations.
    ///
    /// ```
    /// # let mut game = tris3d::new_game();
    /// # game.add_player(String::from("Alice")).unwrap();
    /// # game.add_player(String::from("Bob")).unwrap();
    /// # game.add_player(String::from("Neuromancer")).unwrap();
    /// let num_winning_combinations = game.add_move(String::from("Alice"), 'A').unwrap();
    /// ```
    pub fn add_move(&mut self, player_id: String, position: char) -> Result<u8, Error> {
        if self.status == Status::WaitingForPlayers {
            return Err(Error::GameNotStartedYet);
        }
        if self.status == Status::IsOver {
            return Err(Error::GameIsOver);
        }
        if !self.player_ids.contains(&player_id) {
            return Err(Error::PlayerNotFound);
        }
        let next_player_index = self.board.get_num_moves() % 3;
        if player_id != self.player_ids[next_player_index] {
            return Err(Error::PlayerMustWaitForTurn);
        }
        match self.board.add_move(position) {
            Ok(num) => {
                if self.board.status != BoardStatus::IsPlaying {
                    self.status = Status::IsOver;
                }
                Ok(num)
            }
            Err(error) => Err(error),
        }
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
    fn new_game_is_waiting_for_players() {
        assert_eq!(Game::new().num_players(), 0);
        assert_eq!(Game::new().status, Status::WaitingForPlayers);
    }

    #[test]
    fn add_player_increments_num_players() {
        let mut game = Game::new();
        match game.add_player(String::from("player 1")) {
            Ok(_) => assert_eq!(game.num_players(), 1),
            Err(_) => assert!(false),
        }
        match game.add_player(String::from("player 2")) {
            Ok(_) => assert_eq!(game.num_players(), 2),
            Err(_) => assert!(false),
        }
        match game.add_player(String::from("player 3")) {
            Ok(_) => assert_eq!(game.num_players(), 3),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn add_player_checks_it_was_not_already_added() {
        let mut game = Game::new();
        game.add_player(String::from("Alice")).unwrap();

        assert_eq!(game.add_player(String::from("Alice")).unwrap_err(), Error::CannotAddSamePlayerTwice);
    }

    #[test]
    fn add_player_does_not_add_more_players_than_allowed() {
        let mut game = Game::new();
        game.add_player(String::from("Alice")).unwrap();
        game.add_player(String::from("Bob")).unwrap();
        game.add_player(String::from("Neuromancer")).unwrap();

        assert_eq!(game.add_player(String::from("Another player")).unwrap_err(), Error::CannotAddMoreThanThreePlayers);
    }

    #[test]
    fn add_move_checks_that_game_is_not_waiting_for_players() {
        let mut game = Game::new();
        game.add_player(String::from("Alice")).unwrap();
        game.add_player(String::from("Bob")).unwrap();

        assert_eq!(game.add_move(String::from("Alice"), 'A').unwrap_err(), Error::GameNotStartedYet);
    }

    #[test]
    fn add_move_checks_that_player_is_playing() {
        let mut game = Game::new();
        game.add_player(String::from("Alice")).unwrap();
        game.add_player(String::from("Bob")).unwrap();
        game.add_player(String::from("Neuromancer")).unwrap();

        assert_eq!(game.add_move(String::from("Alice"), ' ').unwrap_err(), Error::InvalidPosition);
    }

    #[test]
    fn add_move_checks_that_player_turn_is_correct() {
        let mut game = Game::new();
        game.add_player(String::from("Alice")).unwrap();
        game.add_player(String::from("Bob")).unwrap();
        game.add_player(String::from("Neuromancer")).unwrap();

        assert_eq!(game.add_move(String::from("Bob"), 'A').unwrap_err(), Error::PlayerMustWaitForTurn);
    }

    #[test]
    fn add_move_checks_that_position_is_valid() {
        let mut game = Game::new();
        game.add_player(String::from("Alice")).unwrap();
        game.add_player(String::from("Bob")).unwrap();
        game.add_player(String::from("Neuromancer")).unwrap();

        assert_eq!(game.add_move(String::from("Another player"), 'A').unwrap_err(), Error::PlayerNotFound);
    }

    #[test]
    fn add_move_checks_if_game_is_over() {
        let mut game = Game::new();
        game.add_player(String::from("Alice")).unwrap();
        game.add_player(String::from("Bob")).unwrap();
        game.add_player(String::from("Neuromancer")).unwrap();

        game.add_move(String::from("Alice"), 'A').unwrap();
        game.add_move(String::from("Bob"), 'H').unwrap();
        game.add_move(String::from("Neuromancer"), 'G').unwrap();
        game.add_move(String::from("Alice"), '*').unwrap();
        game.add_move(String::from("Bob"), 'I').unwrap();
        game.add_move(String::from("Neuromancer"), 'F').unwrap();
        game.add_move(String::from("Alice"), 'V').unwrap();

        assert_eq!(game.add_move(String::from("Bob"), 'B').unwrap_err(), Error::GameIsOver);
    }
}
