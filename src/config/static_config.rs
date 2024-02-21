use std::fs::File;
use std::io::Read;
use serde::Deserialize;
use lazy_static::lazy_static;

use crate::config::config::ConfigValue;

pub(crate) enum StaticConfigParameter {
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
    OutputMatchKey,
    TotalKillsKey,
    PlayersKey,
    KillsKey,
    KillByMeansKey,
    InvalidKillMeanTokenErrMsg,
    LogFilePathNotFoundErrMsg
}

lazy_static! {

    pub(crate) static ref STATIC_CONFIG: StaticConfig = {

        let mut file = File::open("STATIC_CONFIG.json").expect("Unable to open static config file");
        let mut file_content = String::new();
        file.read_to_string(&mut file_content).expect("Unable to read static config file");
        return serde_json::from_str(&file_content).expect("Unable to parse static config Json file");

    };
}

#[derive(Debug, Deserialize)]
pub(crate) struct StaticConfig {
    regex_pattern_engine: RegexPatterns,
    log_patterns: LogPatterns,
    output_format: OutputFormat,
    error_messages: ErrorMessages
}

impl StaticConfig {
    pub(crate) fn get_parameter(&self, config: StaticConfigParameter) -> ConfigValue {
        match config {
            StaticConfigParameter::InitGameEventRegex => ConfigValue::Str(self.regex_pattern_engine.init_game_event.clone()),
            StaticConfigParameter::ClientConnectEventRegex => ConfigValue::Str(self.regex_pattern_engine.client_connect_event.clone()),
            StaticConfigParameter::ClientInfoChangeEventRegex => ConfigValue::Str(self.regex_pattern_engine.client_info_change_event.clone()),
            StaticConfigParameter::ClientBeginEventRegex => ConfigValue::Str(self.regex_pattern_engine.client_begin_event.clone()),
            StaticConfigParameter::ClientDisconnectEventRegex => ConfigValue::Str(self.regex_pattern_engine.client_disconnect_event.clone()),
            StaticConfigParameter::ItemEventRegex => ConfigValue::Str(self.regex_pattern_engine.item_event.clone()),
            StaticConfigParameter::KillEventRegex => ConfigValue::Str(self.regex_pattern_engine.kill_event.clone()),
            StaticConfigParameter::ShutdownEventRegex => ConfigValue::Str(self.regex_pattern_engine.shutdown_event.clone()),
            StaticConfigParameter::ExitEventRegex => ConfigValue::Str(self.regex_pattern_engine.exit_event.clone()),
            StaticConfigParameter::KillEventLineParserRegex => ConfigValue::Str(self.regex_pattern_engine.kill_event_line_parser.clone()),
            StaticConfigParameter::UserInfoLineParserRegex => ConfigValue::Str(self.regex_pattern_engine.user_info_line_parser.clone()),
            StaticConfigParameter::WorldLogPattern => ConfigValue::Str(self.log_patterns.world.clone()),
            StaticConfigParameter::KillsKey => ConfigValue::Str(self.log_patterns.kills_key.clone()),
            StaticConfigParameter::PlayersKey => ConfigValue::Str(self.log_patterns.players_key.clone()),
            StaticConfigParameter::TotalKillsKey => ConfigValue::Str(self.log_patterns.total_kills_key.clone()),
            StaticConfigParameter::KillByMeansKey => ConfigValue::Str(self.log_patterns.kill_by_means_key.clone()),
            StaticConfigParameter::OutputMatchKey => ConfigValue::Str(self.output_format.match_key.clone()),
            StaticConfigParameter::InvalidKillMeanTokenErrMsg => ConfigValue::Str(self.error_messages.invalid_kill_mean_token.clone()),
            StaticConfigParameter::LogFilePathNotFoundErrMsg => ConfigValue::Str(self.error_messages.log_file_path_not_found.clone())
        }
    }
}

#[derive(Debug, Deserialize)]
struct LogPatterns {
    world: String,
    total_kills_key: String,
    players_key: String,
    kills_key: String,
    kill_by_means_key: String
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
struct OutputFormat {
    match_key: String
}

#[derive(Debug, Deserialize)]
struct ErrorMessages {
    invalid_kill_mean_token: String,
    log_file_path_not_found: String
}
