use crate::logcat::LoggerEntry;

#[derive(Debug, Eq, PartialEq)]
pub enum Event {
    Command(String),
    KeyCode(i32),
    LoggerEntry(LoggerEntry),
}
