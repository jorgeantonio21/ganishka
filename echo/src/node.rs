use std::{collections::HashSet, io::StdoutLock};

use crate::{
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
            node_neighborhood: init.node_ids,
        }
    }

    fn step(&mut self, message: Message, stdout_lock: &mut StdoutLock) -> Result<(), Error> {
        let mut reply = message.into_reply(&mut self.id);
        match reply.body.r#type {
            Type::Init(..) => {
                bail!("Already initialized");
            }
            Type::InitOk => {
                bail!("Unexpected type InitOk")
            }
            Type::Echo { echo } => {
                reply.body.r#type = Type::EchoOk { echo: echo.clone() };
                serialize_to_stdout(&reply, stdout_lock)?;
                self.id += 1;
            }
            Type::Generate => {
                let id = generate_id();
                reply.body.r#type = Type::GenerateOk { id };
                serialize_to_stdout(&reply, stdout_lock)?;
                self.id += 1;
            }
            Type::Broadcast { message: msg } => {
                self.received_messages.insert(msg);
                reply.body.r#type = Type::BroadcastOk;
                serialize_to_stdout(&reply, stdout_lock)?;
                self.id += 1;
            }
            Type::Read => {
                reply.body.r#type = Type::ReadOk {
                    messages: self.received_messages.clone(),
                };
                serialize_to_stdout(&reply, stdout_lock)?;
                self.id += 1;
            }
            Type::Topology { mut topology } => {
                self.node_neighborhood = topology
                    .remove(&self.node_id)
                    .unwrap_or_else(|| panic!("No topology provided, in received message"));
                reply.body.r#type = Type::TopologyOk;
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
