use std::{collections::LinkedList, io::Write};

use crate::frontend::ast::LineInfo;

pub enum LoggerType {
    ONLY_ERRORS,
    ERROR_AND_WARNS,
}

pub struct Logger {
    writer: Box<dyn Write>,
    loger_type: LoggerType,
    file_name: Option<String>,
    logs: LinkedList<Log>,
}

impl Logger {
    pub fn new(logger_type: LoggerType, writer: Box<dyn Write>) -> Self {
        Self {
            writer: writer,
            loger_type: logger_type,
            file_name: None,
            logs: LinkedList::new(),
        }
    }

    pub fn error(&mut self, line: u32, token_index: u32, log: String) {
        self.logs.push_back(Log {line, token_index, log});
        println!()
    }
}

pub struct Log {
    line: u32,
    token_index: u32,
    log: String,
}

#[macro_export]
macro_rules! error_log {
    ($logger:expr, $line:expr, $index:expr, $($arg:tt)+) => {{
        $logger.error($line, $index, format!($($arg)*))
    }}
}