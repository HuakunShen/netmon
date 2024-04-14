use crate::common::NetStatRow;
use std::{collections::HashMap, io::Error};

#[derive(Debug)]
struct WinSimpleNetstatRow {
    name: String,
    received: u64,
    sent: u64,
}

/// Sample Output
///
/// Interface Statistics
// Received            Sent

// Bytes                     210561855        33081616
// Unicast packets              211295          107570
// Non-unicast packets           11108            8993
// Discards                          0               0
// Errors                            0               0
// Unknown protocols                 0
pub fn parse_netstat_output(output: &str) -> Vec<NetStatRow> {
    let a: Vec<WinSimpleNetstatRow> = output
        .lines()
        .skip(4)
        .filter_map(|line| {
            // get first 20 characters and strip whitespace
            let name = line.chars().take(20).collect::<String>().trim().to_string();
            let rest = line.chars().skip(20).collect::<String>();
            let parts: Vec<&str> = rest.split_whitespace().collect();
            // let mut parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() == 2 {
                Some(WinSimpleNetstatRow {
                    name,
                    received: parts[0].parse().unwrap_or(0),
                    sent: parts[1].parse().unwrap_or(0),
                })
            } else {
                None
            }
        })
        .collect();
        // let b: HashMap<String, u> = HashMap::new();
    // println!("{:?}", a);
    let mut stat = NetStatRow::default();
    stat.ibytes = a[0].received;
    stat.obytes = a[0].sent;
    stat.ipkts = a[1].received + a[2].received;
    stat.opkts = a[1].sent + a[2].sent;
    stat.ierrs = a[4].received;
    stat.oerrs = a[4].sent;
    vec![stat]
}

pub fn get_current_netstat() -> Result<Vec<NetStatRow>, Error> {
    let output = std::process::Command::new("netstat")
        .arg("-e")
        .output()
        .expect("failed to execute process");
    let output_str = String::from_utf8(output.stdout).unwrap();
    let stats = parse_netstat_output(&output_str);
    Ok(stats)
}
