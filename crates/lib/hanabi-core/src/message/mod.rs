mod error;

use crate::message::error::MessageError;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use xxhash_rust::xxh3::xxh3_64;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub header: MessageHeader,
    pub content: bytes::Bytes,
}

impl Message {
    pub fn try_from_serializable<T: Serialize>(
        header: MessageHeader,
        content: &T,
    ) -> Result<Self, MessageError> {
        let mut data = vec![];
        ciborium::ser::into_writer(content, &mut data)
            .inspect_err(|error| tracing::error!(%error, "Failed to serialize message content"))
            .map_err(|_| MessageError::Serialization)
            .map(|_| Self {
                header,
                content: bytes::Bytes::from(data),
            })
    }

    pub fn body<T: DeserializeOwned>(&self) -> Result<T, MessageError> {
        ciborium::de::from_reader(self.content.as_ref())
            .inspect_err(|error| tracing::error!(%error, "Failed to deserialize message content"))
            .map_err(|_| MessageError::Deserialization)
    }

    pub fn is_consistent(&self) -> bool {
        let new_hash = xxh3_64(&self.content);
        self.header.content_hash.eq(&new_hash)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageHeader {
    pub id: ulid::Ulid,
    pub publisher: ulid::Ulid,
    pub address: MessageAddress,
    pub content_hash: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageAddress {
    PubSub(String),
}
