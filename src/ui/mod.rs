use crossbeam_channel::Sender;
use ncurses;

mod input;

pub use input::input_thread;

pub struct UserInterface {
    sender: Sender<i32>,
}

impl UserInterface {
    pub fn new(s: Sender<i32>) -> UserInterface {
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
        ncurses::addch(ch as u32);
        ncurses::refresh();

        if ch == 'q' as i32 {
            self.sender.send(0).unwrap();
        }
    }
}
