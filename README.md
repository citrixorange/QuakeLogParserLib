# QuakeLogParserLib

A Crate Lib for Parsing Quake Game Log

This crate is available at: https://crates.io/crates/quake_log_parser_lib

Docs at: https://docs.rs/quake_log_parser_lib/0.1.8/quake_log_parser_lib/

# Instructions

Add this crate as a dependency in your Rust App:

```cargo add quake_log_parser_lib```

# Author Notes

This Lib has been developed thinking to apply some of Clean Code Techniques:

- **Dependency Inversion Principle:** declaring an abstract interface which depends by business core. Develop a Concrete Class dependent by interface only. Injects Concrete Class into a Extern Crate Public Service Class.

- **Static Config File:** Some configurations instead being hard coded in source code are configured as a .json file present at STATIC_CONFIG.json file. Some of those configs are: regex patterns applied for log matching, error messages, key words used in Log file...

- **Feature Flags:** Dynamic Configuration which changes lib behaviour as: display kill by means stats in output, increases kill player score when  a player performs a self kill, decreases player kill score when killed...

- **Flexible Design:** This lib provide three kind of callbacks: success, warning and error. The use of callbacks make the lib flexible to be used by different distinct ways.

- **Future Improvements:** Check Issues Section in this Repo to check the nexts steps for this crate.

# Applications

A List of Applications that use this crate lib:

- **Cli App:** https://github.com/citrixorange/QuakeLogParserCliApp   
