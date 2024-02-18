use crate::interface::{ LogParserCallBack, ILogParser };
use crate::errors::LogParserError;

use std::future::Future;
use std::pin::Pin;

pub struct LogParser<'a> {
    log_parser: &'a mut dyn ILogParser
}

impl <'a>LogParser<'a> {
    pub fn new(log_parser: &'a mut dyn ILogParser) -> Self {
        Self { log_parser }
    }

    pub fn register_callback(&mut self, callback: Box<LogParserCallBack>) {
        return self.log_parser.register_callback(callback);
    }

    pub fn parse_file(&self) -> Pin<Box<dyn Future<Output = Result<(), LogParserError>> + '_>> {
        return self.log_parser.parse_file();
    }
}