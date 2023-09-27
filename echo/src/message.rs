use crate::body::Body;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Message {
    pub src: String,
    pub dest: String,
    pub body: Body,
}

impl Message {
    pub fn new(src: String, dest: String, body: Body) -> Self {
        Self { src, dest, body }
    }
}
