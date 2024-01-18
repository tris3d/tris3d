#[derive(Debug, PartialEq)]
pub enum Error {
    BoardIsFull,
    CannotAddMoreThanThreePlayers,
    CannotAddSamePlayerTwice,
    GameIsOver,
    GameNotStartedYet,
    InvalidPosition,
    PlayerMustWaitForTurn,
    PlayerNotFound,
    PositionAlreadyTaken,
    PositionsMustBeDistinct,
    ThereIsAlreadyAWinner,
}
