#[derive(Debug, PartialEq)]
pub enum Error {
    BoardIsFull,
    CannotAddMoreThanThreePlayers,
    CannotAddSamePlayerTwice,
    GameNotStartedYet,
    GameIsOver,
    InvalidPosition,
    PlayerNotFound,
    PositionAlreadyTaken,
    PositionsMustBeDistinct,
    ThereIsAlreadyAWinner,
}
