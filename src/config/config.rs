use std::fs::File;
use std::io::Read;
use serde::Deserialize;
use lazy_static::lazy_static;


pub enum ConfigParameter {
    InitGameEventRegex,
    ClientConnectEventRegex,
    ClientInfoChangeEventRegex,
    ClientBeginEventRegex,
    ClientDisconnectEventRegex,
    ItemEventRegex,
    KillEventRegex,
    ShutdownEventRegex,
    ExitEventRegex,
    KillEventLineParserRegex,
    UserInfoLineParserRegex,
    WorldLogPattern,
    BeingKilledDecreasesScore,
    KillYourselfIncreasesScore,
    OutuputMatchKey
}

lazy_static! {
    pub static ref CONFIG: Config = {
        let config_file_path = "/home/cesar-sandbox/Projects/QuakeLogParserLib/config.json";
        let mut file = File::open(config_file_path).expect("Unable to open config file");
        let mut file_content = String::new();
        file.read_to_string(&mut file_content).expect("Unable to read config file");
        serde_json::from_str(&file_content).expect("Unable to parse config Json file")
    };
}

pub enum ConfigOutput {
    Str(String),
    Bool(bool)
}

impl ConfigOutput {
    pub fn to_string(&self) -> String {
        match self {
            ConfigOutput::Str(value) => value.clone(),
            ConfigOutput::Bool(value) => value.to_string()
        }
    }

    pub fn to_boolean(&self) -> bool {
        match self {
            ConfigOutput::Str(value) => !value.is_empty(),
            ConfigOutput::Bool(value) => *value
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Config {
    regex_pattern_engine: RegexPatterns,
    log_patterns: LogPatterns,
    kills_rules: KillsRules,
    output_format: OutputFormat
}

impl Config {
    pub fn get_parameter(&self, config: ConfigParameter) -> ConfigOutput {
        match config {
            ConfigParameter::InitGameEventRegex => ConfigOutput::Str(self.regex_pattern_engine.init_game_event.clone()),
            ConfigParameter::ClientConnectEventRegex => ConfigOutput::Str(self.regex_pattern_engine.client_connect_event.clone()),
            ConfigParameter::ClientInfoChangeEventRegex => ConfigOutput::Str(self.regex_pattern_engine.client_info_change_event.clone()),
            ConfigParameter::ClientBeginEventRegex => ConfigOutput::Str(self.regex_pattern_engine.client_begin_event.clone()),
            ConfigParameter::ClientDisconnectEventRegex => ConfigOutput::Str(self.regex_pattern_engine.client_disconnect_event.clone()),
            ConfigParameter::ItemEventRegex => ConfigOutput::Str(self.regex_pattern_engine.item_event.clone()),
            ConfigParameter::KillEventRegex => ConfigOutput::Str(self.regex_pattern_engine.kill_event.clone()),
            ConfigParameter::ShutdownEventRegex => ConfigOutput::Str(self.regex_pattern_engine.shutdown_event.clone()),
            ConfigParameter::ExitEventRegex => ConfigOutput::Str(self.regex_pattern_engine.exit_event.clone()),
            ConfigParameter::KillEventLineParserRegex => ConfigOutput::Str(self.regex_pattern_engine.kill_event_line_parser.clone()),
            ConfigParameter::UserInfoLineParserRegex => ConfigOutput::Str(self.regex_pattern_engine.user_info_line_parser.clone()),
            ConfigParameter::WorldLogPattern => ConfigOutput::Str(self.log_patterns.world.clone()),
            ConfigParameter::BeingKilledDecreasesScore => ConfigOutput::Bool(self.kills_rules.being_killed_decreases_score),
            ConfigParameter::KillYourselfIncreasesScore => ConfigOutput::Bool(self.kills_rules.kill_yourself_increases_score),
            ConfigParameter::OutuputMatchKey => ConfigOutput::Str(self.output_format.match_key.clone())
        }
    }
}

#[derive(Debug, Deserialize)]
struct LogPatterns {
    world: String
}

#[derive(Debug, Deserialize)]
struct RegexPatterns {
    init_game_event: String,
    client_connect_event: String,
    client_info_change_event: String,
    client_begin_event: String,
    client_disconnect_event: String,
    item_event: String,
    kill_event: String,
    shutdown_event: String,
    exit_event: String,
    kill_event_line_parser: String,
    user_info_line_parser: String
}

#[derive(Debug, Deserialize)]
struct KillsRules {
    being_killed_decreases_score: bool,
    kill_yourself_increases_score: bool
}

#[derive(Debug, Deserialize)]
struct OutputFormat {
    match_key: String
}