use serde::Serialize;
use std::collections::HashMap;
use std::error::Error;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;
use systemstat::{saturating_sub_bytes, Platform, System};

//mod crate::beat;
use onbeat::beat;

#[derive(Debug, Serialize)]
struct Stats {
    networks: HashMap<String, String>,
    battery: String,
    uptime: String,
    cpu_temp: String,
    memory: HashMap<String, String>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let stats = Stats {
        networks: HashMap::new(),
        battery: String::new(),
        uptime: String::new(),
        cpu_temp: String::new(),
        memory: HashMap::new(),
    };
    let mut sys = System::new();
    let index = "http://localhost:9200/heartbeat/_doc";
    let time_stamp = Some(String::from("dd/mm/YYYY HH:MM:SS"));
    let mtx = Arc::from(RwLock::from(stats));
    let wlock = mtx.write().expect("RwLock is poisned");
    update_stats(wlock, &mut sys);
    let my_beat = beat::Beat::new(index, mtx.clone(), time_stamp).unwrap();
    loop {
        thread::sleep(Duration::from_secs(60));
        let wlock = mtx.write().expect("RwLock is poisned");
        update_stats(wlock, &mut sys);
        println!("{}", my_beat.send()?);
    }
}

fn update_stats(
    mut stats: std::sync::RwLockWriteGuard<Stats>,
    sys: &mut systemstat::platform::linux::PlatformImpl,
) {
    let mut networks = HashMap::new();
    match sys.networks() {
        Ok(netifs) => {
            for netif in netifs.values() {
                let net_stats = format!("{:?}", netif.addrs);
                networks.insert(netif.name.clone(), net_stats);
            }
        }
        Err(e) => {
            networks.insert(String::from("Error"), format!("{:?}", e));
        }
    }

    let battery = match sys.battery_life() {
        Ok(battery) => format!("{}%", battery.remaining_capacity * 100.0,),
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

    let mut memory = HashMap::new();
    match sys.memory() {
        Ok(stats) => {
            memory.insert(
                String::from("used"),
                saturating_sub_bytes(stats.total, stats.free).to_string(),
            );
            memory.insert(String::from("total"), stats.total.to_string());
        }
        Err(e) => {
            memory.insert(String::from("used"), format!("{}", e));
            memory.insert(String::from("total"), format!("{}", e));
        }
    }

    stats.memory = memory;
    stats.cpu_temp = cpu_temp;
    stats.uptime = uptime;
    stats.battery = battery;
    stats.networks = networks;
}
