use std::future::Future;
use std::pin::Pin;
use std::io::{self, BufRead};

use crate::interface::{ILogParser, LogParserCallBack};
use crate::errors::LogParserError;

pub struct ConcreteLogParser {
    callback: Option<Box<LogParserCallBack>>
}

impl ConcreteLogParser {
    pub fn new() -> Self {
        Self {
            callback: None
        }
    }

    fn parse_log_line(line: &str) {
        println!("{}", line);
    }
}

impl ILogParser for ConcreteLogParser {
    fn register_callback(&mut self, callback: Box<LogParserCallBack>) {
        self.callback = Some(callback);
    }

    fn parse_file(&self) -> Pin<Box<dyn Future<Output = Result<(), LogParserError>> + '_>> {
        let future = async {
            let input = std::fs::File::open("sample_log.log")?;
            let reader = io::BufReader::new(input);
        
            for line in reader.lines() {
                let line = line?;
                ConcreteLogParser::parse_log_line(&line)
            }
            return Ok(());
        };

        return Box::pin(future);
    }
}