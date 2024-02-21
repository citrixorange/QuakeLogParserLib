pub enum ConfigValue {
    Str(String),
    OptStr(Option<String>),
    Bool(bool)
}

impl ConfigValue {
    pub fn to_string(&self) -> String {
        match self {
            ConfigValue::Str(value) => value.clone(),
            ConfigValue::OptStr(value) => {
                if let Some(unwrapped_value) = value {
                    return unwrapped_value.clone();
                } else {
                    return String::from("None");
                }
            },
            ConfigValue::Bool(value) => value.to_string()
        }
    }

    pub fn to_optional_string(&self) -> Option<String> {
        match self {
            ConfigValue::Str(value) => Some(value.clone()),
            ConfigValue::OptStr(value) => value.clone(),
            ConfigValue::Bool(value) => Some(value.to_string())
        }
    }

    pub fn to_boolean(&self) -> bool {
        match self {
            ConfigValue::Str(value) => !value.is_empty(),
            ConfigValue::OptStr(value) => {
                if let Some(_unwrapped_value) = value {
                    return true;
                } else {
                    return false;
                }
            },
            ConfigValue::Bool(value) => *value
        }
    }
}