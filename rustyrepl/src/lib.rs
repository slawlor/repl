// Copyright (c) Sean Lawlor
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

//! A simple read-evaluate-print-loop processor based on Clap and Rustyline for handling the
//! terminal input. You simply need to define your command structure in a [clap::Parser] derived
//! struct and the processing logic, and the Repl will handle
//!
//! 1. Capturing exits/quits of the REPL interface
//! 2. Storing and managing REPL history commands as well as an index of said commands for you
//! 3. Allowing operators to get a help menu at any point, using the full Clap supported help interface (i.e. sub-command help as well)
//! 4. Processing the commands as incoming
//!

mod commands;
mod repl;

pub use crate::commands::ReplCommandProcessor;
pub use crate::repl::Repl;
