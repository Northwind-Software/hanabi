mod error;

use crate::message::Message;
use crate::pubsub::error::PubSubError;

#[derive(Clone)]
pub struct Channel {
    id: ulid::Ulid,
    address: String,
    receiver: flume::Receiver<Message>,
    sender: flume::Sender<Message>,
}

impl Channel {
    pub fn new(address: String) -> Self {
        let (sender, receiver) = flume::unbounded();
        Self {
            id: ulid::Ulid::new(),
            address,
            receiver,
            sender,
        }
    }

    #[tracing::instrument(skip(self), fields(channel = %self.address))]
    pub fn publish(&self, message: Message) -> Result<(), PubSubError> {
        self.sender
            .send(message)
            .inspect_err(|error| tracing::error!(%error, "Failed to publish message"))
            .map_err(|_| PubSubError::Publish(self.id))
    }

    pub fn subscribe(&self) -> flume::Receiver<Message> {
        self.receiver.clone()
    }
}
