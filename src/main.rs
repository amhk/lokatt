use crossbeam_channel::bounded;
use std::env;
use std::fs::File;
use std::thread;

mod event;
mod logcat;
mod reader;
mod ui;

use crate::event::Event;
use crate::reader::reader_thread;
use crate::ui::{input_thread, UserInterface};

fn main() {
    let path = env::args_os().nth(1).expect("usage: lokatt <path-to-file>");
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

    let mut src = File::open(path).unwrap();
    let s = sender.clone();
    thread::Builder::new()
        .name("reader".to_string())
        .spawn(move || {
            reader_thread(s, &mut src);
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
            Event::Logcat(s) => {
                ui.on_logcat(&s);
            }
        }
    }

    ui.shutdown();
}
