use crate::r#type::Type;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Body {
    pub msg_id: Option<usize>,
    pub in_reply_to: Option<usize>,
    #[serde(flatten)]
    pub r#type: Type,
}

impl Body {
    pub fn new(msg_id: Option<usize>, in_reply_to: Option<usize>, r#type: Type) -> Self {
        Self {
            msg_id,
            in_reply_to,
            r#type,
        }
    }
}
