use crossbeam_channel::Sender;
use ncurses;
use std::time::Duration;

use crate::event::Event;
use crate::logcat::LogcatEntry;

mod input;
mod refresh;
mod window;

use refresh::RefreshThread;
use window::Window;

pub use input::input_thread;

pub struct UserInterface {
    sender: Sender<Event>,
    main_window: Window,
    refresh_thread: RefreshThread,
}

impl UserInterface {
    pub fn init(s: Sender<Event>) -> UserInterface {
        let s2 = s.clone();
        let refresh_thread = RefreshThread::new(s2, Duration::from_millis(16));

        ncurses::initscr();
        ncurses::curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE);
        ncurses::cbreak();
        ncurses::noecho();
        ncurses::refresh();

        let mut maxy = 0;
        let mut maxx = 0;
        ncurses::getmaxyx(ncurses::stdscr(), &mut maxy, &mut maxx);
        let main_window = Window::new(0, 0, maxx, maxy);

        UserInterface {
            sender: s,
            main_window,
            refresh_thread,
        }
    }

    pub fn shutdown() {
        ncurses::endwin();
    }

    pub fn on_key(&self, ch: i32) {
        if ch == 'q' as i32 {
            self.sender
                .send(Event::Command("quit".to_string()))
                .unwrap();
        }
    }

    pub fn on_logcat(&self, entry: &LogcatEntry) {
        self.main_window.add_str(&format!(
            "{:?} {:20} {}\n",
            entry.timestamp,
            entry.tag,
            entry.text.trim()
        ));
        self.refresh_thread.schedule_refresh();
    }

    pub fn on_refresh(&self) {
        ncurses::doupdate();
    }
}
