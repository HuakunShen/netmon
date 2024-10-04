#[cfg(test)]
mod tests {
    use crate::get_all_process_netstat;

    #[test]
    fn test_get_all_process_netstat() {
        let processes = get_all_process_netstat().unwrap();
        assert!(processes.len() > 0);
        println!("{:#?}", processes);
    }
}
