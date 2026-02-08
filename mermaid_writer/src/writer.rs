use crate::prelude::*;
use std::io::Write as IoWrite;
pub trait Writer<W: IoWrite> {
	fn write(&self, writer: &mut W) -> Result<()>;
}
