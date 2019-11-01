use crossbeam_channel::{bounded, Sender};
use std::io;
use std::thread;

fn input_thread(s: Sender<u32>) {
    loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        let i = buffer.trim().parse::<u32>().unwrap();
        s.send(i).unwrap();
    }
}

fn main() {
    let (sender, receiver) = bounded(32);

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
            x => println!("received {}", x),
        }
    }
}
