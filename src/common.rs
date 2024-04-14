#[derive(Debug, Default, Clone)]
pub struct NetStatRow {
    pub name: String,
    pub ipkts: u64,
    pub opkts: u64,
    pub ibytes: u64,
    pub obytes: u64,
    pub ierrs: u64,
    pub oerrs: u64,
    pub drop: u64,
    // collisions
    pub colls: u64,
    pub address: String,
    // maximum transmission unit
    pub mtu: u64,
}