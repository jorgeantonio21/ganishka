use crate::{body::Body, message::Message, r#type::Type};
use anyhow::{Context, Error};
use serde::Serialize;
use std::io::StdoutLock;

pub trait Node {
    fn step(
        &mut self,
        message: Message,
        output: &mut serde_json::Serializer<StdoutLock>,
    ) -> Result<(), Error>;
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
    fn step(
        &mut self,
        message: Message,
        output: &mut serde_json::Serializer<StdoutLock>,
    ) -> Result<(), Error> {
        match message.body.r#type {
            Type::EchoOk { .. } => {}
            Type::Echo { echo } => {
                let reply = Message {
                    dest: message.src.clone(),
                    src: message.dest.clone(),
                    body: Body {
                        msg_id: Some(self.id),
                        in_reply_to: message.body.msg_id,
                        r#type: crate::r#type::Type::EchoOk { echo: echo.clone() },
                    },
                };
                reply
                    .serialize(output)
                    .context("Failed to serialize EchoOk response.")?;
                self.id += 1;
            }
        };

        Ok(())
    }
}
