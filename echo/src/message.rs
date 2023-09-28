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

    pub fn into_reply(self, msg_id: &mut usize) -> Self {
        let reply = Self {
            dest: self.src,
            src: self.dest,
            body: Body {
                msg_id: Some(*msg_id),
                in_reply_to: self.body.msg_id,
                r#type: self.body.r#type,
            },
        };
        *msg_id += 1;
        reply
    }
}
