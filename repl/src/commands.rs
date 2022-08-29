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
    async fn process_command(&self, command: C) -> Result<()>;
}
