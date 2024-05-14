use anyhow::{Context, Result};
use maelstrom_rs::node::echo::{EchoMessage, EchoNode};
use maelstrom_rs::node::traits::Node;
use serde_json;
use std::io;

/// A simple Echo server.  Instantiate a single echo node and echo back a message.
fn main() -> Result<()> {
    main_loop().context("Failed to run echo server")?;

    Ok(())
}

fn read_stdin() -> String {
    io::stdin()
        .lines()
        .into_iter()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect::<Vec<String>>()
        .join("\n")
}

fn main_loop() -> Result<()> {
    let node = EchoNode;
    loop {
        let stdin = read_stdin();

        let input: EchoMessage =
            serde_json::from_str(&stdin).context("Failed to read message from stdin")?;
        let output = node.respond(input)?;

        let mut stdout = io::stdout();
        output
            .reply(&mut stdout)
            .context("Failed to write response to stdout.")?;
    }
}
