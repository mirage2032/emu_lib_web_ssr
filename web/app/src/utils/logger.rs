#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Log {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub level: LogLevel,
    pub short_message: &'static str,
    pub message: String,
}

pub struct LogStore {
    logs: Vec<Log>,
}

impl Default for LogStore {
    fn default() -> Self {
        LogStore { logs: Vec::new() }
    }
}

impl LogStore {
    pub fn log(&mut self, level: LogLevel, short_message: &'static str, message: String) {
        let log = Log {
            timestamp: chrono::Utc::now(),
            level,
            short_message,
            message,
        };
        self.logs.push(log);
    }

    pub fn log_info(&mut self, short_message: &'static str, message: String) {
        self.log(LogLevel::Info, short_message, message);
    }

    pub fn log_warning(&mut self, short_message: &'static str, message: String) {
        self.log(LogLevel::Warning, short_message, message);
    }

    pub fn log_error(&mut self, short_message: &'static str, message: String) {
        self.log(LogLevel::Error, short_message, message);
    }

    pub fn last_log(&self) -> Option<&Log> {
        self.logs.last()
    }
    pub fn get_logs(&self) -> &Vec<Log> {
        &self.logs
    }
}
