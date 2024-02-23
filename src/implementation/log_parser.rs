use std::future::Future;
use std::pin::Pin;
use std::cell::RefCell;
use std::io::{self, BufRead};
use std::collections::{HashMap, HashSet};

use crate::interface::{ILogParser, LogParserCallBack, CallbackType, CallbackPayload};
use crate::errors::LogParserError;
use crate::config::static_config::{STATIC_CONFIG, StaticConfigParameter};
use crate::config::dynamic_config::{CONFIG, ConfigParameter};
use crate::death_causes::DeathCauses;
use crate::implementation::{
    death_causes::MatchKillMeans,
    match_data::MatchData,
    log_event::{
        LogEvent,
        KILL_PARSER_REGEX,
        USER_INFO_PARSER_REGEX
    },
};

thread_local!(pub static LOG_FILE_PATH: RefCell<Option<String>> = RefCell::new(None) );

pub(crate) struct ConcreteLogParser {
    success_callback: Option<Box<LogParserCallBack>>,
    warning_callback: Option<Box<LogParserCallBack>>,
    error_callback: Option<Box<LogParserCallBack>>,
    matches_data: Vec<MatchData>,
    current_match_data: MatchData,
    first_match: bool
}

impl ConcreteLogParser {
    pub(crate) fn new() -> Self {

        let mut show_death_causes: bool = false;

        CONFIG.with(|config| {
            show_death_causes = config.borrow().get_parameter(ConfigParameter::ShowDeathCauses).to_boolean();
        });

        Self {
            success_callback: None,
            warning_callback: None,
            error_callback: None,
            matches_data: Vec::<MatchData>::new(),
            current_match_data: MatchData {
                game_match: String::from(""),
                total_kills: 0,
                players: HashSet::new(),
                kills: HashMap::new(),
                kill_means: if show_death_causes { Some(MatchKillMeans::new()) } else { None } 
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
        return format!("{}_{}", STATIC_CONFIG.get_parameter(StaticConfigParameter::OutputMatchKey).to_string().as_str(), self.matches_data.len());
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

                    let mut show_death_causes: bool = false;

                    CONFIG.with(|config| {
                        show_death_causes = config.borrow().get_parameter(ConfigParameter::ShowDeathCauses).to_boolean();
                    });

                    self.current_match_data.game_match = self.get_match_label();

                    self.register_new_match_stat(
                        self.current_match_data.clone()
                    );
    
                    self.current_match_data = MatchData {
                        game_match: String::from(""),
                        total_kills: 0,
                        players: HashSet::new(),
                        kills: HashMap::new(),
                        kill_means: if show_death_causes { Some(MatchKillMeans::new()) } else { None }
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

                    let mut show_death_causes: bool = false;
                    let mut self_kill_increases_score: bool = false;
                    let mut being_killed_decreases_score: bool = false;

                    CONFIG.with(|config| {
                        show_death_causes = config.borrow().get_parameter(ConfigParameter::ShowDeathCauses).to_boolean();
                        self_kill_increases_score = config.borrow().get_parameter(ConfigParameter::KillYourselfIncreasesScore).to_boolean();
                        being_killed_decreases_score = config.borrow().get_parameter(ConfigParameter::BeingKilledDecreasesScore).to_boolean();
                    });

                    let killer = &captures[3];
                    let player_killed = &captures[4];
                    let gun = &captures[5];
                    
                    self.current_match_data.total_kills += 1;

                    if killer == STATIC_CONFIG.get_parameter(StaticConfigParameter::WorldLogPattern).to_string().as_str() {
                        if let Some(kills) = self.current_match_data.kills.get(player_killed) {
                            self.current_match_data.kills.insert(String::from(player_killed), kills - 1);
                        } else {
                            self.current_match_data.kills.insert(String::from(player_killed), -1);
                        }
                    } else {

                        if killer != player_killed || self_kill_increases_score {
                            if let Some(kills) = self.current_match_data.kills.get(killer) {
                                self.current_match_data.kills.insert(String::from(killer), kills + 1);
                            } else {
                                self.current_match_data.kills.insert(String::from(killer), 1);
                            }
                        }

                        if being_killed_decreases_score {
                            if let Some(kills) = self.current_match_data.kills.get(player_killed) {
                                self.current_match_data.kills.insert(String::from(player_killed), kills - 1);
                            } else {
                                self.current_match_data.kills.insert(String::from(player_killed), -1);
                            }
                        }
                    }

                    if show_death_causes {
                        
                        if let Ok(death_cause) = DeathCauses::from_str(gun) {
                            self.current_match_data.kill_means.as_mut().unwrap().increase_stat(death_cause);
                        } else {
                            return Err(LogParserError::RegexParserError);
                        }
                    }

                    return Ok(());
                } else {
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

            let mut path:String = String::from(""); 

            CONFIG.with(|config| {
                if let Some(log_file_path) = config.borrow().get_parameter(ConfigParameter::LogFilePath).to_optional_string() {
                    path = log_file_path.clone();
                } else {
                    panic!("{}", STATIC_CONFIG.get_parameter(StaticConfigParameter::LogFilePathNotFoundErrMsg).to_string().as_str())
                }
            });

            let input = std::fs::File::open(path).map_err(|_e| LogParserError::ReadFileError)?;
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

            return Ok(stringfied_json);
        };

        return Box::pin(future);
    }
}