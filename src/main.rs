use crossbeam_channel::bounded;
use std::thread;

mod event;
mod ui;

use crate::event::Event;
use crate::ui::{input_thread, UserInterface};

fn main() {
    let (sender, receiver) = bounded(32);

    let s = sender.clone();
    let ui = UserInterface::new(s);
    ui.init();

    let s = sender.clone();
    thread::Builder::new()
        .name("input".to_string())
        .spawn(move || {
            input_thread(s);
        })
        .unwrap();

    drop(sender);

    loop {
        match receiver.recv().unwrap() {
            Event::Command(cmd) => {
                if cmd == "quit" {
                    break;
                }
            }
            Event::KeyCode(ch) => {
                ui.on_key(ch);
            }
        }
    }

    ui.shutdown();
}
