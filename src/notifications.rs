use std::{thread, time::Duration};
use notify_rust::{
    Notification,
    Urgency,
    Hint
};
use crate::config::BatteryNotification;

pub fn update_notification(id: u32, battery_level: &i32, notification_information: &BatteryNotification) {
    Notification::new()
    .summary("test")
    .body("test body")
    .timeout(2000) 
    .urgency(Urgency::Normal)
    .id(id)
    .hint(Hint::Transient(true))
    .show()
    .unwrap()
    .update(); 
} 

pub fn create_notification_id() -> u32 {
    let handle  = Notification::new()
    .show()
    .unwrap();
    let id = handle.id();   
    handle.close();  
    thread::sleep(Duration::from_secs(1));
    id
} 


pub fn testing_notification(id: u32, str: &str) {
    Notification::new()
    .summary(str)
    .body("test body")
    .timeout(2000) 
    .urgency(Urgency::Normal)
    .id(id)
    .hint(Hint::Transient(true))
    .show()
    .unwrap()
    .update(); 
} 