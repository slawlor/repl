
# rustyrepl

The easy Rust Read-Evaluate-Print-Loop (REPL) utility crate

[<img alt="github" src="https://img.shields.io/badge/github-slawlor/repl-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/slawlor/repl)
[<img alt="crates.io" src="https://img.shields.io/crates/v/rustyrepl.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/rustyrepl)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-rustyrepl-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/rustyrepl)
[![CI/main](https://github.com/slawlor/repl/actions/workflows/ci.yaml/badge.svg?branch=main)](https://github.com/slawlor/repl/actions/workflows/ci.yaml)

## About

`rustyrepl` is a simple crate to facilitate creation of Read, Evaluate, Print, Loop utilities at the command-line. A unique combination of `rustyline` and `clap` to build a simple REPL interface
with handy argument parsing.

## Purpose

1. Capturing exits/quits of the REPL interface
2. Storing and managing REPL history commands as well as an index of said commands for you
3. Allowing operators to get a help menu at any point, using the full Clap supported help interface (i.e. sub-command help as well)
4. Processing the commands as incoming

# Usage

First, add rustyrepl to your `Cargo.toml` file

```toml
[dependencies]
rustyrepl = "0.2"
```

Next:

```rust
use anyhow::Result;
use clap::{Parser, Subcommand};
use rustyrepl::{Repl, ReplCommandProcessor};
/// The enum of sub-commands supported by the CLI
#[derive(Subcommand, Clone, Debug)]
pub enum Command {
    /// Execute a test command
    Test,
}
/// The general CLI, essentially a wrapper for the sub-commands [Commands]
#[derive(Parser, Clone, Debug)]
pub struct Cli {
    #[clap(subcommand)]
    command: Command,
}
#[derive(Debug)]
pub struct CliProcessor {}
#[async_trait::async_trait]
impl ReplCommandProcessor<Cli> for CliProcessor {
    fn is_quit(&self, command: &str) -> bool {
        matches!(command, "quit" | "exit")
    }
    async fn process_command(&self, command: Cli) -> Result<()> {
        match command.command {
            Command::Test => println!("A wild test appeared!"),
        }
        Ok(())
    }
}
// MAIN //
#[tokio::main]
async fn main() -> Result<()> {
    let processor: Box<dyn ReplCommandProcessor<Cli>> = Box::new(CliProcessor {});
    let mut repl = Repl::<Cli>::new(processor, None, Some(">>".to_string()))?;
    repl.process().await
}
```

This small program will startup up a REPL with the prompt ">>" which you can interact with

```text
>> help
The general CLI, essentially a wrapper for the sub-commands [Commands]

Usage: repl-interface <COMMAND>

Commands:
  test  Execute a test command
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help

```
