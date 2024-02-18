mod errors;
mod interface;
mod death_causes;
mod implementation;
mod service;

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
        log_parser_service.parse_file().await;
    }

}