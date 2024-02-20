use std::fmt;
use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
pub enum DeathCauses {
    Unknown,
    Shotgun,
    Gauntlet,
    MachineGun,
    Grenade,
    GrenadeSplash,
    Rocket,
    RocketSplash,
    Plasma,
    PlasmaSplash,
    Railgun,
    Lightning,
    Bfg,
    BfgSplash,
    Water,
    Slime,
    Lava,
    Crush,
    Telefrag,
    Falling,
    Suicide,
    TargetLaser,
    TriggerHurt,
    Nail,
    Chaingun,
    ProximityMine,
    Kamikaze,
    Juiced,
    Grapple
}

impl fmt::Display for DeathCauses {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DeathCauses::Unknown => write!(f, "MOD_UNKNOWN"),
            DeathCauses::Shotgun => write!(f, "MOD_SHOTGUN"),
            DeathCauses::Gauntlet => write!(f, "MOD_GAUNTLET"),
            DeathCauses::MachineGun => write!(f, "MOD_MACHINEGUN"),
            DeathCauses::Grenade => write!(f, "MOD_GRENADE"),
            DeathCauses::GrenadeSplash => write!(f, "MOD_GRENADE_SPLASH"),
            DeathCauses::Rocket => write!(f, "MOD_ROCKET"),
            DeathCauses::RocketSplash => write!(f, "MOD_ROCKET_SPLASH"),
            DeathCauses::Plasma => write!(f, "MOD_PLASMA"),
            DeathCauses::PlasmaSplash => write!(f, "MOD_PLASMA_SPLASH"),
            DeathCauses::Railgun => write!(f, "MOD_RAILGUN"),
            DeathCauses::Lightning => write!(f, "MOD_LIGHTNING"),
            DeathCauses::Bfg => write!(f, "MOD_BFG"),
            DeathCauses::BfgSplash => write!(f, "MOD_BFG_SPLASH"),
            DeathCauses::Water => write!(f, "MOD_WATER"),
            DeathCauses::Slime => write!(f, "MOD_SLIME"),
            DeathCauses::Lava => write!(f, "MOD_LAVA"),
            DeathCauses::Crush => write!(f, "MOD_CRUSH"),
            DeathCauses::Telefrag => write!(f, "MOD_TELEFRAG"),
            DeathCauses::Falling => write!(f, "MOD_FALLING"),
            DeathCauses::Suicide => write!(f, "MOD_SUICIDE"),
            DeathCauses::TargetLaser => write!(f, "MOD_TARGET_LASER"),
            DeathCauses::TriggerHurt => write!(f, "MOD_TRIGGER_HURT"),
            DeathCauses::Nail => write!(f, "MOD_NAIL"),
            DeathCauses::Chaingun => write!(f, "MOD_CHAINGUN"),
            DeathCauses::ProximityMine => write!(f, "MOD_PROXIMITY_MINE"),
            DeathCauses::Kamikaze => write!(f, "MOD_KAMIKAZE"),
            DeathCauses::Juiced => write!(f, "MOD_JUICED"),
            DeathCauses::Grapple => write!(f, "MOD_GRAPPLE")
        }
    }
}

impl DeathCauses {

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "MOD_UNKNOWN" => Ok(DeathCauses::Unknown),
            "MOD_SHOTGUN" => Ok(DeathCauses::Shotgun),
            "MOD_GAUNTLET" => Ok(DeathCauses::Gauntlet),
            "MOD_MACHINEGUN" => Ok(DeathCauses::MachineGun),
            "MOD_GRENADE" => Ok(DeathCauses::Grenade),
            "MOD_GRENADE_SPLASH" => Ok(DeathCauses::GrenadeSplash),
            "MOD_ROCKET" => Ok(DeathCauses::Rocket),
            "MOD_ROCKET_SPLASH" => Ok(DeathCauses::RocketSplash),
            "MOD_PLASMA" => Ok(DeathCauses::Plasma),
            "MOD_PLASMA_SPLASH" => Ok(DeathCauses::PlasmaSplash),
            "MOD_RAILGUN" => Ok(DeathCauses::Railgun),
            "MOD_LIGHTNING" => Ok(DeathCauses::Lightning),
            "MOD_BFG" => Ok(DeathCauses::Bfg),
            "MOD_BFG_SPLASH" => Ok(DeathCauses::BfgSplash),
            "MOD_WATER" => Ok(DeathCauses::Water),
            "MOD_SLIME" => Ok(DeathCauses::Slime),
            "MOD_LAVA" => Ok(DeathCauses::Lava),
            "MOD_CRUSH" => Ok(DeathCauses::Crush),
            "MOD_TELEFRAG" => Ok(DeathCauses::Telefrag),
            "MOD_FALLING" => Ok(DeathCauses::Falling),
            "MOD_SUICIDE" => Ok(DeathCauses::Suicide),
            "MOD_TARGET_LASER" => Ok(DeathCauses::TargetLaser),
            "MOD_TRIGGER_HURT" => Ok(DeathCauses::TriggerHurt),
            "MOD_NAIL" => Ok(DeathCauses::Nail),
            "MOD_CHAINGUN" => Ok(DeathCauses::Chaingun),
            "MOD_PROXIMITY_MINE" => Ok(DeathCauses::ProximityMine),
            "MOD_KAMIKAZE" => Ok(DeathCauses::Kamikaze),
            "MOD_JUICED" => Ok(DeathCauses::Juiced),
            "MOD_GRAPPLE" => Ok(DeathCauses::Grapple),
            _ => Err("Invalid string".to_string())
        }
    }
}