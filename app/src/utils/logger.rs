use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt;

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

pub struct Logger {
    origin: Vec<String>,
    parent: Option<LoggerSignal>,
    store: LoggerStoreSignal,
}

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
struct LoggerStore {
    name: String,
    logs: Vec<LogMessage>,
}
impl LoggerStore {
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

pub trait Logging: Send + Sync {
    fn new_sub_logger(&self, name: &str) -> LoggerSignal;
    fn log(&self, message: &str, level: LogLevel);
    fn list(&self) -> Vec<LogMessage>;
    fn with_iter(&self, c: fn(LoggerIterator));
}

#[derive(Clone)]
pub struct LoggerSignal {
    signal: RwSignal<Logger>,
}

#[derive(Clone)]
pub struct LoggerStoreSignal {
    signal: RwSignal<LoggerStore>,
}

impl LoggerStoreSignal {
    pub fn new(name: &str) -> Self {
        LoggerStoreSignal {
            signal: RwSignal::new(LoggerStore {
                name: name.to_string(),
                logs: vec![],
            }),
        }
    }

    fn as_sub_logger(&self) -> LoggerSignal {
        self.signal.with(|master| LoggerSignal {
            signal: RwSignal::new(Logger {
                origin: vec![],
                parent: None,
                store: self.clone(),
            }),
        })
    }
}

impl From<LoggerStoreSignal> for LoggerSignal {
    fn from(value: LoggerStoreSignal) -> Self {
        value.as_sub_logger()
    }
}

impl LoggerSignal {
    fn parent(&self) -> Option<LoggerSignal> {
        self.signal.with(|slave| slave.parent.clone())
    }
    fn master(&self) -> LoggerStoreSignal {
        self.signal.with(|slave| slave.store.clone())
    }
}

impl Logging for LoggerSignal {
    fn new_sub_logger(&self, name: &str) -> LoggerSignal {
        self.signal.with(|slave| {
            let mut origin = slave.origin.clone();
            origin.push(name.to_string());
            LoggerSignal {
                signal: RwSignal::new(Logger {
                    origin,
                    parent: Some(self.clone()),
                    store: slave.store.clone(),
                }),
            }
        })
    }
    fn log(&self, message: &str, level: LogLevel) {
        self.signal.with(|slave| {
            slave.store.signal.update(|master| {
                master.logs.push(LogMessage {
                    origin: slave.origin.clone(),
                    message: message.to_string(),
                    level,
                })
            })
        })
    }

    fn list(&self) -> Vec<LogMessage> {
        self.signal.with(|slave| {
            slave
                .store
                .signal
                .with(|master| master.iter_slave(&slave.origin).cloned().collect())
        })
    }
    fn with_iter(&self, c: fn(LoggerIterator)) {
        self.signal.with(|slave| {
            slave
                .store
                .signal
                .with(|master| c(master.iter_slave(&slave.origin)))
        })
    }
}

impl Logging for LoggerStoreSignal {
    fn new_sub_logger(&self, name: &str) -> LoggerSignal {
        self.signal.with(|master| LoggerSignal {
            signal: RwSignal::new(Logger {
                origin: vec![master.name.clone(), name.to_string()],
                parent: None,
                store: self.clone(),
            }),
        })
    }
    fn log(&self, message: &str, level: LogLevel) {
        self.signal.update(|master| {
            master.logs.push(LogMessage {
                origin: vec![],
                message: message.to_string(),
                level,
            })
        })
    }

    fn list(&self) -> Vec<LogMessage> {
        self.signal
            .with(|master| master.iter_master().cloned().collect())
    }
    fn with_iter(&self, c: fn(LoggerIterator)) {
        self.signal.with(|master| c(master.iter_master()))
    }
}
