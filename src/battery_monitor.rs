use std::io;
use battery;

#[derive(Debug)]
pub struct BatteryStats {
    state: String, 
    percentage: f32, 
}



pub fn get_battery_manager () -> Option<battery::Manager> {
    let manager = match battery::Manager::new(){
        Ok(manager) => Some(manager),
        Err(error) => {
            
            eprintln!("Unable to get battery manager\n\n\n {error}");
            
            return None;
        },
    };
    manager
}


pub fn get_battery(battery_manager: & battery::Manager) -> Option<battery::Battery> {
    let mut batteries_iterator = battery_manager.batteries().ok()?;

    let battery = match batteries_iterator.next() {
        Some(Ok(battery)) => Some(battery),
        Some(Err(error)) => {
            eprintln!("Unable to access battery information\n\n\n {error}");
            return None;
        }
        None => {
            eprintln!(
                "Unable to find any batteries\n\n\n {}",
                io::Error::from(io::ErrorKind::NotFound)
                );

            return None;
        }
    };
    battery
}


pub fn get_battery_stats(battery_manager: &battery::Manager,  battery: &mut battery::Battery) -> Option<BatteryStats> {
    // Function 
    battery_manager.refresh(battery).ok()?;
    return Some(
                BatteryStats {
                    state: battery.state().to_string(), 
                    percentage: (battery.state_of_charge()).value
                });
}