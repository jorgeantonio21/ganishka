use anyhow::Error;
use echo::message::Message;
use std::io::{self, BufRead, Write};

fn main() -> Result<(), Error> {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut stdout_lock = stdout.lock();

    let inputs = serde_json::Deserializer::from_reader(stdin.lock()).into_iter::<Message>();

    for line in stdin.lock().lines() {
        match line {
            Ok(input) => match serde_json::from_str::<serde_json::Value>(&input) {
                Ok(json) => {
                    writeln!(
                        stdout_lock,
                        "{}",
                        serde_json::to_string_pretty(&json).unwrap()
                    )
                    .unwrap();
                }
                Err(e) => eprintln!("Failed to parse JSON from input, with error: {}", e),
            },
            Err(e) => eprintln!("Failed to read input, with error: {}", e),
        }
    }

    Ok(())
}
