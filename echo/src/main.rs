use anyhow::{bail, Context, Error};
use echo::{
    body::Body,
    event::Event,
    message::Message,
    node::{BroadcastNode, Node},
    r#type::Type,
    utils::serialize_to_stdout,
};
use std::{
    io::{self, BufRead},
    sync::mpsc,
    thread,
};

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

    let (tx, rx) = mpsc::channel();

    let mut broadcast_node = if let Type::Init(init) = init_message.body.r#type {
        BroadcastNode::init(init, tx.clone())
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
    drop(lines);
    let join_handle = thread::spawn(move || {
        let stdin_lock = io::stdin().lock();
        let lines = stdin_lock.lines();

        for line_result in lines {
            let line = line_result.context("Failed to deserialize STDIN input message")?;
            let message: Message =
                serde_json::from_str(&line).context("STDIN input could not be deserialized")?;
            tx.send(Event::Message(message))
                .context("Failed to send new event message")?;
            // broadcast_node.step(message, &mut stdout_lock)?;
        }

        Ok::<(), anyhow::Error>(())
    });

    while let Ok(event) = rx.recv() {
        match event {
            Event::StartGossip => {
                let message = Message {
                    dest: broadcast_node.node_id.clone(),
                    src: broadcast_node.node_id.clone(),
                    body: Body {
                        in_reply_to: None,
                        msg_id: None,
                        r#type: Type::Gossip,
                    },
                };
                broadcast_node.step(message, &mut stdout_lock)?;
            }
            Event::Message(message) => {
                broadcast_node.step(message, &mut stdout_lock)?;
            }
        }
    }

    join_handle.join().expect("Failed to run thread")?;

    // for line_result in lines {
    //     let line = line_result.context("Failed to deserialize STDIN input message")?;
    //     let message: Message =
    //         serde_json::from_str(&line).context("STDIN input could not be deserialized")?;
    //     broadcast_node.step(message, &mut stdout_lock)?;
    // }

    Ok(())
}
