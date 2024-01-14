mod board;
mod game;
mod z3;

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
