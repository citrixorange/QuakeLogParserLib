use serde_json::Value;
use std::future::Future;
use std::pin::Pin;

use crate::errors::LogParserError;

pub type LogParserCallBack = dyn Fn(Option<Value>) -> Pin<Box<dyn Future<Output = Result<(), LogParserError>> + Send + Sync >> + Send + Sync;

pub trait ILogParser {
    fn register_success_callback(&mut self, callback: Box<LogParserCallBack>);
    fn register_warning_callback(&mut self, callback: Box<LogParserCallBack>);
    fn register_error_callback(&mut self, callback: Box<LogParserCallBack>);
    fn parse_file(&mut self) -> Pin<Box<dyn Future<Output = Result<String, LogParserError>> + '_>>;
}
