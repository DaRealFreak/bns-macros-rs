use std::fs::File;

use log::LevelFilter;
use simplelog::*;

use crate::Poharan;

pub(crate) trait Logging {
    fn init_log(&self);
}

impl Logging for Poharan {
    fn init_log(&self) {
        let configuration = self.settings.section(Some("Configuration")).unwrap();
        CombinedLogger::init(
            vec![
                TermLogger::new(LevelFilter::Info, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
                WriteLogger::new(LevelFilter::Info, Config::default(), File::create(configuration.get("LogFile").unwrap()).unwrap()),
            ]
        ).unwrap();
    }
}
