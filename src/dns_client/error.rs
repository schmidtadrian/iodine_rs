use thiserror::Error;


#[derive(Debug, Error)]
pub enum DnsError {
    #[error("{0}")]
    Socket(String),
    #[error("Couldn't send query, because there is no server connection")]
    Disconnected,
    #[error("Received malformed response")]
    Malformed,
    #[error("Timeout! No response received")]
    Timeout,
    #[error("Received packet is no response")]
    NoResponse,
    #[error("Server sent error response")]
    ErrorResponse,
    #[error("Response has no query answer")]
    NoAnswer
}
