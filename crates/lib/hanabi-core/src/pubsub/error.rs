#[derive(Debug, thiserror::Error)]
pub enum PubSubError {
    #[error("Publish failed on channel: {0}")]
    Publish(ulid::Ulid),
}
