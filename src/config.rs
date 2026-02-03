use std::{collections::BTreeMap, fs};
use serde_json;
use serde;
use serde::Deserialize;


#[derive(Debug, Deserialize)]
pub struct Config {
    notification_time: Option<u64>,

    pub(crate) high_battery_levels: Option<BTreeMap<u8, BatteryNotification>>,

    pub(crate) low_battery_levels: Option<BTreeMap<u8, BatteryNotification>>,
    
    charger_notifications: Option<ChargerNotification>
}

#[derive(Debug, Deserialize)]
pub struct BatteryNotification{
    message: String,
    notification_icon: Option<String>,
    notification_sound: Option<String>,
    urgent_level: Option<String>
} 

#[derive(Debug, Deserialize)]
pub struct ChargerNotification {
    charging: Option<bool>,
    plugged_sound: Option<String>,
    charging_icon: Option<String>,
    discharging: Option<bool>,
    unplugged_sound: Option<String>,
    discharging_icon: Option<String>,
    urgent_level: Option<String>
}

impl Config {
    pub fn parse_json() -> Config {
        let file: String = fs::read_to_string("config.json").expect("Failed to open file");
        let config: Config = serde_json::from_str(&file).unwrap();
        config
    }
}
