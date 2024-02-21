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
        implementation::log_parser::{ConcreteLogParser, LOG_FILE_PATH},
        config::config::CONFIG_FILE_PATH
    };

    #[test]
    async fn test() {

        CONFIG_FILE_PATH.with(|config_file_path_handler| {
            *config_file_path_handler.borrow_mut() = Some(String::from("config.json"));
        });

        LOG_FILE_PATH.with(|log_file_path_handler| {
            *log_file_path_handler.borrow_mut() = Some(String::from("sample_log.log"));
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