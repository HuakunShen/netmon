use crate::common::NetStatRow;
use std::io::Error;

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
        .collect()  // Collect all matching interfaces into a Vec
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


