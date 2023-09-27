use serde::Serialize;
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    io::{self, StdoutLock, Write},
};
use uuid::Uuid;

pub(crate) fn serialize_to_stdout<T: Serialize>(
    value: &T,
    stdout_lock: &mut StdoutLock,
) -> io::Result<()> {
    serde_json::to_writer(&mut *stdout_lock, value)?;
    stdout_lock.write_all(b"\n")?;
    Ok(())
}

pub(crate) fn generate_id() -> usize {
    let mut hasher = DefaultHasher::new();
    let id = Uuid::new_v4();
    id.hash(&mut hasher);
    hasher.finish() as usize
}
