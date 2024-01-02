#![allow(unused)]
use chrono::{NaiveTime, Utc};
use clokwerk::{Job, Scheduler, TimeUnits};
use hdc_shared::models::ingestion_container::IngestionPacket;
use hdc_shared::models::tasklist::Tasklist;
use std::thread;
use std::sync::{mpsc::channel, RwLock};
use std::time::{Duration, SystemTime};
use once_cell::sync::Lazy;

mod collector;
mod models;
mod taskforce;

use taskforce::{taskforce, tasklist_observer};

fn main() {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    let mut scheduler = Scheduler::with_tz(chrono::Utc);

    const COLLECTOR_INTERVAL: u64 = 30;
    const TASK_UPDATE_INTERVAL: u64 = 300;
    const SECONDS_IN_DAY: u64 = 60 * 24;

    static TASKLIST: Lazy<RwLock<Tasklist>> = Lazy::new(|| {
        RwLock::new(Tasklist::new())
    });

    let now = Utc::now();
    let sys_time_now = SystemTime::now();
    let offset = sys_time_now
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs()
        % COLLECTOR_INTERVAL;

    let taskforce_start: NaiveTime = now.time() + Duration::from_secs(30 - offset);
    let observer_start: NaiveTime = now.time() + Duration::from_secs(15 - (offset - 15));

    scheduler
        .every(1.day())
        .at_time(taskforce_start)
        .repeating_every((COLLECTOR_INTERVAL as u32).seconds())
        .times((SECONDS_IN_DAY / COLLECTOR_INTERVAL - 1) as usize)
        .run(|| {
            let _ = taskforce(&TASKLIST);
        });
    scheduler
        .every(1.day())
        .at_time(observer_start)
        .repeating_every((TASK_UPDATE_INTERVAL as u32).seconds())
        .times((SECONDS_IN_DAY / TASK_UPDATE_INTERVAL - 1) as usize)
        .run( || {
            let _ = tasklist_observer(&TASKLIST);
        });

    loop {
        scheduler.run_pending();
        thread::sleep(Duration::from_millis(10));
    }
}
