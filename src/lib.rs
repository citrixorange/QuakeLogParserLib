#![recursion_limit = "256"]

mod errors;
mod interface;
mod death_causes;
mod implementation;
mod service;
mod config;

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

        let mut concrete_log_parser = ConcreteLogParser::new();
        let mut log_parser_service = LogParser::new(&mut concrete_log_parser);
        if let Ok(value) = log_parser_service.parse_file().await {
            println!("{}", value);
        } else {
            println!("Error Parsing Log File");
        }
        
    }

}