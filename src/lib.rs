#![recursion_limit = "512"]

pub mod errors;
pub mod interface;
mod death_causes;
pub mod implementation;
mod service;
pub mod config;

pub mod lib {
    use super::{
        service::LogParser,
        implementation::log_parser::ConcreteLogParser,
    };

    pub fn factory() -> LogParser {
        let concrete_log_parser = ConcreteLogParser::new();
        let log_parser_service = LogParser::new(Box::new(concrete_log_parser));
        return log_parser_service;
    }

}

#[cfg(test)]
mod tests {

    use tokio::test;

    use super::{
        service::LogParser,
        implementation::log_parser::{ConcreteLogParser},
        config::{
            config::ConfigValue,
            dynamic_config::{CONFIG_FILE_PATH, CONFIG, ConfigParameter}
        }
    };

    #[test]
    async fn test() {

        CONFIG_FILE_PATH.with(|config_file_path_handler| {
            *config_file_path_handler.borrow_mut() = Some(String::from("config.json"));
        });

        CONFIG.with(|config| {
            config.borrow_mut().set_parameter(ConfigParameter::LogFilePath, ConfigValue::Str(String::from("sample_log.log")))
        });

        let concrete_log_parser = ConcreteLogParser::new();
        let mut log_parser_service = LogParser::new(Box::new(concrete_log_parser));
        if let Ok(value) = log_parser_service.parse_file().await {
            println!("{}", value);
        } else {
            println!("Error Parsing Log File");
        }
        
    }

}