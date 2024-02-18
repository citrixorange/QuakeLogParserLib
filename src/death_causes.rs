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
            DeathCauses::Unknown => write!(f, "DeathCauses::Unknown"),
            DeathCauses::Shotgun => write!(f, "DeathCauses::Shotgun"),
            DeathCauses::Gauntlet => write!(f, "DeathCauses::Gauntlet"),
            DeathCauses::MachineGun => write!(f, "DeathCauses::MachineGun"),
            DeathCauses::Grenade => write!(f, "DeathCauses::Grenade"),
            DeathCauses::GrenadeSplash => write!(f, "DeathCauses::GrenadeSplash"),
            DeathCauses::Rocket => write!(f, "DeathCauses::Rocket"),
            DeathCauses::RocketSplash => write!(f, "DeathCauses::RocketSplash"),
            DeathCauses::Plasma => write!(f, "DeathCauses::Plasma"),
            DeathCauses::PlasmaSplash => write!(f, "DeathCauses::PlasmaSplash"),
            DeathCauses::Railgun => write!(f, "DeathCauses::Railgun"),
            DeathCauses::Lightning => write!(f, "DeathCauses::Lightning"),
            DeathCauses::Bfg => write!(f, "DeathCauses::Bfg"),
            DeathCauses::BfgSplash => write!(f, "DeathCauses::BfgSplash"),
            DeathCauses::Water => write!(f, "DeathCauses::Water"),
            DeathCauses::Slime => write!(f, "DeathCauses::Slime"),
            DeathCauses::Lava => write!(f, "DeathCauses::Lava"),
            DeathCauses::Crush => write!(f, "DeathCauses::Crush"),
            DeathCauses::Telefrag => write!(f, "DeathCauses::Telefrag"),
            DeathCauses::Falling => write!(f, "DeathCauses::Falling"),
            DeathCauses::Suicide => write!(f, "DeathCauses::Suicide"),
            DeathCauses::TargetLaser => write!(f, "DeathCauses::TargetLaser"),
            DeathCauses::TriggerHurt => write!(f, "DeathCauses::TargetHurt"),
            DeathCauses::Nail => write!(f, "DeathCauses::Nail"),
            DeathCauses::Chaingun => write!(f, "DeathCauses::Chaingun"),
            DeathCauses::ProximityMine => write!(f, "DeathCauses::ProximityMine"),
            DeathCauses::Kamikaze => write!(f, "DeathCauses::Kamikaze"),
            DeathCauses::Juiced => write!(f, "DeathCauses::Juiced"),
            DeathCauses::Grapple => write!(f, "DeathCauses::Grapple")
        }
    }
}

impl FromStr for DeathCauses {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "\"MOD_UNKNOWN\"" => Ok(DeathCauses::Unknown),
            "\"MOD_SHOTGUN\"" => Ok(DeathCauses::Shotgun),
            "\"MOD_GAUNTLET\"" => Ok(DeathCauses::Gauntlet),
            "\"MOD_MACHINEGUN\"" => Ok(DeathCauses::MachineGun),
            "\"MOD_GRENADE\"" => Ok(DeathCauses::Grenade),
            "\"MOD_GRENADE_SPLASH\"" => Ok(DeathCauses::GrenadeSplash),
            "\"MOD_ROCKET\"" => Ok(DeathCauses::Rocket),
            "\"MOD_ROCKET_SPLASH\"" => Ok(DeathCauses::RocketSplash),
            "\"MOD_PLASMA\"" => Ok(DeathCauses::Plasma),
            "\"MOD_PLASMA_SPLASH\"" => Ok(DeathCauses::PlasmaSplash),
            "\"MOD_RAILGUN\"" => Ok(DeathCauses::Railgun),
            "\"MOD_LIGHTNING\"" => Ok(DeathCauses::Lightning),
            "\"MOD_BFG\"" => Ok(DeathCauses::Bfg),
            "\"MOD_BFG_SPLASH\"" => Ok(DeathCauses::BfgSplash),
            "\"MOD_WATER\"" => Ok(DeathCauses::Water),
            "\"MOD_SLIME\"" => Ok(DeathCauses::Slime),
            "\"MOD_LAVA\"" => Ok(DeathCauses::Lava),
            "\"MOD_CRUSH\"" => Ok(DeathCauses::Crush),
            "\"MOD_TELEFRAG\"" => Ok(DeathCauses::Telefrag),
            "\"MOD_FALLING\"" => Ok(DeathCauses::Falling),
            "\"MOD_SUICIDE\"" => Ok(DeathCauses::Suicide),
            "\"MOD_TARGET_LASER\"" => Ok(DeathCauses::TargetLaser),
            "\"MOD_TRIGGER_HURT\"" => Ok(DeathCauses::TriggerHurt),
            "\"MOD_NAIL\"" => Ok(DeathCauses::Nail),
            "\"MOD_CHAINGUN\"" => Ok(DeathCauses::Chaingun),
            "\"MOD_PROXIMITY_MINE\"" => Ok(DeathCauses::ProximityMine),
            "\"MOD_KAMIKAZE\"" => Ok(DeathCauses::Kamikaze),
            "\"MOD_JUICED\"" => Ok(DeathCauses::Juiced),
            "\"MOD_GRAPPLE\"" => Ok(DeathCauses::Grapple),
            _ => Err(())
        }
    }
}