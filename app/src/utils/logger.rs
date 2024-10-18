use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt;
use crate::utils::logger;

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
    Critical,
}

impl LogLevel {
    fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Debug => "Debug",
            LogLevel::Info => "Info",
            LogLevel::Warn => "Warn",
            LogLevel::Error => "Error",
            LogLevel::Critical => "Critical",
        }
    }
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogMessage {
    origin: Vec<String>,
    message: String,
    level: LogLevel,
}

impl fmt::Display for LogMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let origin = self.origin.join("::");
        write!(f, "[{} {}] {}", self.level.as_str(), origin, self.message)
    }
}

impl LogMessage {
    fn origin(&self) -> &Vec<String> {
        &self.origin
    }
    fn message(&self) -> &String {
        &self.message
    }
    fn level(&self) -> &LogLevel {
        &self.level
    }
}

pub trait AnyLogger {}
pub struct SlaveLogger {
    origin: Vec<String>,
    parent: Box<dyn AnyLoggerSignal>,
    master: LoggerSignal<MasterLogger>,
}
impl AnyLogger for SlaveLogger {}

pub struct LoggerIterator<'a> {
    logs: &'a Vec<LogMessage>,
    origin: &'a [String],
    index: usize,
}
impl<'a> Iterator for LoggerIterator<'a> {
    type Item = &'a LogMessage;

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < self.logs.len() {
            let log = &self.logs[self.index];
            self.index += 1;
            if log.origin.starts_with(self.origin) {
                return Some(log);
            }
        }
        None
    }
}
pub struct MasterLogger {
    logs: Vec<LogMessage>,
}
impl MasterLogger {
    fn iter_master(&self) -> LoggerIterator {
        LoggerIterator {
            logs: &self.logs,
            origin: &[],
            index: 0,
        }
    }
    fn iter_slave<'a>(&'a self, origin: &'a [String]) -> LoggerIterator<'a> {
        LoggerIterator {
            logs: &self.logs,
            origin,
            index: 0,
        }
    }
}
impl AnyLogger for MasterLogger {}

pub trait AnyLoggerSignal: Send + Sync {
    fn new_sub_logger(&self, name: String) -> LoggerSignal<SlaveLogger>;
    fn log(&self, message: String, level: LogLevel);
    fn list(&self) -> Vec<LogMessage>;
    fn with_iter(&self, c: fn(LoggerIterator));
    fn as_any(&self) -> Box<dyn AnyLoggerSignal>;
    fn context(&self) -> LoggerContext{
        LoggerContext{logger:self.as_any()}
    }
}

pub struct LoggerSignal<T: AnyLogger> {
    signal: RwSignal<T>,
}
impl Default for LoggerSignal<MasterLogger> {
    fn default() -> Self {
        LoggerSignal {
            signal: RwSignal::new(MasterLogger { logs: vec![] }),
        }
    }
}

impl<T: AnyLogger> Clone for LoggerSignal<T> {
    fn clone(&self) -> Self {
        LoggerSignal {
            signal: self.signal.clone(),
        }
    }
}

impl AnyLoggerSignal for LoggerSignal<MasterLogger> {
    fn new_sub_logger(&self, name: String) -> LoggerSignal<SlaveLogger> {
        LoggerSignal {
            signal: RwSignal::new(SlaveLogger {
                origin: vec![name],
                parent: Box::new(self.clone()),
                master: self.clone(),
            }),
        }
    }
    fn log(&self, message: String, level: LogLevel) {
        self.signal.update(|master| {
            master.logs.push(LogMessage {
                origin: vec![],
                message,
                level,
            })
        })
    }
    fn list(&self) -> Vec<LogMessage> {
        self.signal.with(|master| master.logs.clone())
    }

    fn with_iter(&self, c: fn(LoggerIterator)) {
        self.signal.with(|master| c(master.iter_master()));
    }
    fn as_any(&self) -> Box<dyn AnyLoggerSignal> {
        Box::new(self.clone())
    }
}

impl LoggerSignal<SlaveLogger> {
    fn parent(&self) -> Option<Box<dyn AnyLoggerSignal>> {
        Some(self.signal.with(|slave| slave.parent.as_any()))
    }
    fn master(&self) -> LoggerSignal<MasterLogger> {
        self.signal.with(|slave| slave.master.clone())
    }
}

impl AnyLoggerSignal for LoggerSignal<SlaveLogger> {
    fn new_sub_logger(&self, name: String) -> LoggerSignal<SlaveLogger> {
        self.signal.with(|slave| {
            let mut origin = slave.origin.clone();
            origin.push(name);
            LoggerSignal {
                signal: RwSignal::new(SlaveLogger {
                    origin,
                    parent: Box::new(self.clone()),
                    master: slave.master.clone(),
                }),
            }
        })
    }
    fn log(&self, message: String, level: LogLevel) {
        self.signal.with(|slave| {
            slave.master.signal.update(|master| {
                master.logs.push(LogMessage {
                    origin: slave.origin.clone(),
                    message,
                    level,
                })
            })
        })
    }

    fn list(&self) -> Vec<LogMessage> {
        self.signal.with(|slave| {
            slave
                .master
                .signal
                .with(|master| master.iter_slave(&slave.origin).cloned().collect())
        })
    }
    fn with_iter(&self, c: fn(LoggerIterator)) {
        self.signal.with(|slave| {
            slave
                .master
                .signal
                .with(|master| c(master.iter_slave(&slave.origin)))
        })
    }
    fn as_any(&self) -> Box<dyn AnyLoggerSignal> {
        Box::new(self.clone())
    }
}
pub struct LoggerContext{
    logger:Box<dyn AnyLoggerSignal>
}

impl Clone for LoggerContext{
    fn clone(&self) -> Self {
        LoggerContext{
            logger:self.logger.as_any()
        }
    }
}