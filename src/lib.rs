use common::NetStatRow;
use std::time::Duration;

use crate::platform::get_current_netstat_by_iface;
pub mod platform;
pub mod common;


pub fn print_net_speed(iface: &str, duration_secs: Option<u8>) {
    let mut last_stats = NetStatRow::default();
    let duration_secs = duration_secs.unwrap_or(1);
    let duration = Duration::from_secs(duration_secs as u64);
    loop {
        let stats = get_current_netstat_by_iface(iface).unwrap().unwrap();
        if last_stats.name == "" {
            last_stats = stats.clone();
            continue;
        }
        let ibytes_diff = stats.ibytes - last_stats.ibytes;
        let obytes_diff = stats.obytes - last_stats.obytes;
        let in_mbps = (ibytes_diff as f32) / 1024.0 / 1024.0 / duration.as_secs_f32() / (duration_secs as f32);
        let out_mbps = (obytes_diff as f32) / 1024.0 / 1024.0 / duration.as_secs_f32() / (duration_secs as f32);
        // println keep 2 decimal places
        println!("Name: {}, {:.2} MBps in, {:.2} MBps out", stats.name, in_mbps, out_mbps);
        std::thread::sleep(duration); // Adjust the interval as needed
        last_stats = stats.clone();
    }
}