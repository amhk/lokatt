use crossbeam_channel::Sender;
use ncurses;

use crate::event::Event;
use crate::logcat::LogcatEntry;

mod input;

pub use input::input_thread;

pub struct UserInterface {
    sender: Sender<Event>,
}

impl UserInterface {
    pub fn new(s: Sender<Event>) -> UserInterface {
        UserInterface { sender: s }
    }

    pub fn init(&self) {
        ncurses::initscr();
        ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE);
        ncurses::cbreak();
        ncurses::noecho();
        ncurses::refresh();
    }

    pub fn shutdown(&self) {
        ncurses::endwin();
    }

    pub fn on_key(&self, ch: i32) {
        self.clear_screen_if_needed();
        ncurses::addch(ch as ncurses::chtype);
        ncurses::addch('\n' as ncurses::chtype);
        ncurses::refresh();

        if ch == 'q' as i32 {
            self.sender
                .send(Event::Command("quit".to_string()))
                .unwrap();
        }
    }

    pub fn on_logcat(&self, entry: &LogcatEntry) {
        self.clear_screen_if_needed();
        ncurses::addstr(&format!(
            "{:?} {:20} {}\n",
            entry.timestamp,
            entry.tag,
            entry.text.trim()
        ));
        ncurses::refresh();
    }

    fn clear_screen_if_needed(&self) {
        let mut y = 0;
        let mut x = 0;
        ncurses::getyx(ncurses::stdscr(), &mut y, &mut x);

        let mut maxy = 0;
        let mut maxx = 0;
        ncurses::getmaxyx(ncurses::stdscr(), &mut maxy, &mut maxx);

        if y > maxy - 2 {
            ncurses::clear();
        }
    }
}
