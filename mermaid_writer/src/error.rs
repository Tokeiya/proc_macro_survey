use std::io::Error as IoError;
use thiserror::Error as ThisError;

pub type Result<T, K> = std::result::Result<T, Error<K>>;

#[derive(Debug, ThisError)]
pub enum Error<T> {
	#[error(transparent)]
	IoError(#[from] IoError),
	#[error("Node with same id already exists:{0}")]
	NodeAlreadyExists(T),
	#[error("Link with same id already exists,scr:{0},tgt:{1}")]
	LinkAlreadyExists(T, T),
}
