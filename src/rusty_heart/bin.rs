use serde::Serialize;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;
use systemstat::{saturating_sub_bytes, Platform, System};

mod crate::beat;

#[derive(Debug)]
struct Stats {
    networks: HashMap<String, Vec<String>>,
    battery: String,
    uptime: String,
    cpu_temp: String,
    memory: HashMap<String, String>
}

fn main() {
    let sys = System::new();
    let networks = HashMap::new();
    match sys.networks() {
        Ok(nets) => {
            for netif in nets {
                networks.insert(netif.name, sys.network_stats(&netif.name));
            }
        }
    }

    let battery = match sys.battery_life() {
        Ok(battery) => format!(
            "{}%",
            battery.remaining_capacity * 100.0,
        ),
        Err(x) => format!("\nBattery: error: {}", x),
    };

    let uptime = match sys.uptime() {
        Ok(uptime) => format!("{:?}", uptime),
        Err(x) => format!("{}", x),
    };

    let cpu_temp = match sys.cpu_temp() {

        Ok(temp) => format!("{}", temp),
        Err(e) => format!("{}", e),
    };

    let memory = HashMap::new();
    match sys.memory() {
       Ok(stats) => {
            memory.insert("used", saturating_sub_bytes(stats.total, stats.free));
            memory.insert("word", stats.total);
       },
       Err(e) => {
            memory.insert("used", format!("{}", e));
            memory.insert("word", format!("{}", e));
       }
    }
    
    let stats = Stats {
        memory,
        battery,
        uptime,
        cpu_temp,
        networks
   };
    println!("{:?}", stats); 
}
