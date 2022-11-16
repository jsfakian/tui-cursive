use std::collections::HashMap;

const LABELS: [&str; 16] = ["FS", "RAID", "Adapter", "Subnet", "Gateway", "DNS", "Eve_install_server", "Eve_install_disk", "Eve_persist_disk", "Eve_soft_serial", "Eve_reboot_after_install", "Eve_pause_after_install", "Eve_pause_before_install", "Root", "Find_boot", "Console"];

const VALUES: [&str; 16] = ["0", "0", "0", "", "", "", "", "", "", "", "", "","", "", "", ""];

#[derive(Debug, Clone)]
pub struct Field {
    pub label: String,
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct Data {
    pub map: HashMap<String, String>
}

impl Data {
    pub fn new() -> Self {
        let mut data = Self { 
            map: HashMap::new(),
        };
        for (i, label) in LABELS.iter().enumerate() {
            data.map.insert(label.to_string(), VALUES[i].to_string());
        }
        data
    }
}
