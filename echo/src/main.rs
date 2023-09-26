use anyhow::{Context, Error};
use echo::{
    message::Message,
    node::{EchoNode, Node},
};
use std::io::{self};

fn main() -> Result<(), Error> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let stdout_lock = stdout.lock();
    let mut output = serde_json::Serializer::new(stdout_lock);

    let mut echo_node = EchoNode::new();

    let inputs = serde_json::Deserializer::from_reader(stdin.lock()).into_iter::<Message>();
    for input in inputs {
        let message = input.context("Failed to deserialize STDIN input message")?;
        echo_node.step(message, &mut output)?;
    }

    Ok(())
}
