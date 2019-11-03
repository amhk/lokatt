use crate::logcat::LoggerEntry;

pub enum Event {
    Command(String),
    KeyCode(i32),
    LoggerEntry(LoggerEntry),
}
