#[derive(Debug, thiserror::Error)]
pub enum MessageError {
    #[error("Serialization error")]
    Serialization,

    #[error("Deserialization error")]
    Deserialization,
}
