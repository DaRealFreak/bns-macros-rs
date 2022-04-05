use log4rs::append::file::FileAppender;
use log4rs::Config;
use log4rs::config::{Appender, Root};
use log4rs::encode::pattern::PatternEncoder;
use log::LevelFilter;

use crate::Poharan;

pub(crate) trait Logging {
    fn init_log(&self);
}

impl Logging for Poharan {
    fn init_log(&self) {
        let configuration = self.settings.section(Some("Configuration")).unwrap();
        let logfile = FileAppender::builder()
            .encoder(Box::new(PatternEncoder::new("{d} {l} - {m}{n}")))
            .build(configuration.get("LogFile").unwrap()).unwrap();

        let config = Config::builder()
            .appender(Appender::builder().build("logfile", Box::new(logfile)))
            .build(Root::builder()
                .appender("logfile")
                .build(LevelFilter::Info)).unwrap();

        log4rs::init_config(config).unwrap();
    }
}
