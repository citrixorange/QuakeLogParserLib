use std::error::{Error};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
pub enum LogParserError {
    UnexpectedError
}

impl fmt::Display for LogParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LogParserError::UnexpectedError => write!(f,"An unexpected error has happened on Log Parsing...")
        }
    }
}

impl Error for LogParserError {
    fn description(&self) -> &str {
        match self {
            LogParserError::UnexpectedError => "An unexpected error has happened on Log Parsing..."
        }
    }
}

impl FromStr for LogParserError {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "\"LogParserError::UnexpectedError\"" => Ok(LogParserError::UnexpectedError),
            _ => Err(())
        }
    }
}