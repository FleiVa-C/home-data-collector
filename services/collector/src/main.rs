use clokwerk::{Scheduler, TimeUnits, Job};
use std::time::{Duration, SystemTime};
use std::thread;
use chrono::{Utc, NaiveTime};

mod models;
mod util;

use util::scheduler::*;

fn main() {
    let mut scheduler = Scheduler::with_tz(chrono::Utc);

    const INTERVAL: u64 = 30;
    const SECONDS_IN_DAY: u64 = 60*24;

    let sys_time_now = SystemTime::now();
    let offset = sys_time_now.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs()%INTERVAL;

    let now = Utc::now();
    let start: NaiveTime = now.time() + Duration::from_secs(60-offset); 
    scheduler
        .every(1.day())
        .at_time(start)
        .repeating_every((INTERVAL as u32).seconds())
        .times((SECONDS_IN_DAY/INTERVAL) as usize)
        .run(|| {let _ = taskforce();});

    let thread_handle = scheduler.watch_thread(Duration::from_millis(100));
    thread::sleep(Duration::from_secs(1000));

    thread_handle.stop();
}

