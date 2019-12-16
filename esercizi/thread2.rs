// threads1.rs
// Make this compile! Scroll down for hints :) The idea is the thread
// spawned on line 19 is completing jobs while the main thread is
// monitoring progress until 10 jobs are completed. If you see 6 lines
// of "waiting..." and the program ends without timing out the playground,
// you've got it :)

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let status_shared = status.clone();
    thread::spawn(move || {
        for _ in 0..10 {
            thread::sleep(Duration::from_millis(250));
            let mut jobs = status_shared.lock().unwrap(); 
            jobs.jobs_completed += 1;
        }
    });
    
    
    while status.lock().unwrap().jobs_completed < 10 {
        println!("waiting... ");
        thread::sleep(Duration::from_millis(500));
    }
}

//con RwLock invece che il Mutex (quindi differenzia tra write e read) loca diversamente posso
//loccare  contemporaneamente in lettura e solo una volta a botta in scrittura

use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    let status = Arc::new(RwLock::new(JobStatus { jobs_completed: 0 }));
    let status_shared = status.clone();
    thread::spawn(move || {
        for _ in 0..10 {
            thread::sleep(Duration::from_millis(250));
            let mut jobs = status_shared.write().unwrap(); 
            jobs.jobs_completed += 1;
        }
    });
    
    
    while status.read().unwrap().jobs_completed < 10 {
        println!("waiting... ");
        thread::sleep(Duration::from_millis(500));
    }
}

