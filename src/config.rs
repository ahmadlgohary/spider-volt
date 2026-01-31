use std::{collections::HashMap, fs};
use serde_json;
use serde;
use serde::Deserialize;


#[derive(Debug, Deserialize)]
pub struct Config {
    notification_time: Option<u64>,

    high_battery_levels: Option<HashMap<String, BatteryNotification>>,

    low_battery_levels: Option<HashMap<String, BatteryNotification>>,
    
    charger_notifications: Option<ChargerNotification>
}

#[derive(Debug, Deserialize)]
pub struct BatteryNotification{
    message: String,
    notification_icon: Option<String>,
    notification_sound: Option<String>,
    persistent: Option<bool>
} 

#[derive(Debug, Deserialize)]
pub struct ChargerNotification {
    charging: Option<bool>,
    plugged_sound: Option<String>,
    charging_icon: Option<String>,
    discharging: Option<bool>,
    unplugged_sound: Option<String>,
    discharging_icon: Option<String>,
    persistent: Option<bool>
}



pub fn parse_json() -> Config {
    let file: String = fs::read_to_string("config.json").expect("Failed to open file");
    let config: Config = serde_json::from_str(&file).unwrap();
    config
}