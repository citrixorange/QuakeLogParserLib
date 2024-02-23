use std::collections::{HashMap, HashSet};
use serde::{Serialize, Serializer};
use serde_json::{json};

use crate::config::{
    dynamic_config::{CONFIG, ConfigParameter},
    static_config::{STATIC_CONFIG, StaticConfigParameter}
};
use crate::implementation::death_causes::MatchKillMeans;

#[derive(Clone, Default)]
pub(crate) struct MatchData {
    pub(crate) game_match: String,
    pub(crate) total_kills: i32,
    pub(crate) players: HashSet<String>,
    pub(crate) kills: HashMap<String, i32>,
    pub(crate) kill_means: Option<MatchKillMeans>
}

impl Serialize for MatchData {

    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut show_death_causes: bool = false;

        CONFIG.with(|config| {
            show_death_causes = config.borrow().get_parameter(ConfigParameter::ShowDeathCauses).to_boolean();
        });

        if show_death_causes { 

            return json!({
                &self.game_match: {
                    STATIC_CONFIG.get_parameter(StaticConfigParameter::TotalKillsKey).to_string().as_str(): &self.total_kills,
                    STATIC_CONFIG.get_parameter(StaticConfigParameter::PlayersKey).to_string().as_str(): serde_json::to_value(&self.players).unwrap(),
                    STATIC_CONFIG.get_parameter(StaticConfigParameter::KillsKey).to_string().as_str(): serde_json::to_value(&self.kills).unwrap(),
                    STATIC_CONFIG.get_parameter(StaticConfigParameter::KillByMeansKey).to_string().as_str(): &self.kill_means.as_ref().unwrap().to_json()
                }
            }).serialize(serializer); 
        } else { 
            return json!({
                &self.game_match: {
                    STATIC_CONFIG.get_parameter(StaticConfigParameter::TotalKillsKey).to_string().as_str(): &self.total_kills,
                    STATIC_CONFIG.get_parameter(StaticConfigParameter::PlayersKey).to_string().as_str(): serde_json::to_value(&self.players).unwrap(),
                    STATIC_CONFIG.get_parameter(StaticConfigParameter::KillsKey).to_string().as_str(): serde_json::to_value(&self.kills).unwrap()
                }
            }).serialize(serializer);
        } 
    }
}
