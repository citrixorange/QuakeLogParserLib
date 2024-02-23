use regex::Regex;
use once_cell::sync::Lazy;

use crate::config::static_config::{STATIC_CONFIG, StaticConfigParameter};
use crate::errors::LogParserError;

pub static INIT_GAME_EVENT_DETECT_REGEX: Lazy<Regex> = Lazy::new(|| { Regex::new(STATIC_CONFIG.get_parameter(StaticConfigParameter::InitGameEventRegex).to_string().as_str()).unwrap() });
pub static CLIENT_CONNECT_EVENT_DETECT_REGEX: Lazy<Regex> = Lazy::new(|| { Regex::new(STATIC_CONFIG.get_parameter(StaticConfigParameter::ClientConnectEventRegex).to_string().as_str()).unwrap() });
pub static CLIENT_INFO_CHANGE_EVENT_DETECT_REGEX: Lazy<Regex> = Lazy::new(|| { Regex::new(STATIC_CONFIG.get_parameter(StaticConfigParameter::ClientInfoChangeEventRegex).to_string().as_str()).unwrap() });
pub static CLIENT_BEGIN_EVENT_DETECT_REGEX: Lazy<Regex> = Lazy::new(|| { Regex::new(STATIC_CONFIG.get_parameter(StaticConfigParameter::ClientBeginEventRegex).to_string().as_str()).unwrap() });
pub static CLIENT_DISCONNECT_EVENT_DETECT_REGEX: Lazy<Regex> = Lazy::new(|| { Regex::new(STATIC_CONFIG.get_parameter(StaticConfigParameter::ClientDisconnectEventRegex).to_string().as_str()).unwrap() });
pub static ITEM_EVENT_DETECT_REGEX: Lazy<Regex> = Lazy::new(|| { Regex::new(STATIC_CONFIG.get_parameter(StaticConfigParameter::ItemEventRegex).to_string().as_str()).unwrap() });
pub static KILL_EVENT_DETECT_REGEX: Lazy<Regex> = Lazy::new(|| { Regex::new(STATIC_CONFIG.get_parameter(StaticConfigParameter::KillEventRegex).to_string().as_str()).unwrap() });
pub static SHUTDOWN_EVENT_DETECT_REGEX: Lazy<Regex> = Lazy::new(|| { Regex::new(STATIC_CONFIG.get_parameter(StaticConfigParameter::ShutdownEventRegex).to_string().as_str()).unwrap() });
pub static EXIT_EVENT_DETECT_REGEX: Lazy<Regex> = Lazy::new(|| { Regex::new(STATIC_CONFIG.get_parameter(StaticConfigParameter::ExitEventRegex).to_string().as_str()).unwrap() });
pub static KILL_PARSER_REGEX: Lazy<Regex> = Lazy::new(|| { Regex::new(STATIC_CONFIG.get_parameter(StaticConfigParameter::KillEventLineParserRegex).to_string().as_str()).unwrap() });
pub static USER_INFO_PARSER_REGEX: Lazy<Regex> = Lazy::new(|| { Regex::new(STATIC_CONFIG.get_parameter(StaticConfigParameter::UserInfoLineParserRegex).to_string().as_str()).unwrap() });

pub(crate) enum LogEvent {
    InitMatch,
    ClientConnect,
    ClientUserinfoChanged,
    ClientBegin,
    ClientDisconnect,
    Item,
    Kill,
    ShutdownGame,
    Exit
}

impl LogEvent {
    pub(crate) fn detect_line_log_event(log_line: &str) -> Result<Self, LogParserError> {
         if ITEM_EVENT_DETECT_REGEX.is_match(log_line) {
            return Ok(LogEvent::Item);
         } else if KILL_EVENT_DETECT_REGEX.is_match(log_line) {
            return Ok(LogEvent::Kill);
         } else if CLIENT_INFO_CHANGE_EVENT_DETECT_REGEX.is_match(log_line){
             return Ok(LogEvent::ClientUserinfoChanged);
         } else if CLIENT_CONNECT_EVENT_DETECT_REGEX.is_match(log_line) {
            return Ok(LogEvent::ClientConnect);
         } else if CLIENT_BEGIN_EVENT_DETECT_REGEX.is_match(log_line) {
            return Ok(LogEvent::ClientBegin);
         } else if CLIENT_DISCONNECT_EVENT_DETECT_REGEX.is_match(log_line) {
            return Ok(LogEvent::ClientDisconnect);
         } else if INIT_GAME_EVENT_DETECT_REGEX.is_match(log_line) {
            return Ok(LogEvent::InitMatch);
         } else if SHUTDOWN_EVENT_DETECT_REGEX.is_match(log_line) {
            return Ok(LogEvent::ShutdownGame);
         } else if EXIT_EVENT_DETECT_REGEX.is_match(log_line) {
            return Ok(LogEvent::Exit);
         } else {
            return Err(LogParserError::RegexParserError)
         }
    }
}
