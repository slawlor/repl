# rustyrepl
The easy Rust Read-Evaluate-Print-Loop (REPL) utility crate

## About

`rustyrepl` is a simple crate to facilitate creation of Read, Evaluate, Print, Loop utilities at the command-line.

## Purpose
1. Capturing exits/quits of the REPL interface
2. Storing and managing REPL history commands as well as an index of said commands for you
3. Allowing operators to get a help menu at any point, using the full Clap supported help interface (i.e. sub-command help as well)
4. Processing the commands as incoming
# Usage
First, add rustyrepl to your Cargo.toml file
```toml
[dependencies]
rustyrepl = "0.1"
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
