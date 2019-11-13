use crossbeam_channel::Sender;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;

use crate::event::Event;

fn refresh_thread(lock: &Mutex<bool>, cvar: &Condvar, sender: Sender<Event>, delay: Duration) {
    loop {
        let mut signal = lock.lock().unwrap();
        while !*signal {
            signal = cvar.wait(signal).unwrap();
        }
        *signal = false;
        drop(signal);

        thread::sleep(delay);
        sender.send(Event::Refresh).unwrap();
    }
}

pub struct RefreshThread {
    _thread: thread::JoinHandle<()>,
    arc: Arc<(Mutex<bool>, Condvar)>,
}

impl RefreshThread {
    pub fn new(sender: Sender<Event>, delay: Duration) -> RefreshThread {
        #[allow(clippy::mutex_atomic)] // Mutex<bool> is actually needed here
        let arc = Arc::new((Mutex::new(false), Condvar::new()));
        let arc2 = arc.clone();

        let _thread = thread::Builder::new()
            .name("refresh".to_string())
            .spawn(move || {
                let (lock, cvar) = &*arc2;
                refresh_thread(lock, cvar, sender, delay);
            })
            .unwrap();

        RefreshThread { _thread, arc }
    }

    pub fn schedule_refresh(&self) {
        let (lock, cvar) = &*self.arc;
        let mut signal = lock.lock().unwrap();
        *signal = true;
        cvar.notify_one();
    }
}

#[cfg(test)]
mod tests {
    use crate::event::Event;
    use crate::ui::refresh::RefreshThread;
    use crossbeam_channel::{bounded, RecvTimeoutError};
    use std::time::{Duration, Instant};

    #[test]
    fn refresh_event_after_expected_delay() {
        let (s, r) = bounded(32);
        let delay = Duration::from_millis(10);
        let t = RefreshThread::new(s, delay);
        let timestamp = Instant::now();
        t.schedule_refresh();
        assert_eq!(r.recv().unwrap(), Event::Refresh);
        assert!(timestamp.elapsed() >= delay);
    }

    #[test]
    fn multiple_calls_to_schedule_refresh_only_single_event() {
        let (s, r) = bounded(32);
        let t = RefreshThread::new(s, Duration::from_millis(10));
        for _ in 0..2 {
            for _ in 0..10 {
                t.schedule_refresh();
            }
            assert_eq!(r.recv().unwrap(), Event::Refresh);
            assert_eq!(
                r.recv_timeout(Duration::from_nanos(1)),
                Err(RecvTimeoutError::Timeout)
            );
        }
    }
}
