use chrono::NaiveDateTime;
use crossbeam_channel::bounded;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::panic;
use std::rc::Rc;
use std::thread;

mod event;
mod logcat;
mod reader;
mod ui;

use crate::event::Event;
use crate::logcat::{LogcatEntry, LoggerEntry};
use crate::reader::reader_thread;
use crate::ui::{input_thread, UserInterface};

struct Context {
    tags: HashSet<Rc<String>>,
    process_names: HashMap<i32, Option<Rc<String>>>,
    logcat_entries: Vec<LogcatEntry>,
}

impl Context {
    fn new() -> Context {
        Context {
            tags: HashSet::new(),
            process_names: HashMap::new(),
            logcat_entries: Vec::new(),
        }
    }

    fn on_logger_entry(&mut self, raw: LoggerEntry) -> &LogcatEntry {
        if !self.tags.contains(&raw.tag) {
            self.tags.insert(Rc::new(raw.tag.clone()));
        }
        self.process_names.entry(raw.pid).or_insert(None); // TODO: call 'adb shell' instead
        let e = LogcatEntry {
            pid: raw.pid,
            tid: raw.tid,
            process_name: self
                .process_names
                .get(&raw.pid)
                .unwrap()
                .as_ref()
                .map(|v| Rc::clone(v)),
            timestamp: NaiveDateTime::from_timestamp(i64::from(raw.sec), raw.nsec),
            level: raw.level,
            tag: Rc::clone(self.tags.get(&raw.tag).unwrap()),
            text: raw.text,
        };
        self.logcat_entries.push(e);
        self.logcat_entries.last().unwrap()
    }
}

fn main() {
    let path = env::args_os().nth(1).expect("usage: lokatt <path-to-file>");

    panic::set_hook(Box::new(|panic_info| {
        UserInterface::shutdown();

        match panic_info.location() {
            Some(l) => eprintln!("{}:{}: panic:", l.file(), l.line()),
            None => eprintln!("panic:"),
        }
        eprintln!("{:#?}", panic_info);
    }));

    let (sender, receiver) = bounded(32);

    let s = sender.clone();
    let ui = UserInterface::init(s);

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

    let mut ctx = Context::new();
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
            Event::LoggerEntry(le) => {
                let logcat = ctx.on_logger_entry(le);
                ui.on_logcat(logcat);
            }
        }
    }

    UserInterface::shutdown();
}
