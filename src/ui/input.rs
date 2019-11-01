use crossbeam_channel::Sender;
use ncurses;

pub fn input_thread(s: Sender<i32>) {
    loop {
        s.send(ncurses::getch()).unwrap();
    }
}
