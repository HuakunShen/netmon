use crate::platform::get_current_netstat;
use common::NetStatRow;
use std::io::Error;
use std::time::Duration;

pub mod common;
pub mod platform;

pub fn get_current_netstat_by_iface(iface: &str) -> Result<Option<NetStatRow>, Error> {
    let stats = get_current_netstat()?;
    let found = stats.into_iter().find(|stat| &stat.name == iface);
    Ok(found)
}

pub fn print_net_speed_by_iface(iface: &str, duration_secs: Option<u8>) {
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
        let in_mbps = (ibytes_diff as f32)
            / 1024.0
            / 1024.0
            / duration.as_secs_f32()
            / (duration_secs as f32);
        let out_mbps = (obytes_diff as f32)
            / 1024.0
            / 1024.0
            / duration.as_secs_f32()
            / (duration_secs as f32);
        println!(
            "Name: {}, {:.2} MBps in, {:.2} MBps out",
            stats.name, in_mbps, out_mbps
        );
        std::thread::sleep(duration); // Adjust the interval as needed
        last_stats = stats.clone();
    }
}

pub fn print_total_net_speed() {
    let mut last_ibytes = 0;
    let mut last_obytes = 0;
    let duration = Duration::from_secs(1);
    loop {
        let stats = get_current_netstat().unwrap();
        let ibytes: u64 = stats.iter().map(|stat| stat.ibytes).sum();
        let obytes: u64 = stats.iter().map(|stat| stat.obytes).sum();
        if last_ibytes == 0 {
            last_ibytes = ibytes;
            last_obytes = obytes;
            continue;
        }
        let ibytes_diff = ibytes - last_ibytes;
        let obytes_diff = obytes - last_obytes;
        let in_mbps = (ibytes_diff as f32) / 1024.0 / 1024.0 / duration.as_secs_f32();
        let out_mbps = (obytes_diff as f32) / 1024.0 / 1024.0 / duration.as_secs_f32();
        println!(
            "Total Network Throughput: {:.2} MBps in, {:.2} MBps out",
            in_mbps, out_mbps
        );
        std::thread::sleep(duration); // Adjust the interval as needed
        last_ibytes = ibytes;
        last_obytes = obytes;
    }
}
