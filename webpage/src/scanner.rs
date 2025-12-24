use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialOrd, PartialEq)]
pub enum Token {}

#[derive(Debug)]
pub enum ScanError {}

enum State {
    Data,
}

/// HTML tokenizer whose implementation is based on
/// https://html.spec.whatwg.org/multipage/parsing.html#tokenization
pub struct Scanner {
    current_state: State,
}

impl Scanner {
    pub fn new() -> Scanner {
        Scanner {
            current_state: State::Data,
        }
    }

    pub fn scan_source_code(&self, source: &str) -> Result<Vec<Token>, ScanError> {
        Ok(vec![])
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            _ => write!(f, "unknown"),
        }
    }
}

impl Display for ScanError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            _ => write!(f, "unknown error"),
        }
    }
}

impl Error for ScanError {}
