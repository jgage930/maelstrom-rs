use anyhow::{Context, Result};
use maelstrom_rs::node::echo::{EchoMessage, EchoNode};
use maelstrom_rs::node::traits::Node;
use maelstrom_rs::read_stdin;
use serde_json;
use std::io;

/// A simple Echo server.  Instantiate a single echo node and echo back a message.
fn main() -> Result<()> {
    main_loop().context("Failed to run echo server")?;

    Ok(())
}

fn main_loop() -> Result<()> {
    let mut node = EchoNode;
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
