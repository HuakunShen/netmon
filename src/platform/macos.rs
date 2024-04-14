use std::time::Duration;
use crate::common::NetStatRow;

/// Sample netstat -ibnd on MacOS
/// Name       Mtu   Network       Address            Ipkts Ierrs     Ibytes    Opkts Oerrs     Obytes  Coll Drop
/// lo0        16384 <Link#1>                         18381     0    2224104    18381     0    2224104     0   0
///
pub fn parse_netstat_output(output: &str) -> Vec<NetStatRow> {
    output.lines()
        .skip(1)  // Skip the header line
        .filter_map(|line| {
            let mut parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() == 12 {
                let name = parts[0].to_string();
                let ipkts = parts[4].parse().unwrap_or(0);
                let opkts = parts[7].parse().unwrap_or(0);
                let ibytes = parts[6].parse().unwrap_or(0);
                let obytes = parts[9].parse().unwrap_or(0);
                let ierrs = parts[5].parse().unwrap_or(0);
                let oerrs = parts[8].parse().unwrap_or(0);
                let drop = parts[11].parse().unwrap_or(0);
                let colls = parts[10].parse().unwrap_or(0);
                let address = parts[3].to_string();
                let mtu = parts[1].parse().unwrap_or(0);
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
                    address,
                    mtu,
                })
            } else {
                None
            }

            // mtu: parts.next().unwrap().parse().unwrap(),
        })
        .collect()  // Collect all matching interfaces into a Vec
}

pub fn get_current_netstat() -> Vec<NetStatRow> {
    let output = std::process::Command::new("netstat")
        .arg("-ibnd")
        .output()
        .expect("failed to execute process");

    let output_str = String::from_utf8(output.stdout).unwrap();
    let mut stats = parse_netstat_output(&output_str);
    // sort stats by name
    stats.sort_by(|a, b| a.name.cmp(&b.name));
    // remove duplicate by name
    stats.dedup_by(|a, b| a.name == b.name);
    stats
}


pub fn get_current_netstat_by_iface(iface: &str) -> Option<NetStatRow> {
    let stats = get_current_netstat();
    stats.into_iter().find(|stat| stat.name == iface)
}

pub fn print_net_speed(iface: &str, duration_secs: Option<u8>) {
    let mut last_stats = NetStatRow::default();
    let duration_secs = duration_secs.unwrap_or(1);
    let duration = Duration::from_secs(duration_secs as u64);
    loop {
        let stats = get_current_netstat_by_iface(iface).unwrap();
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