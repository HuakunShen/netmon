use crate::common::NetStatRow;
use std::collections::HashMap;
use std::io::{BufRead, Error, Read};

/// Sample /proc/net/dev on Linux
/// Inter-|   Receive                                                |  Transmit
/// face |bytes    packets errs drop fifo frame compressed multicast|bytes    packets errs drop fifo colls carrier compressed
/// lo: 1645945   21802    0    0    0     0          0         0  1645945   21802    0    0    0     0       0          0
/// enp7s0: 3068427008 2279482    0    0    0     0          0      8850 2288919797 1847864    0    0    0     0       0          0
pub fn parse_netstat_output(output: &str) -> Vec<NetStatRow> {
    output
        .lines()
        .skip(2) // Skip the header line
        .filter_map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() == 17 {
                let name = parts[0].to_string();
                // if "name" ends with a colon, remove it
                let name = if name.ends_with(":") {
                    name[..name.len() - 1].to_string()
                } else {
                    name
                };
                let ipkts = parts[2].parse().unwrap_or(0);
                let ierrs = parts[3].parse().unwrap_or(0);
                let ibytes = parts[1].parse().unwrap_or(0);
                let opkts = parts[10].parse().unwrap_or(0);
                let oerrs = parts[11].parse().unwrap_or(0);
                let obytes = parts[9].parse().unwrap_or(0);
                let colls = parts[14].parse().unwrap_or(0);
                let drop = parts[4].parse().unwrap_or(0);
                let mtu = parts[2].parse().unwrap_or(0);
                Some(NetStatRow {
                    name,
                    ipkts,
                    opkts,
                    ibytes,
                    obytes,
                    ierrs,
                    oerrs,
                    drop,
                    colls,
                    // address,
                    mtu,
                })
            } else {
                None
            }
        })
        .collect() // Collect all matching interfaces into a Vec
}

/// Get Iface and MTU mapping
/// Sample output table
/// Iface      MTU    RX-OK RX-ERR RX-DRP RX-OVR    TX-OK TX-ERR TX-DRP TX-OVR Flg
// br-4f4c3  1500      233      0      0 0           898      0      0      0 BMRU
fn parse_netstat_table(content: &str) -> HashMap<String, u64> {
    let mut map = HashMap::new();
    for line in content.lines().skip(1) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() != 11 {
            continue;
        }
        let name = parts[0].to_string();
        let mtu = parts[1].parse().unwrap_or(0);
        map.insert(name, mtu);
    }
    map
}

pub fn get_current_netstat() -> Result<Vec<NetStatRow>, Error> {
    let output = std::process::Command::new("netstat")
        .arg("-i")
        .output()
        .expect("failed to execute process");
    let output_str = String::from_utf8(output.stdout).unwrap();
    let iface_mtu_map = parse_netstat_table(&output_str);

    let content = std::fs::read_to_string("/proc/net/dev")?;
    let mut stats = parse_netstat_output(&content);
    for stat in stats.iter_mut() {
        if let Some(mtu) = iface_mtu_map.get(&stat.name) {
            stat.mtu = *mtu;
        }
    }
    Ok(stats)
}
