use std::io::StdoutLock;

use crate::{
    body::Body,
    message::Message,
    r#type::Type,
    utils::{generate_id, serialize_to_stdout},
};
use anyhow::{bail, Error};

pub trait Node {
    fn step(&mut self, message: Message, stdout_lock: &mut StdoutLock) -> Result<(), Error>;
}

pub struct EchoNode {
    id: usize,
}

impl EchoNode {
    pub fn new() -> Self {
        Self { id: 0 }
    }
}

impl Node for EchoNode {
    fn step(&mut self, message: Message, stdout_lock: &mut StdoutLock) -> Result<(), Error> {
        match message.body.r#type {
            Type::Init { .. } => {
                let reply = Message {
                    dest: message.src,
                    src: message.dest,
                    body: Body {
                        msg_id: Some(self.id),
                        in_reply_to: message.body.msg_id,
                        r#type: crate::r#type::Type::InitOk,
                    },
                };
                serialize_to_stdout(&reply, stdout_lock)?;
                self.id += 1;
            }
            Type::InitOk => {
                bail!("Unexpected InitOk message")
            }
            Type::EchoOk { .. } => {}
            Type::Echo { echo } => {
                let reply = Message {
                    dest: message.src,
                    src: message.dest,
                    body: Body {
                        msg_id: Some(self.id),
                        in_reply_to: message.body.msg_id,
                        r#type: crate::r#type::Type::EchoOk { echo: echo.clone() },
                    },
                };
                serialize_to_stdout(&reply, stdout_lock)?;
                self.id += 1;
            }
            Type::Generate => {
                let id = generate_id();
                let reply = Message {
                    dest: message.src,
                    src: message.dest,
                    body: Body {
                        msg_id: Some(self.id),
                        in_reply_to: message.body.msg_id,
                        r#type: crate::r#type::Type::GenerateOk { id },
                    },
                };
                serialize_to_stdout(&reply, stdout_lock)?;
                self.id += 1;
            }
            Type::GenerateOk { .. } => {
                bail!("Unexpected GenerateOk message")
            }
        };

        Ok(())
    }
}
