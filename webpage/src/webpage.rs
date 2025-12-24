use crate::scanner::{ScanError, Scanner};
use reqwest::header::USER_AGENT;
use std::error::Error;
use std::fmt::{Display, Formatter, write};

const SCOUT_USER_AGENT: &str = "Scout/0.1";

/// Represents failures to parse a webpage.
#[derive(Debug)]
pub enum ParseError {
    /// No data available for parsing.
    MissingData,

    /// Something went wrong during the scanning process.
    ScanError(ScanError),
}

/// [Webpage] provides a unified interface to fetch and parse raw HTML from the internet.
#[derive(Debug)]
pub struct Webpage {
    uri: String,
    raw_html: Option<String>,

    client: reqwest::blocking::Client,
}

impl Webpage {
    /// Creates a new [Webpage] instance.
    pub fn new(uri: String) -> Self {
        Self {
            uri,
            raw_html: None,
            client: reqwest::blocking::Client::new(),
        }
    }

    /// Fetch raw HTML from the URI passed to the [Webpage::new] function.
    pub fn fetch(&mut self) -> Result<(), Box<dyn Error>> {
        let html = self
            .client
            .get(&self.uri)
            .header(USER_AGENT, SCOUT_USER_AGENT)
            .send()?
            .text()?;

        self.raw_html = Some(html);
        Ok(())
    }

    /// Parses a webpage's raw HTML code into an abstract syntax tree (AST).
    ///
    /// It is assumed that websites have valid source code. If the website's source code is invalid,
    /// it is considered a "skill issue" for whoever built the website. Their inability to write
    /// valid HTML is not this browser's concern.
    ///
    /// Parser method will simply error out and refuse to parse the source code.
    pub fn parse(&mut self) -> Result<(), ParseError> {
        let source_opt = self.raw_html.clone();

        if source_opt.is_none() {
            return Err(ParseError::MissingData);
        }

        let source = source_opt.unwrap();

        let scanner = Scanner::new();
        let tokens = match scanner.scan_source_code(source.as_str()) {
            Ok(t) => t,
            Err(e) => return Err(ParseError::ScanError(e)),
        };

        if tokens.is_empty() {
            println!("no tokens found in source code")
        } else {
            println!("tokens: ");
            for token in tokens {
                println!("  {token}")
            }
        }

        Ok(())
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "failed to parse html source code: ")?;

        match self {
            Self::MissingData => {
                write!(f, "no html found - make sure to call Webpage::parse")
            }
            Self::ScanError(e) => write!(f, "failed to scan source code: {e}"),
        }
    }
}

impl Error for ParseError {}
