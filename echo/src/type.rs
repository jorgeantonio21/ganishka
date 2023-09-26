use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum Type {
    Init,
    InitOk,
    Echo { echo: String },
    EchoOk,
}
