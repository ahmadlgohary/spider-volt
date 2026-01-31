use std::{thread, time::Duration};
extern crate battery;


mod config;
mod battery_monitor;

fn main() {

    let manager = battery_monitor::get_battery_manager();

    if let Some(manager) = manager {
        let battery = battery_monitor::get_battery(&manager);
        if let Some(mut battery) = battery {
            loop {
                let stats = battery_monitor::get_battery_stats(&manager, &mut battery);
                if stats.is_some(){println!("{:?}", stats);}
                thread::sleep(Duration::from_secs(1));
            }
        }
        // exit program for now
        // TODO: implement retries
        else {std::process::exit(1);}
        
    }
    // exit program for now
    // TODO: implement retries
    else {std::process::exit(1);}
}