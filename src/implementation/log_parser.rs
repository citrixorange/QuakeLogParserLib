use std::future::Future;
use std::pin::Pin;
use std::io::{self, BufRead};
use serde::{Serialize, Deserialize, Serializer, Deserializer};
use serde_json::{json, Map, Value};
use std::collections::{HashMap, HashSet};
use regex::Regex;
use once_cell::sync::Lazy;

use crate::interface::{ILogParser, LogParserCallBack};
use crate::errors::LogParserError;

static INIT_GAME_EVENT_DETECT_REGEX: Lazy<Regex> = Lazy::new(|| { Regex::new(r#"\bInitGame\b"#).unwrap() });
static CLIENT_CONNECT_EVENT_DETECT_REGEX: Lazy<Regex> = Lazy::new(|| { Regex::new(r#"\bClientConnect\b"#).unwrap() });
static CLIENT_INFO_CHANGE_EVENT_DETECT_REGEX: Lazy<Regex> = Lazy::new(|| { Regex::new(r#"\bClientUserinfoChanged\b"#).unwrap() });
static CLIENT_BEGIN_EVENT_DETECT_REGEX: Lazy<Regex> = Lazy::new(|| { Regex::new(r#"\bClientBegin\b"#).unwrap() });
static CLIENT_DISCONNECT_EVENT_DETECT_REGEX: Lazy<Regex> = Lazy::new(|| { Regex::new(r#"\bClientDisconnect\b"#).unwrap() });
static ITEM_EVENT_DETECT_REGEX: Lazy<Regex> = Lazy::new(|| { Regex::new(r#"\bItem\b"#).unwrap() });
static KILL_EVENT_DETECT_REGEX: Lazy<Regex> = Lazy::new(|| { Regex::new(r#"\bKill\b"#).unwrap() });
static SHUTDOWN_EVENT_DETECT_REGEX: Lazy<Regex> = Lazy::new(|| { Regex::new(r#"\bShutdownGame\b"#).unwrap() });
static EXIT_EVENT_DETECT_REGEX: Lazy<Regex> = Lazy::new(|| { Regex::new(r#"\bExit\b"#).unwrap() });
static KILL_PARSER_REGEX: Lazy<Regex> = Lazy::new(|| { Regex::new(r#"(\d+|\d+\d+):(\d+|\d+\d+) Kill: \d+ \d+ \d+: ([a-zA-Z0-9\s\p{P}<>]*) killed ([a-zA-Z0-9\s\p{P}<>]*) by (\w+)"#).unwrap() });
static USER_INFO_PARSER_REGEX: Lazy<Regex> = Lazy::new(|| { Regex::new(r#"n\\([^\\]+)\\"#).unwrap() });

enum LogEvent {
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
    fn detect_line_log_event(log_line: &str) -> Result<Self, LogParserError> {
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

#[derive(Clone, Serialize, Deserialize)]
struct MatchesData {
    matches: HashMap<String, MatchData>
}

impl MatchesData {
    pub fn new() -> Self {
        Self {
            matches: HashMap::new()
        }
    }

    fn match_label(&self) -> String {
        let label: String = format!("game_{}", self.matches.len());
        return label;
    }

    pub fn register_new_match_stat(&mut self, match_stats: MatchData) {
        
        self.matches.insert(
            self.match_label(),
            match_stats
        );
    }
}

#[derive(Clone, Serialize, Deserialize)]
struct MatchData {
    total_kills: i32,
    players: HashSet<String>,
    kills: HashMap<String, i32>,
}

#[derive(Serialize, Deserialize)]
struct WarningCallbackPayload {
    error: &'static str,
    line: String,
}

pub struct ConcreteLogParser {
    success_callback: Option<Box<LogParserCallBack>>,
    warning_callback: Option<Box<LogParserCallBack>>,
    error_callback: Option<Box<LogParserCallBack>>,
    matches_data: MatchesData,
    current_match_data: MatchData,
    first_match: bool
}

impl ConcreteLogParser {
    pub fn new() -> Self {
        Self {
            success_callback: None,
            warning_callback: None,
            error_callback: None,
            matches_data: MatchesData::new(),
            current_match_data: MatchData {
                total_kills: 0,
                players: HashSet::new(),
                kills: HashMap::new()
            },
            first_match: true
        }
    }

    fn parse_log_line(&mut self, line: &str) -> Result<(), LogParserError> {

        match LogEvent::detect_line_log_event(line)? {

            LogEvent::InitMatch => {
                if self.first_match == false {

                    self.matches_data.register_new_match_stat(
            self.current_match_data.clone()
                    );
    
                    self.current_match_data = MatchData {
                        total_kills: 0,
                        players: HashSet::new(),
                        kills: HashMap::new()
                    };

                } else {
                    self.first_match = false;
                }

                return Ok(());
            },
            LogEvent::ClientConnect => {
                return Ok(());
            },
            LogEvent::ClientBegin => {
                return Ok(());
            },
            LogEvent::ClientUserinfoChanged => {
                if let Some(captures) = USER_INFO_PARSER_REGEX.captures(line) {
                    let player = &captures[1];

                    if !self.current_match_data.players.contains(player) {
                        self.current_match_data.players.insert(String::from(player));
                    }

                    return Ok(());
                } else {
                    return Err(LogParserError::RegexParserError);
                }
            },
            LogEvent::Item => {
                return Ok(());
            },
            LogEvent::Kill => {
                if let Some(captures) = KILL_PARSER_REGEX.captures(line) {

                    let killer = &captures[3];
                    let player_killed = &captures[4];
                    let gun = &captures[5];
                    
                    self.current_match_data.total_kills += 1;

                    if killer == "<world>" {
                        if let Some(kills) = self.current_match_data.kills.get(player_killed) {
                            self.current_match_data.kills.insert(String::from(player_killed), kills - 1);
                        } else {
                            self.current_match_data.kills.insert(String::from(player_killed), -1);
                        }
                    } else {
                        if killer != player_killed {
                            if let Some(kills) = self.current_match_data.kills.get(killer) {
                                self.current_match_data.kills.insert(String::from(killer), kills + 1);
                            } else {
                                self.current_match_data.kills.insert(String::from(killer), 1);
                            }
                        }
                    }

                    return Ok(());
                } else {
                    println!("{}", line);
                    println!("error");
                    return Err(LogParserError::RegexParserError);
                }
            },
            LogEvent::ClientDisconnect => {
                return Ok(());
            },
            LogEvent::ShutdownGame => {
                return Ok(());
            },
            LogEvent::Exit => {
                return Ok(());
            },
        }
    }
}

impl ILogParser for ConcreteLogParser {
    
    fn register_success_callback(&mut self, callback: Box<LogParserCallBack>) {
        self.success_callback = Some(callback);
    }

    fn register_warning_callback(&mut self, callback: Box<LogParserCallBack>) {
        self.warning_callback = Some(callback);
    }

    fn register_error_callback(&mut self, callback: Box<LogParserCallBack>) {
        self.error_callback = Some(callback);
    }

    fn parse_file(&mut self) -> Pin<Box<dyn Future<Output = Result<String, LogParserError>> + '_>> {
        let future = async {
            let input = std::fs::File::open("sample_log.log").map_err(|_e| LogParserError::ReadFileError)?;
            let reader = io::BufReader::new(input);
        
            for line in reader.lines() {
                
                let line = line.map_err(|_e| LogParserError::ReadFileError)?;
                
                if let Err(err) = self.parse_log_line(&line) {
                    if let Some(warning_cb) = &self.warning_callback {
                        
                        let warning_payload = WarningCallbackPayload {
                            error: err.into(),
                            line: line
                        }; 
                        
                        warning_cb(Some(serde_json::to_value(warning_payload).unwrap()));
                    }
                }
            }

            let parsed_data = serde_json::to_value(&self.matches_data).map_err(|_e| LogParserError::SerializationError)?; 
            let stringfied_json = serde_json::to_string(&parsed_data).map_err(|_e| LogParserError::StringfyError)?;

            return Ok(stringfied_json)
        };

        return Box::pin(future);
    }
}