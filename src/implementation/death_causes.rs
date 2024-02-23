use serde_json::{json, Value};

use crate::death_causes::DeathCauses;

#[derive(Clone, Default)]
pub(crate) struct MatchKillMeans {
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
    pub(crate) fn new() -> Self {
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
    
    pub(crate) fn increase_stat(&mut self, death_cause: DeathCauses) {
        match death_cause {
            DeathCauses::Unknown => self.unknown += 1,
            DeathCauses::Shotgun => self.shotgun += 1,
            DeathCauses::Gauntlet => self.gauntlet += 1,
            DeathCauses::MachineGun => self.machine_gun += 1,
            DeathCauses::Grenade => self.grenade += 1,
            DeathCauses::GrenadeSplash => self.grenade_splash += 1,
            DeathCauses::Rocket => self.rocket += 1,
            DeathCauses::RocketSplash => self.rocket_splash += 1,
            DeathCauses::Plasma => self.plasma += 1,
            DeathCauses::PlasmaSplash => self.plasma_splash += 1,
            DeathCauses::Railgun => self.railgun += 1,
            DeathCauses::Lightning => self.lightning += 1,
            DeathCauses::Bfg => self.bfg += 1,
            DeathCauses::BfgSplash => self.bfg_splash += 1,
            DeathCauses::Water => self.water += 1,
            DeathCauses::Slime => self.slime += 1,
            DeathCauses::Lava => self.lava += 1,
            DeathCauses::Crush => self.crush += 1,
            DeathCauses::Telefrag => self.telefrag += 1,
            DeathCauses::Falling => self.falling += 1,
            DeathCauses::Suicide => self.suicide += 1,
            DeathCauses::TargetLaser => self.target_laser += 1,
            DeathCauses::TriggerHurt => self.trigger_hurt += 1,
            DeathCauses::Nail => self.nail += 1,
            DeathCauses::Chaingun => self.chaingun += 1,
            DeathCauses::ProximityMine => self.proximity_mine += 1,
            DeathCauses::Kamikaze => self.kamikaze += 1,
            DeathCauses::Juiced => self.juiced += 1,
            DeathCauses::Grapple => self.grapple += 1
        }
    }

    pub(crate) fn to_json(&self) -> Value {
        return json!({
            DeathCauses::Unknown.to_string(): &self.unknown,
            DeathCauses::Shotgun.to_string(): &self.shotgun,
            DeathCauses::Gauntlet.to_string(): &self.gauntlet,
            DeathCauses::MachineGun.to_string(): &self.machine_gun,
            DeathCauses::Grenade.to_string(): &self.grenade,
            DeathCauses::GrenadeSplash.to_string(): &self.grenade_splash,
            DeathCauses::Rocket.to_string(): &self.rocket,
            DeathCauses::RocketSplash.to_string(): &self.rocket_splash,
            DeathCauses::Plasma.to_string(): &self.plasma,
            DeathCauses::PlasmaSplash.to_string(): &self.plasma_splash,
            DeathCauses::Railgun.to_string(): &self.railgun,
            DeathCauses::Lightning.to_string(): &self.lightning,
            DeathCauses::Bfg.to_string(): &self.bfg,
            DeathCauses::BfgSplash.to_string(): &self.bfg_splash,
            DeathCauses::Water.to_string(): &self.water,
            DeathCauses::Slime.to_string(): &self.slime,
            DeathCauses::Lava.to_string(): &self.lava,
            DeathCauses::Crush.to_string(): &self.crush,
            DeathCauses::Telefrag.to_string(): &self.telefrag,
            DeathCauses::Falling.to_string(): &self.falling,
            DeathCauses::Suicide.to_string(): &self.suicide,
            DeathCauses::TargetLaser.to_string(): &self.target_laser,
            DeathCauses::TriggerHurt.to_string(): &self.trigger_hurt,
            DeathCauses::Nail.to_string(): &self.nail,
            DeathCauses::Chaingun.to_string(): &self.chaingun,
            DeathCauses::ProximityMine.to_string(): &self.proximity_mine,
            DeathCauses::Kamikaze.to_string(): &self.kamikaze,
            DeathCauses::Juiced.to_string(): &self.juiced,
            DeathCauses::Grapple.to_string(): &self.grapple
        });
    }  
}