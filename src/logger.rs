use std::{fmt::Display};
use colored::Colorize;

use crate::app::LogLevel;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum NoteFor {
    Info = 0,
    Warn = 1,
    Error = 2,
}

enum Log {
    Info(String),
    Note(String, NoteFor),
    Warn(String),
    Error(String),
}

impl Log {
    fn level(&self) -> i32 {
        match &self {
            Log::Info(_) => 0,
            Log::Warn(_) => 1,
            Log::Error(_) => 2,
            Log::Note(_, n_for) => *n_for as i32,
        }
    }
}

impl Display for Log {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            &Log::Info(msg) => write!(f, "{} {}", "Info".bright_black().bold(), msg),
            &Log::Note(msg, _) => write!(f, "{} {}", "Note".bright_black().bold(), msg),
            &Log::Warn(msg) => write!(f, "{} {}", "Warn".yellow().bold(), msg),
            &Log::Error(msg) => write!(f, "{} {}", "Error".red().bold(), msg),
        }
    }
}

pub struct Logger {
    logs: Vec<Log>,
    log_level: LogLevel,
}

impl Logger {
    pub fn new(log_level: LogLevel) -> Logger {
        Logger {
            logs: Vec::new(),
            log_level,
        }
    }
    pub fn info(&mut self, msg: String) {
        self.logs.push(Log::Info(msg));
    }
    pub fn note(&mut self, msg: String, n_for: NoteFor) {
        self.logs.push(Log::Note(msg, n_for));
    }
    pub fn warn(&mut self, msg: String) {
        self.logs.push(Log::Warn(msg));
    }
    pub fn error(&mut self, msg: String) {
        self.logs.push(Log::Error(msg));
    }
    pub fn len(&self) -> usize {
        self.logs.len()
    }
}

impl Display for Logger {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for log in &self.logs {
            if self.log_level as i32 > log.level() {
                continue
            } 
            if let Err(e) = writeln!(f, "{}", log) {
                return Err(e)
            }
        }
        Ok(())
    }
}