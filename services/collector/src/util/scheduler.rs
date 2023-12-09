use std::thread::spawn;
use std::io;
use crate::util::collector::extract;

pub fn taskforce() -> io::Result<()> { 
    let url1: String = "http:192.168.0.184/status".to_string();
    let tasks = ConcurrentTasks{
        tasks: vec![url1]
    };
    let mut thread_handles = Vec::new();

    for task in tasks.tasks {
        thread_handles.push(
        spawn(move || extract(task)))
      }
    for handle in thread_handles {
        let _ = handle.join().unwrap();}
    Ok(())
}
    
pub struct ConcurrentTasks {
    pub tasks: Vec<String>
}

