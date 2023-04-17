pub struct LoggerConfig {
    logger_name: String,
    service_name: String,
    forward_errors_to_logs: bool,
    session_sample_rate: u32,
}

pub enum LogLevel {
    Info,
    DetailedInfo,
    Error,
    Warning,
}

pub type Dictionary<'a> = (&'a str, &'a str);

pub enum ActionType<'a> {
    BeginLogging(LoggerConfig, Option<Dictionary<'a>>),
    DdLogEvent(String, LogLevel, String),
    GeolocationPassed(String, String, String),
    LocationInProgress(Option<String>, Option<String>),
    GeolocationRejected(String, String, String),
    LocationFailure(Option<String>, Option<String>, String),
}
