// Copyright (c) Sean Lawlor
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

//! A test-utility for the REPL crate for manual testing. You can execute
//! the test utility by running
//!
//! ```bash
//! cargo run
//! ```
//!
//! in your terminal

use anyhow::Result;
use clap::{Parser, Subcommand};
use rustyrepl::{Repl, ReplCommandProcessor};

mod console_log;

static LOGGER: console_log::ConsoleLogger = console_log::ConsoleLogger {
    level: log::Level::Info,
};

/// The enum of sub-commands supported by the CLI
#[derive(Subcommand, Clone, Debug)]
pub enum Command {
    /// Execute a test command
    Test,
}

/// The general CLI, essentially a wrapper for the sub-commands [Command]
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
    console_log::ConsoleLogger::touch();

    // initialize the logger
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(LOGGER.level.to_level_filter()))
        .expect("Failed to set up logging");

    let processor: Box<dyn ReplCommandProcessor<Cli>> = Box::new(CliProcessor {});
    // create a dummy test history file
    let history_file = ".test_history".to_string();
    let some_history_file = Some(history_file);

    let mut repl = Repl::<Cli>::new(processor, some_history_file, Some(">> ".to_string()))?;
    repl.process().await
}
