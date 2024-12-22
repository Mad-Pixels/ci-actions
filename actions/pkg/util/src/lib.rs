use slog::{o, Drain, Level, Logger};
use slog_async;
use slog_term;

pub fn init_logger(level: &str) -> Logger {
    let log_level = match level.to_lowercase().as_str() {
        "trace" => Level::Trace,
        "debug" => Level::Debug,
        "info" => Level::Info,
        "warn" => Level::Warning,
        "error" => Level::Error,
        "critical" => Level::Critical,
        _ => {
            eprintln!("Invalid log level '{}', defaulting to 'Info'", level);
            Level::Info
        }
    };

    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::CompactFormat::new(decorator).build().fuse();
    let drain = slog::LevelFilter::new(drain, log_level).fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    Logger::root(drain, o!())
}
