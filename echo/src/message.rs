use crate::body::Body;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Message {
    pub(crate) src: String,
    pub(crate) dest: String,
    pub(crate) body: Body,
}

impl Message {
    pub fn new(src: String, dest: String, body: Body) -> Self {
        Self { src, dest, body }
    }
}
