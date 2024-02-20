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
        implementation::log_parser::ConcreteLogParser
    };

    #[test]
    async fn test() {
        let mut concrete_log_parser = ConcreteLogParser::new();
        let mut log_parser_service = LogParser::new(&mut concrete_log_parser);
        if let Ok(value) = log_parser_service.parse_file().await {
            println!("{}", value);
        } else {
            println!("Error Parsing Log File");
        }
        
    }

}