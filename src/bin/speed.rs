extern crate procfs;

use procfs::process::Process;
use std::{thread, time::Duration, collections::HashMap};

fn main() {
    let refresh_interval = Duration::from_secs(2);
    let mut previous_stats: HashMap<i32, NetIO> = HashMap::new();

    loop {
        // Refresh network stats for all processes
        println!("{:<10} {:<10} {:<15} {:<15}", "PID", "Name", "Upload (Mbps)", "Download (Mbps)");

        for proc in procfs::process::all_processes().unwrap() {
            if let Ok(process) = proc {
                let pid = process.pid;
                let name = process.stat().unwrap().comm;

                // Try to read network information from /proc/[pid]/net/dev
                if let Ok(current_net_io) = read_network_io(pid) {
                    if let Some(previous_net_io) = previous_stats.get(&pid) {
                        // Calculate the rate (bytes/sec) and convert to Mbps
                        let upload_rate = calculate_mbps(current_net_io.bytes_sent, previous_net_io.bytes_sent, refresh_interval);
                        let download_rate = calculate_mbps(current_net_io.bytes_received, previous_net_io.bytes_received, refresh_interval);
                        if name != "speedtest" {
                            continue;
                        }
                        println!(
                            "{:<10} {:<10} {:<15.3} {:<15.3}",
                            pid, name, upload_rate, download_rate
                        );
                    }

                    // Update previous stats with the current stats
                    previous_stats.insert(pid, current_net_io);
                }
            }
        }

        thread::sleep(refresh_interval);
        println!("\nRefreshing...\n");
    }
}

// Structure to hold network statistics
struct NetIO {
    bytes_sent: u64,
    bytes_received: u64,
}

// Function to read /proc/[pid]/net/dev and return network I/O statistics
fn read_network_io(pid: i32) -> Result<NetIO, std::io::Error> {
    let path = format!("/proc/{}/net/dev", pid);
    let contents = std::fs::read_to_string(path)?;

    let mut bytes_sent = 0;
    let mut bytes_received = 0;

    // Parse the /proc/[pid]/net/dev file line by line
    for line in contents.lines().skip(2) { // Skip headers
        let fields: Vec<&str> = line.split_whitespace().collect();
        if fields.len() >= 10 {
            bytes_received += fields[1].parse::<u64>().unwrap_or(0);
            bytes_sent += fields[9].parse::<u64>().unwrap_or(0);
        }
    }

    Ok(NetIO { bytes_sent, bytes_received })
}

// Function to calculate Mbps from bytes transferred in the interval
fn calculate_mbps(current_bytes: u64, previous_bytes: u64, interval: Duration) -> f64 {
    let bytes_per_sec = (current_bytes as f64 - previous_bytes as f64) / interval.as_secs_f64();
    let bits_per_sec = bytes_per_sec * 8.0; // Convert to bits
    bits_per_sec / 1_000_000.0 // Convert to megabits
}
