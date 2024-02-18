use std::error::{Error};
use std::fmt;
use std::io::{self};

#[derive(Debug)]
pub enum LogParserError {
    UnexpectedError,
    ReadFileError(io::Error)
}

impl From<io::Error> for LogParserError {
    fn from(error: io::Error) -> Self {
        LogParserError::ReadFileError(error)
    }
}

impl fmt::Display for LogParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LogParserError::UnexpectedError => write!(f,"An unexpected error has happened on Log Parsing..."),
            LogParserError::ReadFileError(err) => write!(f,"The following Error {} has happened Reading Log File...", err),
        }
    }
}

impl Error for LogParserError {
    fn description(&self) -> &str {
        match self {
            LogParserError::UnexpectedError => "An unexpected error has happened on Log Parsing...",
            LogParserError::ReadFileError(_err) => "The following Error {} has happened Reading Log File...",
        }
    }
}