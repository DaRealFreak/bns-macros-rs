use std::fs::File;

use log::LevelFilter;
use simplelog::*;

use crate::AerodromeExp;

pub(crate) trait Logging {
    fn init_log(&self);
}

impl Logging for AerodromeExp {
    fn init_log(&self) {
        let configuration = self.settings.section(Some("Configuration")).unwrap();
        let log_config = ConfigBuilder::new()
            .set_time_to_local(true)
            .build();

        CombinedLogger::init(
            vec![
                TermLogger::new(LevelFilter::Info, log_config.clone(), TerminalMode::Mixed, ColorChoice::Auto),
                WriteLogger::new(LevelFilter::Info, log_config, File::create(configuration.get("LogFile").unwrap()).unwrap()),
            ]
        ).unwrap();
    }
}
