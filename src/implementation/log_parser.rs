use std::future::Future;
use std::pin::Pin;
use std::io::{self, BufRead};
use serde::{Serialize, Serializer};
use serde_json::{json};
use std::collections::{HashMap, HashSet};
use regex::Regex;
use once_cell::sync::Lazy;

use crate::interface::{ILogParser, LogParserCallBack, CallbackType, CallbackPayload};
use crate::errors::LogParserError;
use crate::config::config::{CONFIG, ConfigParameter};

static INIT_GAME_EVENT_DETECT_REGEX: Lazy<Regex> = Lazy::new(|| { Regex::new(&CONFIG.get_parameter(ConfigParameter::InitGameEventRegex).to_string().as_str()).unwrap() });
static CLIENT_CONNECT_EVENT_DETECT_REGEX: Lazy<Regex> = Lazy::new(|| { Regex::new(CONFIG.get_parameter(ConfigParameter::ClientConnectEventRegex).to_string().as_str()).unwrap() });
static CLIENT_INFO_CHANGE_EVENT_DETECT_REGEX: Lazy<Regex> = Lazy::new(|| { Regex::new(CONFIG.get_parameter(ConfigParameter::ClientInfoChangeEventRegex).to_string().as_str()).unwrap() });
static CLIENT_BEGIN_EVENT_DETECT_REGEX: Lazy<Regex> = Lazy::new(|| { Regex::new(CONFIG.get_parameter(ConfigParameter::ClientBeginEventRegex).to_string().as_str()).unwrap() });
static CLIENT_DISCONNECT_EVENT_DETECT_REGEX: Lazy<Regex> = Lazy::new(|| { Regex::new(CONFIG.get_parameter(ConfigParameter::ClientDisconnectEventRegex).to_string().as_str()).unwrap() });
static ITEM_EVENT_DETECT_REGEX: Lazy<Regex> = Lazy::new(|| { Regex::new(CONFIG.get_parameter(ConfigParameter::ItemEventRegex).to_string().as_str()).unwrap() });
static KILL_EVENT_DETECT_REGEX: Lazy<Regex> = Lazy::new(|| { Regex::new(CONFIG.get_parameter(ConfigParameter::KillEventRegex).to_string().as_str()).unwrap() });
static SHUTDOWN_EVENT_DETECT_REGEX: Lazy<Regex> = Lazy::new(|| { Regex::new(CONFIG.get_parameter(ConfigParameter::ShutdownEventRegex).to_string().as_str()).unwrap() });
static EXIT_EVENT_DETECT_REGEX: Lazy<Regex> = Lazy::new(|| { Regex::new(CONFIG.get_parameter(ConfigParameter::ExitEventRegex).to_string().as_str()).unwrap() });
static KILL_PARSER_REGEX: Lazy<Regex> = Lazy::new(|| { Regex::new(CONFIG.get_parameter(ConfigParameter::KillEventLineParserRegex).to_string().as_str()).unwrap() });
static USER_INFO_PARSER_REGEX: Lazy<Regex> = Lazy::new(|| { Regex::new(CONFIG.get_parameter(ConfigParameter::UserInfoLineParserRegex).to_string().as_str()).unwrap() });

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

#[derive(Clone, Default)]
struct MatchData {
    game_match: String,
    total_kills: i32,
    players: HashSet<String>,
    kills: HashMap<String, i32>,
}

impl Serialize for MatchData {

    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
    
        return json!({
            &self.game_match: {
                "total_kills": &self.total_kills,
                "players": serde_json::to_value(&self.players).unwrap(),
                "kills": serde_json::to_value(&self.kills).unwrap()
            }
        }).serialize(serializer);
    }
}

pub struct ConcreteLogParser {
    success_callback: Option<Box<LogParserCallBack>>,
    warning_callback: Option<Box<LogParserCallBack>>,
    error_callback: Option<Box<LogParserCallBack>>,
    matches_data: Vec<MatchData>,
    current_match_data: MatchData,
    first_match: bool
}

impl ConcreteLogParser {
    pub fn new() -> Self {
        Self {
            success_callback: None,
            warning_callback: None,
            error_callback: None,
            matches_data: Vec::<MatchData>::new(),
            current_match_data: MatchData {
                game_match: String::from(""),
                total_kills: 0,
                players: HashSet::new(),
                kills: HashMap::new()
            },
            first_match: true
        }
    }

    async fn handle_callback(&self, cb_type: CallbackType, error: Option<LogParserError>, data: Option<String>) {
        
        match cb_type {
            CallbackType::Success => {
                if let Some(cb) = &self.success_callback {

                    let payload = CallbackPayload {
                        error: None,
                        data: data
                    }; 
        
                    if let Ok(pl) = serde_json::to_value(payload) {
                        let _res = cb(Some(pl)).await;
                    }
                }
            },
            CallbackType::Warning => {
                if let Some(cb) = &self.warning_callback {

                    let payload = CallbackPayload {
                        error: Some(error.unwrap().into()),
                        data: data
                    }; 
        
                    if let Ok(pl) = serde_json::to_value(payload) {
                        let _res = cb(Some(pl)).await;
                    }
                }
            },
            CallbackType::Error => {
                if let Some(cb) = &self.error_callback {

                    let payload = CallbackPayload {
                        error: Some(error.unwrap().into()),
                        data: data
                    }; 
        
                    if let Ok(pl) = serde_json::to_value(payload) {
                        let _res = cb(Some(pl)).await;
                    }
                }
            }
        }
        
    }

    fn get_match_label(&self) -> String {
        return format!("{}_{}", CONFIG.get_parameter(ConfigParameter::OutuputMatchKey).to_string().as_str(), self.matches_data.len());
    }
 
    fn register_new_match_stat(&mut self, match_stats: MatchData) {
        
        self.matches_data.push(
            match_stats
        );
    }

    async fn parse_log_line(&mut self, line: &str) -> Result<(), LogParserError> {

        match LogEvent::detect_line_log_event(line)? {

            LogEvent::InitMatch => {
                if self.first_match == false {

                    self.current_match_data.game_match = self.get_match_label();

                    self.register_new_match_stat(
            self.current_match_data.clone()
                    );
    
                    self.current_match_data = MatchData {
                        game_match: String::from(""),
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

                    self.handle_callback(
                        CallbackType::Warning,
                          Some(LogParserError::RegexParserError),
                           Some(String::from(line))
                     ).await;

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

                    if killer == CONFIG.get_parameter(ConfigParameter::WorldLogPattern).to_string().as_str() {
                        if let Some(kills) = self.current_match_data.kills.get(player_killed) {
                            self.current_match_data.kills.insert(String::from(player_killed), kills - 1);
                        } else {
                            self.current_match_data.kills.insert(String::from(player_killed), -1);
                        }
                    } else {
                        if killer != player_killed || CONFIG.get_parameter(ConfigParameter::KillYourselfIncreasesScore).to_boolean() {
                            if let Some(kills) = self.current_match_data.kills.get(killer) {
                                self.current_match_data.kills.insert(String::from(killer), kills + 1);
                            } else {
                                self.current_match_data.kills.insert(String::from(killer), 1);
                            }
                        }

                        if CONFIG.get_parameter(ConfigParameter::BeingKilledDecreasesScore).to_boolean() {
                            if let Some(kills) = self.current_match_data.kills.get(player_killed) {
                                self.current_match_data.kills.insert(String::from(player_killed), kills - 1);
                            } else {
                                self.current_match_data.kills.insert(String::from(player_killed), -1);
                            }
                        }
                    }

                    return Ok(());
                } else {
                    
                    self.handle_callback(
                       CallbackType::Warning,
                         Some(LogParserError::RegexParserError),
                          Some(String::from(line))
                    ).await;

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
                
                if let Err(err) = self.parse_log_line(&line).await {
                    self.handle_callback(
                CallbackType::Warning,
                  Some(err), 
                   Some(line)
                    ).await;
                }
            }

            let parsed_data = serde_json::to_value(&self.matches_data).map_err(|_e| LogParserError::SerializationError)?; 
            
            let stringfied_json = serde_json::to_string(&parsed_data).map_err(|_e| LogParserError::StringfyError)?;

            self.handle_callback(
                CallbackType::Success,
                  None,
                   Some(stringfied_json.clone())
             ).await;

            return Ok(stringfied_json)
        };

        return Box::pin(future);
    }
}