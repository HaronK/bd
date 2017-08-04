
use types::errors;

pub struct Error<T>(Option<T>, errors::Error);

/// `PartialResult` is a type that can contain value and error at the same time.
pub type PartialResult<T> = Result<T, Error<T>>;
