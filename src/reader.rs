use crossbeam_channel::Sender;
use std::io::Read;

use crate::event::Event;

pub fn reader_thread(s: Sender<Event>, src: &mut dyn Read) {
    let mut buf = vec![0u8; 4];
    loop {
        src.read_exact(&mut buf).unwrap();
        s.send(Event::Logcat(format!(
            "read bytes 0x{:02x} 0x{:02x} 0x{:02x} 0x{:02x}",
            buf[0], buf[1], buf[2], buf[3]
        )))
        .unwrap();
    }
}
