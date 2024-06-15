use tracing::{info, Level};
use tracing_subscriber::fmt::writer::MakeWriterExt;

fn main() {
    // Create rolling file appenders for different log levels
    let info =
        tracing_appender::rolling::daily("/tmp/logs", "info.log").with_max_level(Level::INFO);
    let error =
        tracing_appender::rolling::daily("/tmp/logs", "error.log").with_max_level(Level::ERROR);
    // Create a log file writer that directs `INFO` logs to `info.log` and `ERROR` logs to `error.log`
    let stdout = std::io::stdout.with_max_level(tracing::Level::INFO);

    tracing_subscriber::fmt()
        .with_writer(stdout.and(info).and(error))
        .init();

    // Example log messages
    info!("This is an info message");
    tracing::error!("This is an error message");
}
