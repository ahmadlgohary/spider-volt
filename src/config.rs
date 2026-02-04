use std::{collections::BTreeMap, fs};
use serde::Deserialize;
use notify_rust::Urgency;

#[derive(Debug, Deserialize)]
pub struct Config {
    notification_time: Option<i32>,
    pub(crate) high_battery_levels: Option<BTreeMap<u8, BatteryNotification>>,
    pub(crate) low_battery_levels: Option<BTreeMap<u8, BatteryNotification>>,
    pub(crate) charger_notifications: Option<ChargerNotification>
}

impl Config {
    pub fn parse_json() -> Config {
        let file: String = fs::read_to_string("config.json").expect("Failed to open file");
        let config: Config = serde_json::from_str(&file).unwrap();
        config
    }
    pub fn get_time(&self) -> i32 {
        if let Some(time) = self.notification_time {
            return time;
        }
        // default to 5000 ms (5 seconds)
        5000
    }
}

#[derive(Debug, Deserialize)]
pub struct BatteryNotification{
    message: String,
    notification_icon: Option<String>,
    notification_sound: Option<String>,
    urgent_level: Option<String>
}

impl BatteryNotification {
    pub fn get_message(&self) -> &str {
        self.message.as_str()
    }
    
    pub fn get_icon(&self) -> &str {
        if let Some(icon) = &self.notification_icon {
            return icon.as_str();
        }
        ""
    }
    
    pub fn get_sound(&self) -> &str {
        if let Some(sound) = &self.notification_sound {
            return sound.as_str();
        }
        ""
    }
    
    pub fn get_urgency(&self) -> Urgency {
        if let Some(urgent_level) = &self.urgent_level {
           match urgent_level.as_str().trim().try_into() {
                Ok(urgent) => return urgent,
                Err(_) => { 
                    eprint!("Unsupported Urgency Level.\t Resorting to Normal");
                    return Urgency::Normal;
                }
           }
        }
        Urgency::Normal
    }
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

impl ChargerNotification {

    pub fn get_charging(&self) -> bool {
        if let Some(send_charging_notifications) = self.charging{
            return send_charging_notifications;
        }
        // default to false
        false
    }

    pub fn get_plugged_sound(&self) -> &str {
        if let Some(plugged_sound) = &self.plugged_sound {
            return plugged_sound;
        }
        ""
    }

    pub fn get_charging_icon(&self) -> &str {
        if let Some(charging_icon) = &self.charging_icon {
            return charging_icon;
        }
        ""
    }

    pub fn get_discharging(&self) -> bool {
        if let Some(send_discharging_notifications) = self.discharging {
            return send_discharging_notifications;
        }
        // default to false
        false
    }

    pub fn get_unplugged_sound(&self) -> &str {
        if let Some(unplugged_sound) = &self.unplugged_sound {
            return unplugged_sound;
        }
        ""
    }

    pub fn get_discharging_icon(&self) -> &str {
        if let Some(discharging_icon) = &self.discharging_icon {
            return discharging_icon;
        }
        ""
    }

    pub fn get_urgency(&self) -> Urgency {
        if let Some(urgent_level) = &self.urgent_level {
           match urgent_level.as_str().trim().try_into() {
                Ok(urgent) => return urgent,
                Err(_) => { 
                    eprint!("Unsupported Urgency Level.\t Resorting to Normal");
                    return Urgency::Normal;
                }
           }
        }
        Urgency::Normal
    }

    pub fn get_bool_by_state(&self, state: &str) -> bool {
        if state == "Charging" {
            return self.get_charging();
        }
        self.get_discharging()
    }

    pub fn get_icon_by_state(&self, state: &str) -> &str {
        if state == "Charging" {
            return self.get_charging_icon();
        }
        self.get_discharging_icon()
    }

    pub fn get_sound_by_state(&self, state: &str) -> &str {
        if state == "Charging" {
            return self.get_plugged_sound();
        }
        self.get_unplugged_sound()
    }
}