pub mod body;
pub mod message;
pub mod node;
pub mod r#type;

use serde::Serialize;
use std::io::{self, StdoutLock, Write};

pub(crate) fn serialize_to_stdout<T: Serialize>(
    value: &T,
    stdout_lock: &mut StdoutLock,
) -> io::Result<()> {
    serde_json::to_writer(&mut *stdout_lock, value)?;
    stdout_lock.write_all(b"\n")?;
    Ok(())
}
