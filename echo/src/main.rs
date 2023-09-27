use anyhow::{Context, Error};
use echo::{
    message::Message,
    node::{EchoNode, Node},
};
use std::io::{self};

fn main() -> Result<(), Error> {
    let stdin = io::stdin();
    let inputs = serde_json::Deserializer::from_reader(stdin.lock()).into_iter::<Message>();

    let stdout = io::stdout();
    let mut stdout_lock = stdout.lock();

    let mut echo_node = EchoNode::new();

    for input in inputs {
        let message = input.context("Failed to deserialize STDIN input message")?;
        echo_node.step(message, &mut stdout_lock)?;
    }

    Ok(())
}
