use crossbeam_channel::Sender;
use ncurses;

use crate::event::Event;

pub fn input_thread(s: Sender<Event>) {
    loop {
        let ch = ncurses::getch();
        s.send(Event::KeyCode(ch)).unwrap();
    }
}
