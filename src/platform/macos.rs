use crate::common::{NetStatRow, ProcessBytes};
use std::io::Error;
use std::process::Command;

/// Sample netstat -ibnd on MacOS
/// Name       Mtu   Network       Address            Ipkts Ierrs     Ibytes    Opkts Oerrs     Obytes  Coll Drop
/// lo0        16384 <Link#1>                         18381     0    2224104    18381     0    2224104     0   0
///
pub fn parse_netstat_output(output: &str) -> Vec<NetStatRow> {
    output
        .lines()
        .skip(1) // Skip the header line
        .filter_map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
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
                // let address = parts[3].to_string();
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
                    // address,
                    mtu,
                })
            } else {
                None
            }

            // mtu: parts.next().unwrap().parse().unwrap(),
        })
        .collect() // Collect all matching interfaces into a Vec
}

pub fn get_current_netstat() -> Result<Vec<NetStatRow>, Error> {
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
    Ok(stats)
}

pub fn get_all_process_netstat() -> Result<Vec<ProcessBytes>, Box<dyn std::error::Error>> {
    let output = Command::new("/usr/bin/nettop")
        .args(&["-P", "-L", "1", "-x", "-J", "bytes_in,bytes_out"])
        .output()?;

    // Convert output to string
    let output_str = std::str::from_utf8(&output.stdout)?;

    // Create a HashMap to store the process name and the (bytes_in, bytes_out)
    let mut process_network_data: Vec<ProcessBytes> = vec![];

    // Iterate over the output, split by lines
    for line in output_str.lines() {
        if line.trim().is_empty() {
            continue;
        }

        // Example format of a line: "ProcessName,bytes_in,bytes_out"
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() >= 3 {
            let process_name = parts[0].to_string();
            let bytes_in: u64 = parts[1].parse().unwrap_or(0);
            let bytes_out: u64 = parts[2].parse().unwrap_or(0);

            // Store in the HashMap
            process_network_data.push(ProcessBytes {
                pid: 0,
                process_name: Some(process_name),
                bytes_sent: bytes_in,
                bytes_received: bytes_out,
            });
        }
    }
    Ok(process_network_data)
}
