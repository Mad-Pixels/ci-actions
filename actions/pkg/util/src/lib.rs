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

    println!("Initializing logger with level: {:?}", log_level);  // Проверка уровня

    let decorator = slog_term::TermDecorator::new()
        .force_color()  // Добавим принудительное включение цвета
        .build();
    
    let drain = slog_term::CompactFormat::new(decorator)
        .build()
        .fuse();
    
    let drain = slog::LevelFilter::new(drain, log_level).fuse();
    let drain = slog_async::Async::new(drain)
        .build()
        .fuse();

    let logger = Logger::root(drain, o!());

    // Проверим все уровни логирования
    slog::trace!(&logger, "Test trace message");
    slog::debug!(&logger, "Test debug message");
    slog::info!(&logger, "Test info message");
    slog::warn!(&logger, "Test warning message");
    slog::error!(&logger, "Test error message"; "error" => "test error");

    println!("Logger initialization complete");

    logger
}