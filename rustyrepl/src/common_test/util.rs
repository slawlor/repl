// Copyright (c) Sean Lawlor
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use super::console_logger::*;
use log::Level;
use std::sync::Once;
use tokio::time::Instant;

static LOGGER: ConsoleLogger = ConsoleLogger {};
static INIT_ONCE: Once = Once::new();

/// Initialize the logger for console logging within test environments.
/// This is safe to call multiple times, but it will only initialize the logger
/// to the log-level _first_ set. If you want a specific log-level (e.g. Debug)
/// for a specific test, make sure to only run that single test after editing that
/// test's log-level.
///
/// The default level applied everywhere is Info
pub fn init_logger(level: Level) {
    EPOCH.get_or_init(Instant::now);

    INIT_ONCE.call_once(|| {
        log::set_logger(&LOGGER)
            .map(|()| log::set_max_level(level.to_level_filter()))
            .unwrap();
    });
}

/// Global test startup constructor. Only runs in the TEST profile. Each
/// crate which wants logging enabled in tests being run should make this call
/// itself.
#[cfg(test)]
#[ctor::ctor]
fn test_start() {
    init_logger(Level::Debug);
}
