//! Logger for the merge core

// TODO: improve this logger

use log::{debug, error, info, trace, warn};
use std::path::Path;

#[derive(Debug)]
struct Logger {
    destination: Destination,
    options: Option<LoggerOptions>,
}

#[derive(Debug)]
enum Destination {
    StdOut,
    StdErr,
    File(Box<Path>),
}

#[derive(Debug)]
struct LoggerOptions {
    beautify: bool,
    verbose: bool,
    debug: bool,
    trace: bool,
    warn: bool,
    error: bool,
}

impl Logger {
    fn new(destination: Destination) -> Self {
        Self {
            destination,
            options: None,
        }
    }

    fn set_options(&mut self, options: LoggerOptions) {
        self.options = Some(options);
    }
}
