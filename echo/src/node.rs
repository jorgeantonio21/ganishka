use std::{collections::HashSet, io::StdoutLock};

use crate::{
    body::Body,
    message::Message,
    r#type::{Init, Type},
    utils::{generate_id, serialize_to_stdout},
};
use anyhow::{bail, Error};

pub trait Node {
    fn init(init: Init) -> Self;
    fn step(&mut self, message: Message, stdout_lock: &mut StdoutLock) -> Result<(), Error>;
}

pub struct BroadcastNode {
    node_id: String,
    id: usize,
    received_messages: HashSet<usize>,
    node_neighborhood: Vec<String>,
}

impl Node for BroadcastNode {
    fn init(init: Init) -> Self {
        Self {
            node_id: init.node_id,
            id: 0,
            received_messages: HashSet::new(),
            node_neighborhood: Vec::new(),
        }
    }

    fn step(&mut self, message: Message, stdout_lock: &mut StdoutLock) -> Result<(), Error> {
        match message.body.r#type {
            Type::Init(..) => {
                bail!("Already initialized");
            }
            Type::InitOk => {
                bail!("Unexpected type InitOk")
            }
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
            Type::Broadcast { message: msg } => {
                self.received_messages.insert(msg);
                let reply = Message {
                    dest: message.src,
                    src: message.dest,
                    body: Body {
                        msg_id: Some(self.id),
                        in_reply_to: message.body.msg_id,
                        r#type: crate::r#type::Type::BroadcastOk,
                    },
                };
                serialize_to_stdout(&reply, stdout_lock)?;
                self.id += 1;
            }
            Type::Read => {
                let reply = Message {
                    dest: message.src,
                    src: message.dest,
                    body: Body {
                        msg_id: Some(self.id),
                        in_reply_to: message.body.msg_id,
                        r#type: crate::r#type::Type::ReadOk {
                            messages: self.received_messages.clone(),
                        },
                    },
                };
                serialize_to_stdout(&reply, stdout_lock)?;
                self.id += 1;
            }
            Type::Topology { mut topology } => {
                self.node_neighborhood = topology.remove(&self.node_id).unwrap_or(vec![]);
                let reply = Message {
                    dest: message.src,
                    src: message.dest,
                    body: Body {
                        msg_id: Some(self.id),
                        in_reply_to: message.body.msg_id,
                        r#type: crate::r#type::Type::TopologyOk,
                    },
                };
                serialize_to_stdout(&reply, stdout_lock)?;
                self.id += 1;
            }
            Type::TopologyOk
            | Type::GenerateOk { .. }
            | Type::BroadcastOk
            | Type::ReadOk { .. }
            | Type::EchoOk { .. } => {}
        };

        Ok(())
    }
}
