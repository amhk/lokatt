use ncurses;

pub struct Window {
    handle: ncurses::WINDOW,
}

impl Window {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Window {
        let handle = ncurses::newwin(height, width, y, x);
        ncurses::scrollok(handle, true);
        Window { handle }
    }

    pub fn add_str(&self, s: &str) {
        ncurses::waddstr(self.handle, s);
        self.mark_dirty();
    }

    fn mark_dirty(&self) {
        ncurses::wnoutrefresh(self.handle);
    }
}
