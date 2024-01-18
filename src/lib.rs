mod board;
mod errors;
pub mod game;
mod winning_combinations;
mod z3;
mod z3xz3xz3;

use crate::game::Game;

/// Create an new [Game].
///
/// ```
/// let tris3d = tris3d::new_game();
/// ```
#[must_use]
pub fn new_game() -> Game {
    Game::new()
}
