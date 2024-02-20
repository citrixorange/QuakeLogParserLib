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
use crate::death_causes::DeathCauses;

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
struct MatchKillMeans {
    unknown: usize,
    shotgun: usize,
    gauntlet: usize,
    machine_gun: usize,
    grenade: usize,
    grenade_splash: usize,
    rocket: usize,
    rocket_splash: usize,
    plasma: usize,
    plasma_splash: usize,
    railgun: usize,
    lightning: usize,
    bfg: usize,
    bfg_splash: usize,
    water: usize,
    slime: usize,
    lava: usize,
    crush: usize,
    telefrag: usize,
    falling: usize,
    suicide: usize,
    target_laser: usize,
    trigger_hurt: usize,
    nail: usize,
    chaingun: usize,
    proximity_mine: usize,
    kamikaze: usize,
    juiced: usize,
    grapple: usize
}

impl MatchKillMeans {
    pub fn new() -> Self {
        Self {
            unknown: 0,
            shotgun: 0,
            gauntlet: 0,
            machine_gun: 0,
            grenade: 0,
            grenade_splash: 0,
            rocket: 0,
            rocket_splash: 0,
            plasma: 0,
            plasma_splash: 0,
            railgun: 0,
            lightning: 0,
            bfg: 0,
            bfg_splash: 0,
            water: 0,
            slime: 0,
            lava: 0,
            crush: 0,
            telefrag: 0,
            falling: 0,
            suicide: 0,
            target_laser: 0,
            trigger_hurt: 0,
            nail: 0,
            chaingun: 0,
            proximity_mine: 0,
            kamikaze: 0,
            juiced: 0,
            grapple: 0
        }
    } 
}

#[derive(Clone, Default)]
struct MatchData {
    game_match: String,
    total_kills: i32,
    players: HashSet<String>,
    kills: HashMap<String, i32>,
    kill_means: Option<MatchKillMeans>
}

impl Serialize for MatchData {

    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {

        if CONFIG.get_parameter(ConfigParameter::ShowDeathCauses).to_boolean() { 

            return json!({
                &self.game_match: {
                    "total_kills": &self.total_kills,
                    "players": serde_json::to_value(&self.players).unwrap(),
                    "kills": serde_json::to_value(&self.kills).unwrap(),
                    "kill_by_means": {
                        DeathCauses::Unknown.to_string(): &self.kill_means.as_ref().unwrap().unknown,
                        DeathCauses::Shotgun.to_string(): &self.kill_means.as_ref().unwrap().shotgun,
                        DeathCauses::Gauntlet.to_string(): &self.kill_means.as_ref().unwrap().gauntlet,
                        DeathCauses::MachineGun.to_string(): &self.kill_means.as_ref().unwrap().machine_gun,
                        DeathCauses::Grenade.to_string(): &self.kill_means.as_ref().unwrap().grenade,
                        DeathCauses::GrenadeSplash.to_string(): &self.kill_means.as_ref().unwrap().grenade_splash,
                        DeathCauses::Rocket.to_string(): &self.kill_means.as_ref().unwrap().rocket,
                        DeathCauses::RocketSplash.to_string(): &self.kill_means.as_ref().unwrap().rocket_splash,
                        DeathCauses::Plasma.to_string(): &self.kill_means.as_ref().unwrap().plasma,
                        DeathCauses::PlasmaSplash.to_string(): &self.kill_means.as_ref().unwrap().plasma_splash,
                        DeathCauses::Railgun.to_string(): &self.kill_means.as_ref().unwrap().railgun,
                        DeathCauses::Lightning.to_string(): &self.kill_means.as_ref().unwrap().lightning,
                        DeathCauses::Bfg.to_string(): &self.kill_means.as_ref().unwrap().bfg,
                        DeathCauses::BfgSplash.to_string(): &self.kill_means.as_ref().unwrap().bfg_splash,
                        DeathCauses::Water.to_string(): &self.kill_means.as_ref().unwrap().water,
                        DeathCauses::Slime.to_string(): &self.kill_means.as_ref().unwrap().slime,
                        DeathCauses::Lava.to_string(): &self.kill_means.as_ref().unwrap().lava,
                        DeathCauses::Crush.to_string(): &self.kill_means.as_ref().unwrap().crush,
                        DeathCauses::Telefrag.to_string(): &self.kill_means.as_ref().unwrap().telefrag,
                        DeathCauses::Falling.to_string(): &self.kill_means.as_ref().unwrap().falling,
                        DeathCauses::Suicide.to_string(): &self.kill_means.as_ref().unwrap().suicide,
                        DeathCauses::TargetLaser.to_string(): &self.kill_means.as_ref().unwrap().target_laser,
                        DeathCauses::TriggerHurt.to_string(): &self.kill_means.as_ref().unwrap().trigger_hurt,
                        DeathCauses::Nail.to_string(): &self.kill_means.as_ref().unwrap().nail,
                        DeathCauses::Chaingun.to_string(): &self.kill_means.as_ref().unwrap().chaingun,
                        DeathCauses::ProximityMine.to_string(): &self.kill_means.as_ref().unwrap().proximity_mine,
                        DeathCauses::Kamikaze.to_string(): &self.kill_means.as_ref().unwrap().kamikaze,
                        DeathCauses::Juiced.to_string(): &self.kill_means.as_ref().unwrap().juiced,
                        DeathCauses::Grapple.to_string(): &self.kill_means.as_ref().unwrap().grapple
                    }
                }
            }).serialize(serializer); 
        } else { 
            return json!({
                &self.game_match: {
                    "total_kills": &self.total_kills,
                    "players": serde_json::to_value(&self.players).unwrap(),
                    "kills": serde_json::to_value(&self.kills).unwrap()
                }
            }).serialize(serializer);
        } 
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
                kills: HashMap::new(),
                kill_means: if CONFIG.get_parameter(ConfigParameter::ShowDeathCauses).to_boolean() { Some(MatchKillMeans::new()) } else { None } 
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
                        kills: HashMap::new(),
                        kill_means: if CONFIG.get_parameter(ConfigParameter::ShowDeathCauses).to_boolean() { Some(MatchKillMeans::new()) } else { None }
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

                    if CONFIG.get_parameter(ConfigParameter::ShowDeathCauses).to_boolean() {
                        
                        if let Ok(death_cause) = DeathCauses::from_str(gun) {
                            match death_cause {
                                DeathCauses::Unknown => self.current_match_data.kill_means.as_mut().unwrap().unknown += 1,
                                DeathCauses::Shotgun => self.current_match_data.kill_means.as_mut().unwrap().shotgun += 1,
                                DeathCauses::Gauntlet => self.current_match_data.kill_means.as_mut().unwrap().gauntlet += 1,
                                DeathCauses::MachineGun => self.current_match_data.kill_means.as_mut().unwrap().machine_gun += 1,
                                DeathCauses::Grenade => self.current_match_data.kill_means.as_mut().unwrap().grenade += 1,
                                DeathCauses::GrenadeSplash => self.current_match_data.kill_means.as_mut().unwrap().grenade_splash += 1,
                                DeathCauses::Rocket => self.current_match_data.kill_means.as_mut().unwrap().rocket += 1,
                                DeathCauses::RocketSplash => self.current_match_data.kill_means.as_mut().unwrap().rocket_splash += 1,
                                DeathCauses::Plasma => self.current_match_data.kill_means.as_mut().unwrap().plasma += 1,
                                DeathCauses::PlasmaSplash => self.current_match_data.kill_means.as_mut().unwrap().plasma_splash += 1,
                                DeathCauses::Railgun => self.current_match_data.kill_means.as_mut().unwrap().railgun += 1,
                                DeathCauses::Lightning => self.current_match_data.kill_means.as_mut().unwrap().lightning += 1,
                                DeathCauses::Bfg => self.current_match_data.kill_means.as_mut().unwrap().bfg += 1,
                                DeathCauses::BfgSplash => self.current_match_data.kill_means.as_mut().unwrap().bfg_splash += 1,
                                DeathCauses::Water => self.current_match_data.kill_means.as_mut().unwrap().water += 1,
                                DeathCauses::Slime => self.current_match_data.kill_means.as_mut().unwrap().slime += 1,
                                DeathCauses::Lava => self.current_match_data.kill_means.as_mut().unwrap().lava += 1,
                                DeathCauses::Crush => self.current_match_data.kill_means.as_mut().unwrap().crush += 1,
                                DeathCauses::Telefrag => self.current_match_data.kill_means.as_mut().unwrap().telefrag += 1,
                                DeathCauses::Falling => self.current_match_data.kill_means.as_mut().unwrap().falling += 1,
                                DeathCauses::Suicide => self.current_match_data.kill_means.as_mut().unwrap().suicide += 1,
                                DeathCauses::TargetLaser => self.current_match_data.kill_means.as_mut().unwrap().target_laser += 1,
                                DeathCauses::TriggerHurt => self.current_match_data.kill_means.as_mut().unwrap().trigger_hurt += 1,
                                DeathCauses::Nail => self.current_match_data.kill_means.as_mut().unwrap().nail += 1,
                                DeathCauses::Chaingun => self.current_match_data.kill_means.as_mut().unwrap().chaingun += 1,
                                DeathCauses::ProximityMine => self.current_match_data.kill_means.as_mut().unwrap().proximity_mine += 1,
                                DeathCauses::Kamikaze => self.current_match_data.kill_means.as_mut().unwrap().kamikaze += 1,
                                DeathCauses::Juiced => self.current_match_data.kill_means.as_mut().unwrap().juiced += 1,
                                DeathCauses::Grapple => self.current_match_data.kill_means.as_mut().unwrap().grapple += 1
                            }
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