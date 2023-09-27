use anyhow::{bail, Context, Error};
use echo::{
    body::Body,
    message::Message,
    node::{BroadcastNode, Node},
    r#type::Type,
    utils::serialize_to_stdout,
};
use std::io::{self, BufRead};

fn main() -> Result<(), Error> {
    let stdin_lock = io::stdin().lock();
    let mut lines = stdin_lock.lines();

    let stdout = io::stdout();
    let mut stdout_lock = stdout.lock();

    let init_message: Message = serde_json::from_str(
        &lines
            .next()
            .expect("Failed to extract first message")
            .context("failed to read init message from stdin")?,
    )?;

    let mut broadcast_node = if let Type::Init(init) = init_message.body.r#type {
        BroadcastNode::init(init)
    } else {
        bail!("Failed to initialize broadcast node");
    };

    let init_reply = Message {
        src: init_message.dest,
        dest: init_message.src,
        body: Body {
            msg_id: Some(0),
            in_reply_to: init_message.body.msg_id,
            r#type: Type::InitOk,
        },
    };

    serialize_to_stdout(&init_reply, &mut stdout_lock)
        .context("Failed to write init reply to STDOUT")?;

    for line_result in lines {
        let line = line_result.context("Failed to deserialize STDIN input message")?;
        let message: Message =
            serde_json::from_str(&line).context("STDIN input could not be deserialized")?;
        broadcast_node.step(message, &mut stdout_lock)?;
    }

    Ok(())
}
