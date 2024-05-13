# maelstrom-rs
maelstrom-rs is an implementation of the RPC protocol defined [here](https://github.com/jepsen-io/maelstrom/blob/main/doc/protocol.md) used to build distributed systems. This project also includes my implementation of the distributed systems challenges found [here](https://fly.io/dist-sys/1/).

## Usage
```
git clone https://github.com/jgage930/maelstrom-rs
cd maelstrom-rs
cargo run --bin hello
```

The protocol reads input from stdin, writes output to stdout, and writes errors to stderr.
