use std::path::Path;
use std::thread::sleep;
use std::time;

use chrono::Local;

mod configuration;

pub(crate) struct Poharan {
    run_count: u16,
    successful_runs: Vec<u16>,
    failed_runs: Vec<u16>,
    run_start_timestamp: Option<std::time::Instant>,
}

pub(crate) trait ChaosSupplyChain {
    fn new() -> Poharan;
    fn start(&mut self) -> bool;
}

trait ChaosSupplyChainBot {
    fn enter_lobby(&mut self) -> bool;
}

impl ChaosSupplyChain for Poharan {
    fn new() -> Poharan {
        if !(Path::new("configuration/poharan.ini").is_file()) {
            configuration::create_ini();
        }

        Poharan {
            run_count: 0,
            successful_runs: vec![],
            failed_runs: vec![],
            run_start_timestamp: None,
        }
    }

    fn start(&mut self) -> bool {
        loop {
            self.enter_lobby();
        }
    }
}

impl ChaosSupplyChainBot for Poharan {
    fn enter_lobby(&mut self) -> bool {
        println!("[{}] entering lobby", Local::now().to_rfc2822());
        sleep(time::Duration::from_secs(1));
        return true
    }
}