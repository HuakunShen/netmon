use netspeed_rs::platform::{*};
use std::{fs::File, io::{self, BufRead, BufReader, Error, Read}, thread::sleep, time::{Duration, Instant}};
use netspeed_rs::print_net_speed;

fn main() {
    // print_net_speed("en6", None);
    // println!("heoi");
    // let file = File::open("/proc/net/dev").unwrap();
    // let mut reader = BufReader::new(file);
    // let mut buf = String::new();
    // let _ = reader.read_to_string(&mut buf);
    // println!("{}", buf);
    print_net_speed("enp7s0", None);

}