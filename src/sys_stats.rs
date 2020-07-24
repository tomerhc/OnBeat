use systemstat::{saturating_sub_bytes, Platform, System};

    let sys = System::new();

    match sys.networks() {
        Ok(netifs) => {
            println!("\nNetwork interface statistics:");
            for netif in netifs.values() {
                println!(
                    "{} statistics: ({:?})",
                    netif.name,
                    sys.network_stats(&netif.name)
                );
            }
        }
        Err(x) => println!("\nNetworks: error: {}", x),
    }

    match sys.battery_life() {
        Ok(battery) => print!(
            "\nBattery: {}%, {}h{}m remaining",
            battery.remaining_capacity * 100.0,
            battery.remaining_time.as_secs() / 3600,
            battery.remaining_time.as_secs() % 60
        ),
        Err(x) => print!("\nBattery: error: {}", x),
    }

    match sys.on_ac_power() {
        Ok(power) => println!(", AC power: {}", power),
        Err(x) => println!(", AC power: error: {}", x),
    }

    match sys.memory() {
        Ok(mem) => println!(
            "\nMemory: {} used / {} ({} bytes) total",
            saturating_sub_bytes(mem.total, mem.free),
            mem.total,
            mem.total.as_u64(),
        ),
        Err(x) => println!("\nMemory: error: {}", x),
    }

    match sys.load_average() {
        Ok(loadavg) => println!(
            "\nLoad average: {} {} {}",
            loadavg.one, loadavg.five, loadavg.fifteen
        ),
        Err(x) => println!("\nLoad average: error: {}", x),
    }

    match sys.uptime() {
        Ok(uptime) => println!("\nUptime: {:?}", uptime),
        Err(x) => println!("\nUptime: error: {}", x),
    }

    match sys.cpu_temp() {
        Ok(cpu_temp) => println!("\nCPU temp: {}", cpu_temp),
        Err(x) => println!("\nCPU temp: {}", x),
    }

    match sys.socket_stats() {
        Ok(stats) => println!("\nSystem socket statistics: {:?}", stats),
        Err(x) => println!("\nSystem socket statistics: error: {}", x.to_string()),
    }
