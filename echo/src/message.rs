use crate::body::Body;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Message {
    src: String,
    dest: String,
    body: Body,
}
