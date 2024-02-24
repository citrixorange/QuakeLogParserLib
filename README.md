# QuakeLogParserLib

A Crate Lib for Parsing Quake Game Log

This crate is available at: https://crates.io/crates/quake_log_parser_lib

Docs at: https://docs.rs/quake_log_parser_lib/0.1.9/quake_log_parser_lib/

Check Time Management of this Project at: https://wakatime.com/@caiobosco/projects/umkpbnqgrz

# Instructions

Add this crate as a dependency in your Rust App:

```cargo add quake_log_parser_lib```

# Author Notes

This Lib has been developed thinking to present Author Clean Code Skills:

- **Dependency Inversion Principle:** declaring an abstract interface which depends on business core only. Develop a Concrete Class which depends on interface only. Injects Concrete Class into a Extern Crate Public Service Class.

- **Static Config File:** Some configurations instead being hard coded in source code are configured as a .json file present at STATIC_CONFIG.json file. Some of those configs are: regex patterns applied for log matching, error messages, key words used in Log file...

- **Feature Flags:** Dynamic Configuration which changes lib behaviour as: display kill by means stats in output, increases kill player score when  a player performs a self kill, decreases player kill score when killed...

- **Flexible Design:** This lib provide three kind of callbacks: success, warning and error. The use of callbacks make the lib flexible to be used by different distinct ways.

- **Future Improvements:** Check Issues Section in this Repo to check the nexts steps for this crate.

# Applications

A List of Applications that use this crate lib:

- **Cli App:** https://github.com/citrixorange/QuakeLogParserCliApp

- **Log Parsing Daemon Agent:** A Daemon Process detects when a new log file is created at server matches log folder. Parsed Stats are fetched by Prometheus Server and displayed by a Graphana Instance. Coming soon...

- **Log Parse Horizontal Scaling using RabbitMQ, AWS S3, Kubernetes:** Log Files are stored and addressed in a AWS S3 instance. RabbitMQ publishes a message containing Log File S3 address. Kubernetes HPA(Horizontal Pod Autoscaling) uses RabbitMQ queue depth as main metrics for autoscaling. New Pods created pull Log File S3 Address from RabbitMQ, parse match log and send stats to a Prometheus Server. Coming soon...    
