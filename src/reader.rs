use crossbeam_channel::Sender;
use std::io::Read;

use crate::event::Event;
use crate::logcat::parse_logger_entry;

pub fn reader_thread<R>(s: Sender<Event>, src: &mut R)
where
    R: Read,
{
    loop {
        match parse_logger_entry(src) {
            Ok(entry) => {
                s.send(Event::Logcat(format!("{:20} {}", entry.tag, entry.text)))
                    .unwrap();
            }
            Err(e) => {
                s.send(Event::Logcat(format!("read failed: {:?}", e)))
                    .unwrap();
                break;
            }
        }
    }
}
