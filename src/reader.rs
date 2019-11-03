use crossbeam_channel::Sender;
use std::io::Read;

use crate::event::Event;
use crate::logcat::parse_logger_entry;

pub fn reader_thread<R>(s: Sender<Event>, src: &mut R)
where
    R: Read,
{
    while let Ok(entry) = parse_logger_entry(src) {
        s.send(Event::LoggerEntry(entry)).unwrap();
    }
}
