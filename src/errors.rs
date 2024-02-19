use serde::{Deserialize, Serialize};
use std::error::{Error};
use std::fmt;
use std::io::{self};

#[derive(Debug)]
pub enum LogParserError {
    RegexParserError,
    ReadFileError(io::Error),
    UnexpectedError,
}

impl From<io::Error> for LogParserError {
    fn from(error: io::Error) -> Self {
        LogParserError::ReadFileError(error)
    }
}

impl fmt::Display for LogParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LogParserError::RegexParserError => write!(f,"An error has happened on Regex Parsing Step..."),            
            LogParserError::ReadFileError(err) => write!(f,"The following Error {} has happened Reading Log File...", err),
            LogParserError::UnexpectedError => write!(f,"An unexpected error has happened on Log Parsing..."),
        }
    }
}

impl Error for LogParserError {
    fn description(&self) -> &str {
        match self {
            LogParserError::RegexParserError => "An error has happened on Regex Parsing Step...",
            LogParserError::ReadFileError(_err) => "The following Error {} has happened Reading Log File...",
            LogParserError::UnexpectedError => "An unexpected error has happened on Log Parsing...",
        }
    }
}

impl From<LogParserError> for &'static str {
    fn from(error: LogParserError) -> &'static str {
        match error {
            LogParserError::RegexParserError => "LogParserError::RegexParserError",
            LogParserError::ReadFileError(_err) => "LogParserError::ReadFileError",
            LogParserError::UnexpectedError => "LogParserError::UnexpectedError",
        }
    }
}