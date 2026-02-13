use std::io::Error as IoError;
use thiserror::Error as ThisError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, ThisError)]
pub enum Error {
	#[error(transparent)]
	IoError(#[from] IoError),
	#[error("Inconsistency between line shape ,arrow shape and contents")]
	InconsistencyShapes,
}
