use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum LogParserError {
    RegexParserError,
    ReadFileError,
    SerializationError,
    StringfyError,
    UnexpectedError,
}


impl fmt::Display for LogParserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LogParserError::RegexParserError => write!(f,"An error has happened on Regex Parsing Step..."),            
            LogParserError::ReadFileError => write!(f,"An error has happened Reading Log File..."),
            LogParserError::SerializationError => write!(f,"An error has happened on Serialization..."),
            LogParserError::StringfyError => write!(f,"An error has happened on Stringfication Process..."),
            LogParserError::UnexpectedError => write!(f,"An unexpected error has happened on Log Parsing..."),
        }
    }
}

impl Error for LogParserError {
    fn description(&self) -> &str {
        match self {
            LogParserError::RegexParserError => "An error has happened on Regex Parsing Step...",
            LogParserError::ReadFileError => "The following Error {} has happened Reading Log File...",
            LogParserError::SerializationError => "An error has happened on Serialization...",
            LogParserError::StringfyError => "An error has happened on Stringfication Process...",
            LogParserError::UnexpectedError => "An unexpected error has happened on Log Parsing...",
        }
    }
}

impl From<LogParserError> for &'static str {
    fn from(error: LogParserError) -> &'static str {
        match error {
            LogParserError::RegexParserError => "LogParserError::RegexParserError",
            LogParserError::ReadFileError => "LogParserError::ReadFileError",
            LogParserError::SerializationError => "LogParserError::SerializationError",
            LogParserError::StringfyError => "LogParserError::StringfyError",
            LogParserError::UnexpectedError => "LogParserError::UnexpectedError",
        }
    }
}