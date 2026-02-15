use crate::prelude::Connection;
use std::io::Error as IoError;
use thiserror::Error as ThisError;

pub type Result<T, K> = std::result::Result<T, Error<K>>;

#[derive(Debug, ThisError)]
pub enum Error<K> {
	#[error(transparent)]
	IoError(#[from] IoError),
	#[error("Node with same id already exists:{0}")]
	NodeAlreadyExists(K),
	#[error("Link with same id already exists")]
	LinkAlreadyExists(Connection<K>),
	#[error("Key not found:{0}")]
	KeyNotFound(K),
	#[error("Source and target keys are the same:{0}")]
	ScrTgtSameKey(K),
}
