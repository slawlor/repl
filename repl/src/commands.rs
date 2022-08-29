// Copyright (c) Sean Lawlor
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

/// Represents a processor of REPL commands from a
/// [crate::repl::Repl<C>]
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait ReplCommandProcessor<C>: std::fmt::Debug
where
    C: clap::Parser,
{
    /// Process the supplied command which is a clap::Parser structure
    async fn process_command(&self, command: C) -> Result<()>;

    /// Determine if the supplied command is a "quit" operation. This will be
    /// formatted into a trimmed string of lowercase letters. Example matching might
    /// check (in English)
    ///
    /// ```rust
    /// match command {
    ///     "quit" | "exit" => true,
    ///     _ => false,
    /// }
    /// ```
    fn is_quit(&self, command: &str) -> bool;
}
