use quinn::{RecvStream, SendStream};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RequestRecvError {
    #[error("bincode error: {0}")]
    Bincode(bincode::Error),
    #[error("quinn error: {0}")]
    Quinn(quinn::ReadToEndError),
}

impl From<bincode::Error> for RequestRecvError {
    fn from(value: bincode::Error) -> Self {
        Self::Bincode(value)
    }
}

impl From<quinn::ReadToEndError> for RequestRecvError {
    fn from(value: quinn::ReadToEndError) -> Self {
        Self::Quinn(value)
    }
}

#[derive(Debug, Error)]
pub enum RequestSendError {
    #[error("bincode error: {0}")]
    Bincode(bincode::Error),
    #[error("quinn error: {0}")]
    Quinn(quinn::WriteError),
}

impl From<bincode::Error> for RequestSendError {
    fn from(value: bincode::Error) -> Self {
        Self::Bincode(value)
    }
}

impl From<quinn::WriteError> for RequestSendError {
    fn from(value: quinn::WriteError) -> Self {
        Self::Quinn(value)
    }
}

pub async fn write_to<T: Serialize>(
    value: &T,
    mut send: SendStream,
) -> Result<(), RequestSendError> {
    send.write_all(&bincode::serialize(value)?).await?;

    send.finish().await?;
    Ok(())
}
pub async fn read_from<T: for<'de> Deserialize<'de>>(
    size_limit: usize,
    recv: RecvStream,
) -> Result<T, RequestRecvError> {
    let data = recv.read_to_end(size_limit).await?;
    Ok(bincode::deserialize(&data)?)
}
