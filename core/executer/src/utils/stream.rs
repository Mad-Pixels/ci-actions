use std::pin::Pin;
use tokio::io::{BufReader, AsyncBufReadExt};
use tokio::sync::mpsc;
use futures::Stream;
use slog::Logger;
use processor::Collection;
use crate::ExecuterError;

pub struct OutputStreamItem {
    pub stream_type: StreamType,
    pub line: String,
    pub masked_line: Option<String>,
}

#[derive(Debug, Clone, Copy)]
pub enum StreamType {
    Stdout,
    Stderr,
}

pub async fn stream_output(
    stdout: tokio::process::ChildStdout,
    stderr: tokio::process::ChildStderr,
    processor: Option<&Collection>,
    logger: &Logger,
) -> Pin<Box<impl Stream<Item = Result<OutputStreamItem, ExecuterError>> + Send>> {
    let (tx, rx) = mpsc::channel(100);
    let logger = logger.clone();
    
    // Клонируем processor для использования в async блоках
    let processor_stdout = processor.cloned();
    let processor_stderr = processor.cloned();

    // Обработка stdout
    let tx_stdout = tx.clone();
    let logger_stdout = logger.clone();
    tokio::spawn(async move {
        let mut reader = BufReader::new(stdout).lines();
        while let Ok(Some(line)) = reader.next_line().await {
            let masked_line = processor_stdout.as_ref().map(|p| p.process(&line));
            
            if let Some(ref masked) = masked_line {
                slog::debug!(logger_stdout, "stdout: {}", masked);
            }

            if let Err(e) = tx_stdout.send(Ok(OutputStreamItem {
                stream_type: StreamType::Stdout,
                line: line.clone(),
                masked_line,
            })).await {
                slog::error!(logger_stdout, "Failed to send stdout: {}", e);
                break;
            }
        }
    });

    // Обработка stderr
    let logger_stderr = logger.clone();
    tokio::spawn(async move {
        let mut reader = BufReader::new(stderr).lines();
        while let Ok(Some(line)) = reader.next_line().await {
            let masked_line = processor_stderr.as_ref().map(|p| p.process(&line));
            
            if let Some(ref masked) = masked_line {
                slog::debug!(logger_stderr, "stderr: {}", masked);
            }

            if let Err(e) = tx.send(Ok(OutputStreamItem {
                stream_type: StreamType::Stderr,
                line: line.clone(),
                masked_line,
            })).await {
                slog::error!(logger_stderr, "Failed to send stderr: {}", e);
                break;
            }
        }
    });

    Box::pin(tokio_stream::wrappers::ReceiverStream::new(rx))
}