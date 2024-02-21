use std::cell::RefCell;
use std::fs::File;
use std::io::Read;
use serde::Deserialize;

use crate::config::config::ConfigValue;

pub enum ConfigParameter {
    LogFilePath,
    BeingKilledDecreasesScore,
    KillYourselfIncreasesScore,
    ShowDeathCauses
}

thread_local!(pub static CONFIG_FILE_PATH: RefCell<Option<String>> = RefCell::new(None) );

thread_local!(pub static CONFIG: RefCell<Config> = {

    let mut path:String =  String::from(""); 

    CONFIG_FILE_PATH.with(|config_file_path_handler| {
        
        if let Some(config_file_path) = config_file_path_handler.borrow().as_ref() {
            path = config_file_path.clone();
        }
    });

    if path == "" {
        return RefCell::new(Config {
            kills_rules: KillsRules {
                being_killed_decreases_score: false,
                kill_yourself_increases_score: false,
                show_death_causes: false
            },
            log_file_path: None
        });
    }

    let mut file = File::open(path).expect("Unable to open config file");
    let mut file_content = String::new();
    file.read_to_string(&mut file_content).expect("Unable to read config file");
    return RefCell::new(serde_json::from_str(&file_content).expect("Unable to parse config Json file"));
});


#[derive(Debug, Deserialize)]
pub struct Config {
    kills_rules: KillsRules,
    log_file_path: Option<String>
}

impl Config {
    pub fn get_parameter(&self, config: ConfigParameter) -> ConfigValue {
        match config {
            ConfigParameter::BeingKilledDecreasesScore => ConfigValue::Bool(self.kills_rules.being_killed_decreases_score),
            ConfigParameter::KillYourselfIncreasesScore => ConfigValue::Bool(self.kills_rules.kill_yourself_increases_score),
            ConfigParameter::ShowDeathCauses => ConfigValue::Bool(self.kills_rules.show_death_causes),
            ConfigParameter::LogFilePath => ConfigValue::OptStr(self.log_file_path.clone())
        }
    }

    pub fn set_parameter(&mut self, config: ConfigParameter, value: ConfigValue) {
        match config {
            ConfigParameter::BeingKilledDecreasesScore => { self.kills_rules.being_killed_decreases_score = value.to_boolean() },
            ConfigParameter::KillYourselfIncreasesScore => { self.kills_rules.kill_yourself_increases_score = value.to_boolean() },
            ConfigParameter::ShowDeathCauses => { self.kills_rules.show_death_causes = value.to_boolean() },
            ConfigParameter::LogFilePath => { self.log_file_path = value.to_optional_string() }
        }
    }
}

#[derive(Debug, Deserialize)]
struct KillsRules {
    being_killed_decreases_score: bool,
    kill_yourself_increases_score: bool,
    show_death_causes: bool
}