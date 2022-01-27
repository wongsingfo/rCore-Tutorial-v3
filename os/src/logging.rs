use log::{self, Level, LevelFilter, Metadata, Record, SetLoggerError};

static LOGGER: SimpleLogger = SimpleLogger;

pub fn init() -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER).map(|()| {
        log::set_max_level(match option_env!("LOG_LEVEL") {
            Some(level) => match level.parse::<LevelFilter>() {
                Ok(level) => level,
                Err(_) => LevelFilter::Info,
            },
            None => LevelFilter::Info,
        });
    })
}

struct SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            print!("\x1b[{}m", level_to_color_code(record.level()));
            println!("[{}] {}", record.level(), record.args());
            print!("\x1b[0m");
        }
    }

    fn flush(&self) {}
}

fn level_to_color_code(level: Level) -> u8 {
    match level {
        Level::Error => 31, // Red
        Level::Warn => 93,  // BrightYellow
        Level::Info => 34,  // Blue
        Level::Debug => 32, // Green
        Level::Trace => 90, // BrightBlack
    }
}