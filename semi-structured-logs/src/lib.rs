#![allow(unused)]

#[derive(Clone, PartialEq, Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
    Debug
}

/// primary function for emitting logs
pub fn log(level: LogLevel, message: &str) -> String {
    match level {
        LogLevel::Info => info(message),
        LogLevel::Warning => warn(message),
        LogLevel::Error => error(message),
        LogLevel::Debug => debug(message),
    }
}

pub fn info(message: &str) -> String {
    let message = String::from(format!("[INFO]: {}", message));
    message
}

pub fn warn(message: &str) -> String {
    let message = String::from(format!("[WARNING]: {}", message));
    message
}

pub fn error(message: &str) -> String {
    let message = String::from(format!("[ERROR]: {}", message));
    message
}

pub fn debug(message: &str) -> String {
    let message = String::from(format!("[DEBUG]: {}", message));
    message
}
