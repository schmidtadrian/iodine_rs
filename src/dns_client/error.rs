use thiserror::Error;


#[derive(Debug, Error)]
pub enum DnsError {
    #[error("{0}")]
    Socket(String),
    #[error("Couldn't send query, because there is no server connection")]
    Disconnected,
    #[error("Couldn't receive data")]
    Receive,
    #[error("Received malformed response")]
    Malformed,
    #[error("Timeout! No response received")]
    Timeout
}
