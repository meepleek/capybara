#[derive(Debug, PartialEq, Eq, derive_more::Error, derive_more::Display)]
pub enum PlaceError {
    Taken,
    OutOfBounds,
}
