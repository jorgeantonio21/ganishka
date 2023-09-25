use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Body {
    r#type: String,
    msg_id: u32,
    echo: String,
}
