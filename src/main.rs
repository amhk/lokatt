use crossbeam_channel::bounded;
use std::thread;

mod ui;

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
            0 => break,
            x => ui.on_key(x),
        }
    }

    ui.shutdown();
}
