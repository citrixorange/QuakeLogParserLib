use crate::interface::{ LogParserCallBack, ILogParser };
use crate::errors::LogParserError;

use std::future::Future;
use std::pin::Pin;

pub struct LogParser {
    log_parser: Box<dyn ILogParser>
}

impl LogParser {
    pub fn new(log_parser: Box<dyn ILogParser>) -> Self {
        Self { log_parser }
    }

    pub fn register_success_callback(&mut self, callback: Box<LogParserCallBack>) {
        return self.log_parser.register_success_callback(callback);
    }

    pub fn register_warning_callback(&mut self, callback: Box<LogParserCallBack>) {
        return self.log_parser.register_warning_callback(callback);
    }

    pub fn register_error_callback(&mut self, callback: Box<LogParserCallBack>) {
        return self.log_parser.register_error_callback(callback);
    }

    pub fn parse_file(&mut self) -> Pin<Box<dyn Future<Output = Result<String, LogParserError>> + '_>> {
        return self.log_parser.parse_file();
    }
}