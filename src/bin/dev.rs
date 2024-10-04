use netmon::get_all_process_netstat;

fn main() {
    let processes = get_all_process_netstat().unwrap();
    println!("{:#?}", processes);
}