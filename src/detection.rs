use std::{collections::HashMap, path::PathBuf};

use sysinfo::System;

fn env_vars(process: &sysinfo::Process) -> HashMap<String, String> {
    let mut env_vars = HashMap::new();
    for entry in process.environ() {
        if let Ok(binding) = entry.clone().into_string() {
            let mut split = binding.splitn(2, '=');
            if let (Some(key), Some(val)) = (split.next(), split.next()) {
                env_vars.insert(key.to_string(), val.to_string());
            }
        }
    }
    env_vars
}

pub const AMARU_LEDGER_DB_ENV: &str = "AMARU_LEDGER_DB";
pub const AMARU_CHAIN_DB_ENV: &str = "AMARU_CHAIN_DB";

pub fn detect_amaru_process() -> Option<(Option<PathBuf>, HashMap<String, String>)> {
    let mut system = System::new_all();
    system.refresh_all();

    system
        .processes()
        .iter()
        .find(|(_, process)| {
            // Filter out processes that are not running or have no executable path
            process.status() == sysinfo::ProcessStatus::Run && process.name().eq("amaru")
        })
        .map(|(_pid, process)| (process.cwd().map(|p| p.to_path_buf()), env_vars(process)))
}
